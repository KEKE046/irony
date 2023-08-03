use irony::{self, preclude::*};


irony::data_type_enum![DataTypeEnum = { UInt(usize), None}];

pub type ConstValue=irony::ConstValueI32<DataTypeEnum>;

#[derive(Debug, Clone, PartialEq)]
pub struct StringAttr(pub String);
impl AttributeTrait for StringAttr {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
    type DataTypeT = DataTypeEnum;
}

impl Into<StringAttr> for &str {
    fn into(self) -> StringAttr {
        StringAttr(self.to_string())
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
            regions: [],
        },
        Assign: {
            defs: [lhs],
            uses: [rhs],
            attrs: [],
            constraints: [SameType::new().into()],
            regions: [],
        },
        ModuleDef: {
            defs: [lhs],
            uses: [],
            attrs: [],
            constraints: [],
            regions: [region],
        },
    }
}


irony::environ_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum, entity = EntityEnum, op = OpEnum, constraint = ConstraintEnum]
    struct CirctEnv;
}

#[cfg(test)]
mod tests;
