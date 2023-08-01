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
            |env, attrs: Vec<(String, crate::AttributeEnum)>, _, _, regions: Vec<(String, irony::RegionId)>|  {

            let region = regions[0].1;

            // TODO: check arg_names and arg_types have the same length
            
            // TODO: check output_namesa and output_types have the same length


            irony::utils::extract_vec(&attrs, "arg_types") == super::utils::extract_input_types(env, region) &&
            irony::utils::extract_vec(&attrs, "output_types") == super::utils::extract_output_types(env, region)
        }),
        InstanceConstraint(InstanceConstraint ,
            |env: &E, attrs, uses: Vec<(String, Vec<EntityId>)>, defs: Vec<(String, Vec<EntityId>)>, _| {
            let target_id = irony::utils::extract_vec(&attrs, "target_id");
            let Some(AttributeEnum::UIntAttr(target_id)) = target_id else {
                panic!("target_id must be a UIntAttr")
            };
            
            let target = env.get_entity(EntityId(target_id.0 as usize));
            let target_def = target.get_def(env).unwrap();
            let (_, target_region) = env.get_op(target_def).get_regions()[0];

            println!("target_region: {:#?}", env.get_region(target_region));

            println!("output_types: {:#?}", super::utils::extract_output_types(env, target_region));
            println!("instance_defs_types: {:#?}", super::utils::extract_types(env, defs[0].1.to_owned()));

            super::utils::extract_input_types(env, target_region) == super::utils::extract_types(env, uses[0].1.to_owned())
            && 
            super::utils::extract_output_types(env, target_region) == super::utils::extract_types(env, defs[0].1.to_owned())

        }),

        SameTypeAggregate(SameTypeAggregate ,
            |env, attrs, uses, defs, regions|  {
                unimplemented!()
        }),
        ArrayConcatConstraint(ArrayConcatConstraint ,
            |env, attrs, uses, defs, regions|  {
                unimplemented!()
        }),
        ArrayCreateConstraint(ArrayCreateConstraint ,
            |env, attrs, uses, defs, regions|  {
                unimplemented!()
        }),
        ArrayGetConstraint(ArrayGetConstraint ,
            |env, attrs, uses, defs, regions|  {
                unimplemented!()
        }),
        ArraySliceConstraint(ArraySliceConstraint ,
            |env, attrs, uses, defs, regions|  {
                unimplemented!()
        }),
        StructCreateConstraint(StructCreateConstraint ,
            |env, attrs, uses, defs, regions|  {
                unimplemented!()
        }),
        StructExtractConstraint(StructExtractConstraint ,
            |env, attrs, uses, defs, regions|  {
                unimplemented!()
        }),
        StructInjectConstraint(StructInjectConstraint ,
            |env, attrs, uses, defs, regions|  {
                unimplemented!()
        }),
        StructExplodeConstraint(StructExplodeConstraint  ,
            |env, attrs, uses, defs, regions| {
                unimplemented!()
        }),
    }
}
