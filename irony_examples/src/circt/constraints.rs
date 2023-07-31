use irony::ConstraintTrait;

use super::{DataTypeEnum, AttributeEnum};

/// ## ModuleConstraint
/// 
#[derive(Default, Clone, Debug, PartialEq)]
pub struct ModuleConstraint;
impl ConstraintTrait for ModuleConstraint {
    type DataTypeT=DataTypeEnum;

    type AttributeT=AttributeEnum;

    fn verify<'env, E, EntityT: irony::Entity>(
        &self,
        env: &'env E,
        values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<irony::EntityId>)>,
        defs: Vec<(String, Vec<irony::EntityId>)>,
        regions: Vec<(String, Vec<irony::RegionId>)>,
    ) -> bool
    where
        E: irony::Environ<EntityT = EntityT>,
        EntityT: irony::Entity<DataTypeT = Self::DataTypeT> {
            todo!()
            // assert!(regions.len() == 1, "module must have one region");
            // let region = regions[0].1[0];
            // irony::utils::TypeMatch(irony::utils::extract_vec(&values, "args_types"), super::utils::extract_input(env, region)) && irony::utils::TypeMatch(irony::utils::extract_vec(&values, "output_types"), super::utils::extract_output(env, region))

    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct InstanceConstraint;
impl ConstraintTrait for InstanceConstraint {
    type DataTypeT=DataTypeEnum;

    type AttributeT=AttributeEnum;

    fn verify<'env, E, EntityT: irony::Entity>(
        &self,
        env: &'env E,
        values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<irony::EntityId>)>,
        defs: Vec<(String, Vec<irony::EntityId>)>,
        regions: Vec<(String, Vec<irony::RegionId>)>,
    ) -> bool
    where
        E: irony::Environ<EntityT = EntityT>,
        EntityT: irony::Entity<DataTypeT = Self::DataTypeT> {
            todo!()
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct SameTypeAggregate;
impl ConstraintTrait for SameTypeAggregate {
    type DataTypeT=DataTypeEnum;

    type AttributeT=AttributeEnum;

    fn verify<'env, E, EntityT: irony::Entity>(
        &self,
        env: &'env E,
        values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<irony::EntityId>)>,
        defs: Vec<(String, Vec<irony::EntityId>)>,
        regions: Vec<(String, Vec<irony::RegionId>)>,
    ) -> bool
    where
        E: irony::Environ<EntityT = EntityT>,
        EntityT: irony::Entity<DataTypeT = Self::DataTypeT> {
            todo!()
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ArrayConcatConstraint;
impl ConstraintTrait for ArrayConcatConstraint {
    type DataTypeT = DataTypeEnum;

    type AttributeT = AttributeEnum;

    fn verify<'env, E, EntityT: irony::Entity>(
        &self,
        env: &'env E,
        values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<irony::EntityId>)>,
        defs: Vec<(String, Vec<irony::EntityId>)>,
        regions: Vec<(String, Vec<irony::RegionId>)>,
    ) -> bool
    where
        E: irony::Environ<EntityT = EntityT>,
        EntityT: irony::Entity<DataTypeT = Self::DataTypeT> {
            todo!()
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ArrayCreateConstraint;
impl ConstraintTrait for ArrayCreateConstraint {
    type DataTypeT = DataTypeEnum;

    type AttributeT = AttributeEnum;

    fn verify<'env, E, EntityT: irony::Entity>(
        &self,
        env: &'env E,
        values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<irony::EntityId>)>,
        defs: Vec<(String, Vec<irony::EntityId>)>,
        regions: Vec<(String, Vec<irony::RegionId>)>,
    ) -> bool
    where
        E: irony::Environ<EntityT = EntityT>,
        EntityT: irony::Entity<DataTypeT = Self::DataTypeT> {
            todo!()
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ArrayGetConstraint;
impl ConstraintTrait for ArrayGetConstraint {
    type DataTypeT = DataTypeEnum;

    type AttributeT = AttributeEnum;

    fn verify<'env, E, EntityT: irony::Entity>(
        &self,
        env: &'env E,
        values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<irony::EntityId>)>,
        defs: Vec<(String, Vec<irony::EntityId>)>,
        regions: Vec<(String, Vec<irony::RegionId>)>,
    ) -> bool
    where
        E: irony::Environ<EntityT = EntityT>,
        EntityT: irony::Entity<DataTypeT = Self::DataTypeT> {
            todo!()
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ArraySliceConstraint;
impl ConstraintTrait for ArraySliceConstraint {
    type DataTypeT = DataTypeEnum;

    type AttributeT = AttributeEnum;

    fn verify<'env, E, EntityT: irony::Entity>(
        &self,
        env: &'env E,
        values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<irony::EntityId>)>,
        defs: Vec<(String, Vec<irony::EntityId>)>,
        regions: Vec<(String, Vec<irony::RegionId>)>,
    ) -> bool
    where
        E: irony::Environ<EntityT = EntityT>,
        EntityT: irony::Entity<DataTypeT = Self::DataTypeT> {
            todo!()
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct StructCreateConstraint;
impl ConstraintTrait for StructCreateConstraint {
    type DataTypeT = DataTypeEnum;

    type AttributeT = AttributeEnum;

    fn verify<'env, E, EntityT: irony::Entity>(
        &self,
        env: &'env E,
        values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<irony::EntityId>)>,
        defs: Vec<(String, Vec<irony::EntityId>)>,
        regions: Vec<(String, Vec<irony::RegionId>)>,
    ) -> bool
    where
        E: irony::Environ<EntityT = EntityT>,
        EntityT: irony::Entity<DataTypeT = Self::DataTypeT> {
            todo!()
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct StructExtractConstraint;
impl ConstraintTrait for StructExtractConstraint {
    type DataTypeT = DataTypeEnum;

    type AttributeT = AttributeEnum;

    fn verify<'env, E, EntityT: irony::Entity>(
        &self,
        env: &'env E,
        values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<irony::EntityId>)>,
        defs: Vec<(String, Vec<irony::EntityId>)>,
        regions: Vec<(String, Vec<irony::RegionId>)>,
    ) -> bool
    where
        E: irony::Environ<EntityT = EntityT>,
        EntityT: irony::Entity<DataTypeT = Self::DataTypeT> {
            todo!()
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct StructInjectConstraint;
impl ConstraintTrait for StructInjectConstraint {
    type DataTypeT = DataTypeEnum;

    type AttributeT = AttributeEnum;

    fn verify<'env, E, EntityT: irony::Entity>(
        &self,
        env: &'env E,
        values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<irony::EntityId>)>,
        defs: Vec<(String, Vec<irony::EntityId>)>,
        regions: Vec<(String, Vec<irony::RegionId>)>,
    ) -> bool
    where
        E: irony::Environ<EntityT = EntityT>,
        EntityT: irony::Entity<DataTypeT = Self::DataTypeT> {
            todo!()
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
pub struct StructExplodeConstraint;
impl ConstraintTrait for StructExplodeConstraint {
    type DataTypeT = DataTypeEnum;

    type AttributeT = AttributeEnum;

    fn verify<'env, E, EntityT: irony::Entity>(
        &self,
        env: &'env E,
        values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<irony::EntityId>)>,
        defs: Vec<(String, Vec<irony::EntityId>)>,
        regions: Vec<(String, Vec<irony::RegionId>)>,
    ) -> bool
    where
        E: irony::Environ<EntityT = EntityT>,
        EntityT: irony::Entity<DataTypeT = Self::DataTypeT> {
            todo!()
    }
}

pub type SameType = irony::SameTypeConstraint<DataTypeEnum, AttributeEnum>;
pub type SameTypeOperands = irony::SameTypeOperandConstraint<DataTypeEnum,AttributeEnum>;

irony::constraint_enum! {
    [data_type = DataTypeEnum, attr = AttributeEnum]
    ConstraintEnum = SameType(SameType), SameTypeOperands(SameTypeOperands), ModuleConstraint(ModuleConstraint), InstanceTypeMatch(InstanceConstraint), SameTypeAggregate(SameTypeAggregate),
    ArrayConcatConstraint(ArrayConcatConstraint), ArrayCreateConstraint(ArrayCreateConstraint), ArrayGetConstraint(ArrayGetConstraint), ArraySliceConstraint(ArraySliceConstraint), StructCreateConstraint(StructCreateConstraint), StructExtractConstraint(StructExtractConstraint), StructInjectConstraint(StructInjectConstraint), StructExplodeConstraint(StructExplodeConstraint)
}