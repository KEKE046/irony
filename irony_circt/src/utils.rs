use irony::{Entity, Environ, Op};

use crate::{ArrayAttr, AttributeEnum, DataTypeEnum, TypeAttr};

pub fn extract_types_for_region<E, EntityT, F>(
    env: &E,
    region_id: irony::RegionId,
    op_name: &str,
    f: F,
) -> Option<AttributeEnum>
where
    E: irony::Environ<EntityT = EntityT>,
    EntityT: Entity<DataTypeT = DataTypeEnum>,
    F: Fn(&<E as Environ>::OpT) -> Vec<(String, Vec<irony::EntityId>)>,
{
    let region = env.get_region(region_id);
    let input_types = region
        .op_children
        .iter()
        .find(|&op_id| {
            let op = env.get_op(*op_id);
            op.get_op_name() == op_name
        })
        .and_then(|x| {
            let op = env.get_op(*x);
            let defs = f(op);
            let inputs = &defs[0].1;
            let input_types = inputs
                .into_iter()
                .map(|x| {
                    let input = env.get_entity(*x);
                    TypeAttr(input.get_dtype().unwrap()).into()
                })
                .collect::<Vec<AttributeEnum>>();
            Some(input_types)
        })
        .unwrap();

    Some(ArrayAttr(input_types).into())
}

pub fn extract_input_types<E, EntityT>(env: &E, region_id: irony::RegionId) -> Option<AttributeEnum>
where
    E: irony::Environ<EntityT = EntityT>,
    EntityT: Entity<DataTypeT = DataTypeEnum>,
{
    extract_types_for_region(env, region_id, "Input", |op: &<E as Environ>::OpT| {
        op.get_defs()
    })
}

pub fn extract_output_types<E, EntityT>(
    env: &E,
    region_id: irony::RegionId,
) -> Option<AttributeEnum>
where
    E: irony::Environ<EntityT = EntityT>,
    EntityT: Entity<DataTypeT = DataTypeEnum>,
{
    extract_types_for_region(env, region_id, "Output", |op: &<E as Environ>::OpT| {
        op.get_uses()
    })
}

pub fn extract_types<E, EntityT>(
    env: &E,
    entities: Vec<irony::EntityId>,
) -> Option<AttributeEnum>
where
    E: irony::Environ<EntityT = EntityT>,
    EntityT: Entity<DataTypeT = DataTypeEnum>,
{
    Some(ArrayAttr(entities
        .into_iter()
        .map(|x| {
            let input = env.get_entity(x);
            TypeAttr(input.get_dtype().unwrap()).into()
        })
        .collect::<Vec<AttributeEnum>>()).into())
}
