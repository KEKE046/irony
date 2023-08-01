#[allow(unused_variables)]

use irony::{self, preclude::*};

/// define types and attributes
mod common;
mod constraints;
pub use common::*;
pub use constraints::*;

mod utils;

irony::entity_def! {
    [data_type = DataTypeEnum]

    EntityEnum = {
        Wire: (store_data=true),
        Module
    }
}

irony::op_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum, constraint = ConstraintEnum]

    OpEnum = {
        // ------ BEGIN: define the operations in `hw` dialect -------

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
            attrs: [name: StringAttr(StringAttr), arg_names: ArrayAttr(ArrayAttr), arg_types: ArrayAttr(ArrayAttr), output_names: ArrayAttr(ArrayAttr), output_types: ArrayAttr(ArrayAttr)],
            constraints: [ModuleConstraint::default().into()],
            regions: [body],
        },

        Instance: {
            defs: [; outputs],
            uses: [; inputs],
            attrs: [target_id: UIntAttr(UIntAttr), instance_name: StringAttr(StringAttr)],
            constraints: [InstanceConstraint::default().into()],
            regions: [], 
        },

        Input: {
            defs: [; inputs],
            uses: [],
            attrs: [],
            constraints: [],
            regions: [],
        },

        Output: {
            defs: [],
            uses: [; outputs],
            attrs: [],
            constraints:[],
            regions: [],
        },

        BitCast: {
            defs: [lhs],
            uses: [rhs],
            attrs: [],
            constraints: [],
            regions: [],
        },

        Constant: {
            defs: [lhs],
            uses: [],
            attrs: [value: ConstAttr(ConstAttr)],
            constraints: [SameType::new().into()],
            regions: [],
        },

        AggregateConstant: {
            defs: [lhs],
            uses: [],
            attrs: [attrs: ArrayAttr(ArrayAttr)],
            constraints: [SameTypeAggregate::default().into()],
            regions: [],
        },

        ArrayConcat: {
            defs: [lhs],
            uses: [; operands],
            attrs: [],
            constraints: [ArrayConcatConstraint::default().into()],
            regions: [],
        },

        ArrayCreate: {
            defs: [lhs],
            uses: [; operands],
            attrs: [],
            constraints: [ArrayCreateConstraint::default().into(), SameTypeOperands::new().into()],
            regions: [],
        },

        ArrayGet: {
            defs: [lhs],
            uses: [array, index],
            attrs: [],
            constraints: [ArrayGetConstraint::default().into()],
            regions: [],
        },

        ArraySlice: {
            defs: [lhs],
            uses: [array, index],
            attrs: [],
            constraints: [ArraySliceConstraint::default().into()],
            regions: [],
        },

        StructCreate: {
            defs: [lhs],
            uses: [; operands],
            attrs: [],
            constraints: [StructCreateConstraint::default().into()],
            regions: [],
        },

        StructExtract: {
            defs: [lhs],
            uses: [struct_input, field],
            attrs: [],
            constraints: [StructExtractConstraint::default().into()],
            regions: [],
        },

        StructInject: {
            defs: [lhs],
            uses: [struct_input, field, new_value],
            attrs: [],
            constraints: [StructInjectConstraint::default().into()],
            regions: [],
        },

        StructExplode: {
            defs: [; outputs],
            uses: [struct_input],
            attrs: [],
            constraints: [StructExplodeConstraint::default().into()],
            regions: [],
        },

        // ------ END: define the operations in `hw` dialect -------
    }
}

irony::environ_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum, entity = EntityEnum, op = OpEnum, constraint = ConstraintEnum]
    struct CirctEnv;
}

#[cfg(test)]
mod tests;
