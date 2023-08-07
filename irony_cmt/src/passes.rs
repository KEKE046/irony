use irony::{Entity, EntityId, Environ, Id, Op, OpId, PassManagerTrait, PassTrait};

use crate::utils::extract_attrs_for_region;
use crate::{AttributeEnum, EntityEnum, OpEnum, StringAttr};

#[derive(Debug, Default, Clone)]
pub struct RenamePass;

impl PassTrait<(), ()> for RenamePass {
    type EntityT = EntityEnum;
    type OpT = OpEnum;

    fn check_op<E>(&self, env: &E, op_id: irony::OpId) -> bool
    where E: Environ<EntityT = Self::EntityT, OpT = Self::OpT> {
        match env.get_op(op_id) {
            OpEnum::HwModule(_) => true,
            _ => false,
        }
    }

    fn run_raw<E>(&self, env: &mut E, op_id: irony::OpId) -> Result<(), ()>
    where E: Environ<EntityT = Self::EntityT, OpT = Self::OpT> {
        let Some(AttributeEnum::ArrayAttr(arg_names)) = extract_attrs_for_region(
            env,
            env.get_op(op_id).get_regions()[0].1,
            "HwInput",
            |op: &<E as Environ>::OpT| op.get_defs(),
            |env: &E, x: &EntityId| {
                let AttributeEnum::StringAttr(name) = irony::utils::extract_vec(&env.get_entity(*x).get_attrs(), "name").unwrap() else {panic!()};
                let id = env.get_entity(*x).id();
                AttributeEnum::StringAttr(StringAttr(format!("{}_{}", name, id)))
            },
        ) else {panic!()};
        let Some(AttributeEnum::ArrayAttr(_output_names)) = extract_attrs_for_region(
            env,
            env.get_op(op_id).get_regions()[0].1,
            "HwOutput",
            |op: &<E as Environ>::OpT| op.get_uses(),
            |env: &E, x: &EntityId| {
                let AttributeEnum::StringAttr(name) = irony::utils::extract_vec(&env.get_entity(*x).get_attrs(), "name").unwrap() else {panic!()};
                let id = env.get_entity(*x).id();
                AttributeEnum::StringAttr(StringAttr(format!("{}_{}", name, id)))
            },
        ) else { panic!() };
        // Change the name of the module by appending it with "_id"
        env.get_op_entry(op_id).and_modify(|op| {
            if let OpEnum::HwModule(mod_def) = op {
                mod_def.name =
                    Some(StringAttr(format!("{}_{}", mod_def.name.as_ref().unwrap(), mod_def.id)));
                mod_def.arg_names = Some(arg_names);
                // mod_def.output_names = Some(output_names);
            }
        });

        let region_id = env.get_op(op_id).get_regions()[0].1.to_owned();

        // Run RenamePass on potential legal ops (with region, currently None)
        let op_children = env.get_region(region_id).op_children.clone();
        for op_id in op_children.iter() {
            self.run_on(env, *op_id)?;
        }

        for op_id in op_children.iter() {
            env.get_op_entry(*op_id).and_modify(|op| {
                let attrs = op
                    .get_attrs()
                    .iter()
                    .map(|(name, attr)| {
                        if name == "name" {
                            (
                                name.to_owned(),
                                AttributeEnum::StringAttr(StringAttr(
                                    format!("{}_{}", attr, op_id.id()).into(),
                                )),
                            )
                        } else {
                            (name.to_owned(), attr.to_owned())
                        }
                    })
                    .collect::<Vec<_>>();
                op.set_attrs(attrs);
            });
        }

        let entity_children = env.get_region(region_id).entity_children.clone();
        for entity_id in entity_children {
            env.get_entity_entry(entity_id).and_modify(|entity| {
                let attrs = entity
                    .get_attrs()
                    .iter()
                    .map(|(name, attr)| {
                        if name == "name" {
                            (
                                name.to_owned(),
                                AttributeEnum::StringAttr(StringAttr(
                                    format!("{}_{}", attr, entity_id.id()).into(),
                                )),
                            )
                        } else {
                            (name.to_owned(), attr.to_owned())
                        }
                    })
                    .collect::<Vec<_>>();
                entity.set_attrs(attrs);
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum PassEnum {
    RenamePass(RenamePass),
}

impl PassTrait<(), ()> for PassEnum {
    type EntityT = EntityEnum;
    type OpT = OpEnum;

    fn check_op<E>(&self, env: &E, op_id: irony::OpId) -> bool
    where E: Environ<EntityT = Self::EntityT, OpT = Self::OpT> {
        match self {
            PassEnum::RenamePass(pass) => pass.check_op(env, op_id),
        }
    }

    fn run_raw<E>(&self, env: &mut E, op_id: irony::OpId) -> Result<(), ()>
    where E: Environ<EntityT = Self::EntityT, OpT = Self::OpT> {
        match self {
            PassEnum::RenamePass(pass) => pass.run_raw(env, op_id),
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
