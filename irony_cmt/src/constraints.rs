use irony::{EntityId, Op};

use super::{AttributeEnum, DataTypeEnum};

pub type SameType = irony::SameTypeConstraint<DataTypeEnum, AttributeEnum>;
pub type SameTypeOperands = irony::SameTypeOperandConstraint<DataTypeEnum, AttributeEnum>;

irony::constraint_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum]
    ConstraintEnum = {
        SameType(SameType),
        SameTypeOperands(SameTypeOperands),
        ModuleConstraint(ModuleConstraint,
            |env, attrs: Vec<(String, crate::AttributeEnum)>, _, _, regions: Vec<(String, Vec<irony::RegionId>)>|  {

            let region = regions[0].1[0];

            // TODO: check arg_names and arg_types have the same length

            // TODO: check output_namesa and output_types have the same length

            irony::utils::extract_vec(&attrs, "arg_names") == super::utils::extract_input_names(env, region) &&
            irony::utils::extract_vec(&attrs, "arg_types") == super::utils::extract_input_types(env, region) &&
            irony::utils::extract_vec(&attrs, "output_types") == super::utils::extract_output_types(env, region)
        }),
        InstanceConstraint(InstanceConstraint ,
            |env: &E, attrs, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
            let target_op_id = irony::utils::extract_vec(&attrs, "target_op_id");
            let Some(AttributeEnum::OpIdAttr(target_op_id)) = target_op_id else {
                panic!("target_id must be a OpIdAttr")
            };

            let target_region= env.get_op(target_op_id.into()).get_regions()[0].1[0];

            super::utils::extract_input_types(env, target_region) == super::utils::extract_types(env, uses[0].1.to_owned())
            &&
            super::utils::extract_output_types(env, target_region) == super::utils::extract_types(env, defs[0].1.to_owned())

        }),

        SameTypeConstant(SameTypeConstant,
            |_, _, _, _, _|  {
                true
        }),
        SameTypeAggregate(SameTypeAggregate,
            |_, _, _, _, _|  {
                true
        }),
        ArrayConcatConstraint(ArrayConcatConstraint ,
            |_, _, _, _, _|  {
                true
        }),
        ArrayCreateConstraint(ArrayCreateConstraint ,
            |_, _, _, _, _|  {
                true
        }),
        ArrayGetConstraint(ArrayGetConstraint ,
            |_, _, _, _, _|  {
                true
        }),
        ArraySliceConstraint(ArraySliceConstraint ,
            |_, _, _, _, _|  {
                true
        }),
        StructCreateConstraint(StructCreateConstraint ,
            |_, _, _, _, _|  {
                true
        }),
        StructExtractConstraint(StructExtractConstraint ,
            |_, _, _, _, _|  {
                true
        }),
        StructInjectConstraint(StructInjectConstraint ,
            |_, _, _, _, _|  {
                true
        }),
        StructExplodeConstraint(StructExplodeConstraint  ,
            |_, _, _, _, _|  {
                true
        }),
    }
}
