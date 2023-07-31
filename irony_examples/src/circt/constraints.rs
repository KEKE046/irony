

use super::{DataTypeEnum, AttributeEnum};

pub type SameType = irony::SameTypeConstraint<DataTypeEnum, AttributeEnum>;
pub type SameTypeOperands = irony::SameTypeOperandConstraint<DataTypeEnum,AttributeEnum>;

irony::constraint_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum]
    ConstraintEnum = {
        SameType(SameType), 
        SameTypeOperands(SameTypeOperands),
        ModuleConstraint(ModuleConstraint {
            todo!()
            // assert!(regions.len() == 1, "module must have one region");
            // let region = regions[0].1[0];
            // irony::utils::TypeMatch(irony::utils::extract_vec(&values, "args_types"), super::utils::extract_input(env, region)) && irony::utils::TypeMatch(irony::utils::extract_vec(&values, "output_types"), super::utils::extract_output(env, region))
        }),
        InstanceConstraint(InstanceConstraint {
            todo!()
        }),
        SameTypeAggregate(SameTypeAggregate {
            todo!()
        }),
        ArrayConcatConstraint(ArrayConcatConstraint {
            todo!()
        }),
        ArrayCreateConstraint(ArrayCreateConstraint {
            todo!()
        }),
        ArrayGetConstraint(ArrayGetConstraint {
            todo!()
        }),
        ArraySliceConstraint(ArraySliceConstraint {
            todo!()
        }),
        StructCreateConstraint(StructCreateConstraint {
            todo!()
        }),
        StructExtractConstraint(StructExtractConstraint {
            todo!()
        }),
        StructInjectConstraint(StructInjectConstraint {
            todo!()
        }),
        StructExplodeConstraint(StructExplodeConstraint {
            todo!()
        }),
    }
}