use core::panic;
use std::collections::HashSet;

use irony::{Entity, Environ, Op, OpId, PassManagerTrait, PassTrait};

use crate::{AttributeEnum, EntityEnum, OpEnum, StringAttr};

#[derive(Debug, Clone)]
pub struct RenamePass;

impl PassTrait<(), ()> for RenamePass {
  type EntityT = EntityEnum;
  type OpT = OpEnum;

  fn check_op<E>(&self, env: &E, op: OpId) -> bool
  where E: Environ<EntityT = Self::EntityT, OpT = Self::OpT> {
    match env.get_op(op) {
      OpEnum::HwModule(_) => true,
      _ => false,
    }
  }

  fn run_raw<E>(&self, env: &mut E, op: OpId) -> Result<(), ()>
  where E: Environ<EntityT = Self::EntityT, OpT = Self::OpT> {
    let region = env.get_op(op).get_regions()[0].1[0];

    let mut name_set = HashSet::new();

    let included = env.get_region(region).op_children.to_owned();
    for op_id in included {
      match env.get_op(op_id) {
        OpEnum::HwInput(_) => {},
        x => {
          let defs = x
            .get_defs()
            .iter()
            .flat_map(|(_, v)| v.iter().filter_map(|x| x.map(|x| x.to_owned())))
            .collect::<Vec<_>>();
          for def in defs {
            let name = env.get_entity(def).get_attr("name").unwrap();
            let name = match name {
              AttributeEnum::StringAttr(StringAttr(name)) => name,
              _ => {
                panic!()
              },
            };

            let mut splits = name.split('_').collect::<Vec<_>>();
            loop {
              let last = splits.pop();
              match last {
                Some(last) => {
                  if last.to_string().parse::<u32>().is_ok() {
                    let shorter = splits.join("_");
                    if name_set.contains(&shorter) {
                      splits.push(last);
                      break;
                    }
                  } else {
                    splits.push(last);
                    break;
                  }
                },
                _ => {
                  break;
                },
              }
            }

            let name = splits.join("_");
            name_set.insert(name.to_owned());

            env.get_entity_entry(def).and_modify(|entity| {
              entity.set_attrs(vec![(
                "name".to_owned(),
                AttributeEnum::StringAttr(StringAttr(name)),
              )]);
            });
          }
        },
      }
    }

    Ok(())
  }
}

#[derive(Debug, Clone)]
pub struct MoveOutputToTailPass;

impl PassTrait<(), ()> for MoveOutputToTailPass {
  type EntityT = EntityEnum;
  type OpT = OpEnum;

  fn check_op<E>(&self, env: &E, op: OpId) -> bool
  where E: Environ<EntityT = Self::EntityT, OpT = Self::OpT> {
    match env.get_op(op) {
      OpEnum::HwModule(_) => true,
      _ => false,
    }
  }

  fn run_raw<E>(&self, env: &mut E, op: OpId) -> Result<(), ()>
  where E: Environ<EntityT = Self::EntityT, OpT = Self::OpT> {
    let region = env.get_op(op).get_regions()[0].1[0];

    let mut included = env.get_region(region).op_children.to_owned();
    if let Some(index) = included
      .iter()
      .position(|op| matches!(env.get_op(op.to_owned()), OpEnum::HwOutput(_)))
    {
      let x = included.remove(index);
      included.push(x);
    };

    env.get_region_entry(region).and_modify(|region| {
      region.op_children = included;
    });

    Ok(())
  }
}

#[derive(Debug, Clone)]
pub enum PassEnum {
  RenamePass(RenamePass),
  MoveOutputToTailPass(MoveOutputToTailPass),
}

impl PassTrait<(), ()> for PassEnum {
  type EntityT = EntityEnum;
  type OpT = OpEnum;

  fn check_op<E>(&self, env: &E, op_id: irony::OpId) -> bool
  where E: Environ<EntityT = Self::EntityT, OpT = Self::OpT> {
    match self {
      PassEnum::RenamePass(pass) => pass.check_op(env, op_id),
      PassEnum::MoveOutputToTailPass(pass) => pass.check_op(env, op_id),
    }
  }

  fn run_raw<E>(&self, env: &mut E, op_id: irony::OpId) -> Result<(), ()>
  where E: Environ<EntityT = Self::EntityT, OpT = Self::OpT> {
    match self {
      PassEnum::RenamePass(pass) => pass.run_raw(env, op_id),
      PassEnum::MoveOutputToTailPass(pass) => pass.run_raw(env, op_id),
    }
  }
}

#[derive(Default, Debug, Clone)]
pub struct PassManager {
  passes: Vec<PassEnum>,
  start_ops: Vec<Vec<OpId>>,
}

impl PassManagerTrait<(), ()> for PassManager {
  type EntityT = EntityEnum;
  type OpT = OpEnum;
  type PassT = PassEnum;

  fn add_passes(&mut self, mut passes: Vec<Self::PassT>, mut start_ops: Vec<Vec<OpId>>) {
    assert_eq!(passes.len(), start_ops.len());
    self.passes.append(&mut passes);
    self.start_ops.append(&mut start_ops);
  }

  fn run_passes<E>(&self, env: &mut E) -> Result<(), ()>
  where E: Environ<EntityT = Self::EntityT, OpT = Self::OpT> {
    for (pass, op) in self.passes.iter().zip(self.start_ops.iter()) {
      for op in op.iter() {
        pass.run_on(env, *op)?;
      }
    }
    Ok(())
  }
}
