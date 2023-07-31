use irony::{self, preclude::*};


irony::data_type_enum![DataTypeEnum = UInt(usize)];

pub type ConstValue=irony::ConstValueI32<DataTypeEnum>;
irony::attribute_enum! {
    [data_type = DataTypeEnum]
    AttributeEnum = ConstValue(ConstValue)
}

type SameType = irony::SameTypeConstraint<DataTypeEnum, AttributeEnum>;
irony::constraint_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum]
    ConstraintEnum = {
        SameType(SameType),
    }
}

irony::entity_def! {
    [data_type = DataTypeEnum]

    EntityEnum = {
        Wire: (store_data=true),
        Module
    }
}

irony::op_def! {
    [attr = AttributeEnum, constraint = ConstraintEnum]

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
    [entity = EntityEnum, op = OpEnum, attr = AttributeEnum, constraint = ConstraintEnum]
    struct CirctEnv;
}

#[cfg(test)]
mod tests;
