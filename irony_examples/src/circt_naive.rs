use irony::preclude::*;
use irony::{self};

irony::data_type_enum![DataTypeEnum = { UInt(usize)}];

pub type ConstValue = irony::ConstValueU32<DataTypeEnum>;

#[derive(Debug, Clone, PartialEq)]
pub struct StringAttr(pub String);

impl Into<StringAttr> for &str {
    fn into(self) -> StringAttr { StringAttr(self.to_string()) }
}

impl std::fmt::Display for StringAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

irony::attribute_enum! {
    [data_type = DataTypeEnum]
    AttributeEnum = { ConstValue(ConstValue), StringAttr(StringAttr)}
}

type SameType = irony::SameTypeConstraint<DataTypeEnum, AttributeEnum>;
irony::constraint_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum]
    ConstraintEnum = {
        SameType(SameType),
    }
}

irony::entity_def! {
    [data_type = DataTypeEnum, attr=AttributeEnum]

    EntityEnum = {
        Wire: [name: StringAttr(StringAttr)],
        Module: [name: StringAttr(StringAttr)],
    }
}

irony::op_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum, constraint = ConstraintEnum]

    OpEnum = {
        Constant:  {
            defs: [lhs],
            uses: [],
            attrs: [rhs: ConstValue(ConstValue)],
            constraints: [SameType::new().into()],
            print: (
                |_, attrs: Vec<(String, AttributeEnum)>, _, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    format!("{:?} = {:?}", defs[0].1, attrs[0].1)
                }
            ),
        },
        Assign: {
            defs: [lhs],
            uses: [rhs],
            constraints: [SameType::new().into()],
            print: (
                |_, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    format!("{:?} = {:?}", defs[0].1, uses[0].1)
                }
            )
        },
        ModuleDef: {
            defs: [lhs],
            uses: [],
            regions: [region],
            print: (
                |env: &E, _, _, defs: Vec<(String, Vec<Option<EntityId>>)>, regions: Vec<(String, Vec<RegionId>)>| {
                    format!("module {:?} {{\n{}\n}}", defs[0].1, env.print_region(regions[0].1[0]))
                }
            )
        },
    }
}
#[derive(Clone)]
pub struct PassPH;
impl PassTrait<(), ()> for PassPH {
    type EntityT = EntityEnum;

    type OpT = EntityEnum;

    fn check_op<E>(&self, _env:&E, _op: OpId) -> bool
    where E: Environ<EntityT=Self::EntityT, OpT=Self::OpT> {
        todo!()
    }

    fn run_raw<E>(&self, _env: &mut E, _op: OpId) -> Result<(), ()>
    where E: Environ<EntityT=Self::EntityT, OpT=Self::OpT> {
        todo!()
    }
}
#[derive(Debug, Default, Clone)]
pub struct PassManager;

impl PassManagerTrait<(), ()> for PassManager {
    type EntityT=EntityEnum;

    type OpT=OpEnum;

    type PassT = PassPH;

    fn add_passes(&mut self, _passes: Vec<Self::PassT>, _start_ops: Vec<Vec<OpId>>) {
        todo!()
    }

    fn run_passes<E>(&self, _env: &mut E) -> Result<(), ()>
    where E: Environ<EntityT=Self::EntityT, OpT=Self::OpT>  {
        todo!()
    }
}

irony::environ_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum, entity = EntityEnum, op = OpEnum, constraint = ConstraintEnum, pm = PassManager]
    struct CirctEnv;
}

#[cfg(test)]
mod tests;
