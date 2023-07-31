
use std::marker::PhantomData;

use crate::{EntityId, RegionId};

use super::common::AttributeTrait;
use super::entity::Entity;
use super::environ::Environ;

pub trait ConstraintTrait {
    type DataTypeT;
    type AttributeT;
    fn verify<'env, E, EntityT: Entity>(
        &self,
        env: &'env E,
        values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<EntityId>)>,
        defs: Vec<(String, Vec<EntityId>)>,
        regions: Vec<(String, Vec<RegionId>)>,
    ) -> bool
    where
        E: Environ<EntityT = EntityT>,
        EntityT: Entity<DataTypeT = Self::DataTypeT>;
}


#[derive( PartialEq, Clone, Copy, Debug)]
pub struct SameTypeConstraint<D, A> {
    _marker: PhantomData<(D, A)>
}

impl<D: PartialEq, A: AttributeTrait<D>> ConstraintTrait for SameTypeConstraint<D, A>{
    type DataTypeT = D;
    type AttributeT = A;
    fn verify<'env, E, EntityT: Entity>(
        &self,
        env: &'env E,
        values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<EntityId>)>,
        defs: Vec<(String, Vec<EntityId>)>,
        _regions: Vec<(String, Vec<RegionId>)>,
    ) -> bool
    where
        E: Environ<EntityT = EntityT>,
        EntityT: Entity<DataTypeT = Self::DataTypeT>
    {

        let value_tys = values.into_iter().map(|pair| pair.1
    ).flat_map(|v| v.iter().map(|value| Some(value.dtype())).collect::<Vec<_>>());
        let uses_tys = uses.into_iter().map(|pair| pair.1).flat_map(|v| v.iter().map(|x| env.get_entity(x.to_owned()).get_dtype()).collect::<Vec<_>>());
        let defs_tys = defs.into_iter().map(|pair| pair.1).flat_map(|v| v.iter().map(|x| env.get_entity(x.to_owned()).get_dtype()).collect::<Vec<_>>());

        let mut ty_collect = value_tys.chain(uses_tys).chain(defs_tys);
        if let Some(first) = ty_collect.next() {
            ty_collect.all(|item| item == first)
        } else {
            true
        }
    }
}

impl<D, A> SameTypeConstraint<D, A> {
    pub fn new() -> Self {
        Self { _marker: PhantomData }
    }
}

#[derive( PartialEq, Clone, Copy, Debug)]
pub struct SameTypeOperandConstraint<D, A> {
    _marker: PhantomData<(D, A)>
}

impl<D: PartialEq, A: AttributeTrait<D>> ConstraintTrait for SameTypeOperandConstraint<D, A>{
    type DataTypeT = D;
    type AttributeT = A;
    fn verify<'env, E, EntityT: Entity>(
        &self,
        env: &'env E,
        _values: Vec<(String, Vec<Self::AttributeT>)>,
        uses: Vec<(String, Vec<EntityId>)>,
        _defs: Vec<(String, Vec<EntityId>)>,
        _regions: Vec<(String, Vec<RegionId>)>,
    ) -> bool
    where
        E: Environ<EntityT = EntityT>,
        EntityT: Entity<DataTypeT = Self::DataTypeT>
    {
        let mut uses_tys = uses.into_iter().map(|pair| pair.1).flat_map(|v| v.iter().map(|x| env.get_entity(x.to_owned()).get_dtype()).collect::<Vec<_>>());


        if let Some(first) = uses_tys.next() {
            uses_tys.all(|item| item == first)
        } else {
            true
        }
    }
}

impl<D, A> SameTypeOperandConstraint<D, A> {
    pub fn new() -> Self {
        Self { _marker: PhantomData }
    }
}


#[macro_export]
macro_rules! constraint_def {
    (
        [data_type = $dtype:ty, attr = $attr:ty] 
        $name:ident = {
            $($variant:ident($variant_ty:ident$($block:block)?),)*
        }
    ) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $name {
            $($variant($variant_ty)),*
        }

        impl irony::ConstraintTrait for $name {
            type DataTypeT = $dtype;
            type AttributeT = $attr;
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
                    match self {
                        $($name::$variant(inner) => inner.verify(env, values, uses, defs, regions)),*
                    }
                }
        }

        $(
        impl Into<$name> for $variant_ty {
            fn into(self) -> $name {
                $name::$variant(self)
            }
        }
        )*

        $(
            $(
                #[derive(Default, Clone, Debug, PartialEq)]
                pub struct $variant_ty;
                impl irony::ConstraintTrait for $variant_ty {
                    type DataTypeT = $dtype;
                
                    type AttributeT = $attr;
                
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
                            $block
                    }
                }
            )?

        )*
    };

}