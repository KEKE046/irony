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


        HwModule: {
            defs: [lhs],
            uses: [],
            attrs: [name: StringAttr(StringAttr), arg_names: ArrayAttr(ArrayAttr), arg_types: ArrayAttr(ArrayAttr), output_names: ArrayAttr(ArrayAttr), output_types: ArrayAttr(ArrayAttr)],
            constraints: [ModuleConstraint::default().into()],
            regions: [body],
        },

        HwInstance: {
            defs: [; outputs],
            uses: [; inputs],
            attrs: [target_id: UIntAttr(UIntAttr), instance_name: StringAttr(StringAttr)],
            constraints: [InstanceConstraint::default().into()],
            regions: [], 
        },

        HwInput: {
            defs: [; inputs],
            uses: [],
            attrs: [],
            constraints: [],
            regions: [],
        },

        HwOutput: {
            defs: [],
            uses: [; outputs],
            attrs: [],
            constraints:[],
            regions: [],
        },

        HwBitCast: {
            defs: [lhs],
            uses: [rhs],
            attrs: [],
            constraints: [],
            regions: [],
        },

        HwConstant: {
            defs: [lhs],
            uses: [],
            attrs: [value: ConstAttr(ConstAttr)],
            constraints: [SameType::new().into()],
            regions: [],
        },

        HwAggregateConstant: {
            defs: [lhs],
            uses: [],
            attrs: [attrs: ArrayAttr(ArrayAttr)],
            constraints: [SameTypeAggregate::default().into()],
            regions: [],
        },

        HwArrayConcat: {
            defs: [lhs],
            uses: [; operands],
            attrs: [],
            constraints: [ArrayConcatConstraint::default().into()],
            regions: [],
        },

        HwArrayCreate: {
            defs: [lhs],
            uses: [; operands],
            attrs: [],
            constraints: [ArrayCreateConstraint::default().into(), SameTypeOperands::new().into()],
            regions: [],
        },

        HwArrayGet: {
            defs: [lhs],
            uses: [array, index],
            attrs: [],
            constraints: [ArrayGetConstraint::default().into()],
            regions: [],
        },

        HwArraySlice: {
            defs: [lhs],
            uses: [array, index],
            attrs: [],
            constraints: [ArraySliceConstraint::default().into()],
            regions: [],
        },

        HwStructCreate: {
            defs: [lhs],
            uses: [; operands],
            attrs: [],
            constraints: [StructCreateConstraint::default().into()],
            regions: [],
        },

        HwStructExtract: {
            defs: [lhs],
            uses: [struct_input, field],
            attrs: [],
            constraints: [StructExtractConstraint::default().into()],
            regions: [],
        },

        HwStructInject: {
            defs: [lhs],
            uses: [struct_input, field, new_value],
            attrs: [],
            constraints: [StructInjectConstraint::default().into()],
            regions: [],
        },

        HwStructExplode: {
            defs: [; outputs],
            uses: [struct_input],
            attrs: [],
            constraints: [StructExplodeConstraint::default().into()],
            regions: [],
        },

        // ------ END: define the operations in `hw` dialect -------

        // ------ BEGIN: define the operations in `comb` dialect -------
        // TODO: Add more constraints for safer usage
        CombVariadic: {
            defs: [lhs],
            uses: [; operands],
            attrs: [predicate: CombVariadicPredicate(CombVariadicPredicate)],
            constraints: [SameType::new().into()],
            regions: [],
        },
        CombBinary: {
            defs: [lhs],
            uses: [op0, op1],
            attrs: [predicate: CombBinaryPredicate(CombBinaryPredicate)],
            constraints: [SameType::new().into()],
            regions: [],
        },
        CombICmp: {
            defs: [lhs],
            uses: [op0, op1],
            attrs: [predicate: CombICmpPredicate(CombICmpPredicate)],
            constraints: [SameTypeOperands::new().into()],
            regions: [],
        },
        CombParity: {
            defs: [lhs],
            uses: [rhs],
            attrs: [],
            constraints: [/* TODO: fill this */],
            regions:[],
        },
        CombExtract: {
            defs: [lhs],
            uses: [input, low],
            attrs: [],
            constraints: [/* TODO: fill this */],
            regions: [],
        },
        CombConcat: {
            defs: [lhs],
            uses: [; operands],
            attrs: [],
            constraints: [/* TODO: fill this */],
            regions: [],
        },
        CombReplicate: {
            defs: [lhs],
            uses: [rhs],
            attrs: [],
            constraints: [/* TODO: fill this */],
            regions: [],
        },
        CombMux2: {
            defs: [lhs],
            uses: [cond, op0, op1],
            attrs: [],
            constraints: [/* TODO: fill this */],
            regions: [],
        },
        // ------ END: define the operations in `comb` dialect -------
        
        // ------ BEGIN: define the operations in `seq` dialect -------
        SeqCompReg: {
            defs: [; outputs],
            uses: [clk,reset,reset_val; inputs],
            attrs: [/*name: StringAttr(StringAttr)*/],
            constraints: [/* TODO: fill this */],
            regions: [],
        },

        SeqHlmem: {
            defs: [handle],
            uses: [clk, reset],
            attrs: [],
            constraints: [/* TODO: fill this */],
            regions: [],
        },

        SeqRead: {
            defs: [rdata],
            uses: [mem, renable; address],
            attrs: [latency: UIntAttr(UIntAttr)],
            constraints: [],
            regions: [],
        },

        SeqWrite: {
            defs: [],
            uses: [mem, wenable, wdata; address],
            attrs: [latency: UIntAttr(UIntAttr)],
            constraints: [],
            regions: [],
        },
        
    }
}

irony::environ_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum, entity = EntityEnum, op = OpEnum, constraint = ConstraintEnum]
    struct CirctEnv;
}

#[cfg(test)]
mod tests;
