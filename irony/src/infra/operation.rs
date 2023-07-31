use crate::RegionId;

use super::common::Id;
use super::entity::{Entity, EntityId};
use super::environ::Environ;

pub trait Op: Id {
    type AttributeT;
    type ConstraintT;

    fn get_defs(&self) -> Vec<(String, Vec<EntityId>)>;
    fn get_uses(&self) -> Vec<(String, Vec<EntityId>)>;

    fn get_attrs(&self) -> Vec<(String, Vec<Self::AttributeT>)>;
    fn get_constraints(&self) -> Vec<Self::ConstraintT>;

    fn uses(&self, entity: EntityId) -> bool;
    fn defs(&self, entity: EntityId) -> bool;

    fn get_parent(&self) -> Option<RegionId>;
    fn set_parent(&mut self, parent: RegionId);

    fn get_regions(&self) -> Vec<(String, RegionId)>;

    fn use_region(&self, region: RegionId) -> bool;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct OpId(pub usize);
impl From<usize> for OpId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl Id for OpId {
    fn id(&self) -> usize {
        self.0
    }
    fn set_id(&mut self, id: usize) {
        self.0 = id
    }
}

#[macro_export]
macro_rules! op_def {
    (
        [attr = $attr_ty:ty, constraint = $constraint_ty:ty]
        $name_enum:ident = {
            $(
                $name_op:ident: {
                    defs: [$($def:ident),*],
                    uses: [$($use:ident),*$(;$($variadic_use:ident),+)?],
                    attrs: [$($attr:ident),*],
                    constraints: [$($constraint:expr),*],
                    regions: [$($region:ident),*],
                },
            )*
        }
    ) => {

        $(
            irony::op_def_one! {
                [attr = $attr_ty, constraint = $constraint_ty]
                $name_op : {
                    defs : [$($def),*],
                    uses : [$($use),*$(;$($variadic_use),+)?],
                    attrs : [$($attr),*],
                    constraints : [$($constraint),*],
                    regions: [$($region),*],
                }
            }
        )*

        irony::op_enum! {
            [attr = $attr_ty, constraint = $constraint_ty]
            $name_enum = $($name_op),*
        }


    };
}
#[macro_export]
macro_rules! op_def_one {
    (
        [attr = $attr_ty:ty, constraint = $constraint_ty:ty]
        $name:ident : {
            defs: [$($def:ident),*],
            uses: [$($use:ident),*$(;$($variadic_use:ident),+)?],
            attrs: [$($attr:ident),*],
            constraints: [$($constraint:expr),*],
            regions: [$($region:ident),*],
        }
    ) => {
        #[derive(PartialEq, Debug)]
        pub struct $name  {
            id: usize,
            $($def: irony::EntityId,)*
            $($use: irony::EntityId,)*
            $($($variadic_use: Vec<irony::EntityId>,)*)?
            $($attr: $attr_ty,)*
            $($region: irony::RegionId,)*
            constraints: Vec<$constraint_ty>,
            parent: Option<irony::RegionId>,
        }

        impl irony::Id for $name {
            fn id(&self) -> usize {
                self.id
            }

            fn set_id(&mut self, id: usize) {
                self.id = id;
            }
        }

        impl irony::Op for $name {
            type ConstraintT = $constraint_ty;
            type AttributeT = $attr_ty;

            fn get_defs(&self) -> Vec<(String, Vec<irony::EntityId>)> {
                vec![
                    $((format!("{}", stringify!($def)), vec![self.$def])),*
                ]
            }

            fn get_uses(&self) -> Vec<(String, Vec<irony::EntityId>)> {
                vec![
                    $((format!("{}", stringify!($use)), vec![self.$use])),*
                    $($((format!("{}", stringify!($variadic_use)), self.$variadic_use.to_owned()))*)?
                ]

            }

            fn get_attrs(&self) -> Vec<(String, Vec<Self::AttributeT>)> {
                vec![
                    $((format!("{}", stringify!($attr)), vec![self.$attr])),*
                ]
            }

            fn get_constraints(&self) -> Vec<Self::ConstraintT> {
                self.constraints.clone()
            }

            fn uses(&self, entity: irony::EntityId) -> bool {
                self.get_uses().iter().flat_map(|(_, v)| v.iter()).any(|&x| x.id() == entity.id())
            }

            fn defs(&self, entity: irony::EntityId) -> bool {
                self.get_defs().iter().flat_map(|(_, v)| v.iter()).any(|&x| x.id() == entity.id())
            }


            fn get_parent(&self) -> Option<irony::RegionId> {
                self.parent
            }
            fn set_parent(&mut self, parent: irony::RegionId) {
                self.parent = Some(parent)
            }

            fn get_regions(&self) -> Vec<(String, irony::RegionId)> {
                vec![
                    $((format!("{}", stringify!($region)), self.$region)),*
                ]
            }

            fn use_region(&self, region: irony::RegionId) -> bool{
                self.get_regions().iter().any(|(_, id)| *id == region)
            }
        }


        impl $name {
            pub fn new(
                $($def: irony::EntityId,)*
                $($use: irony::EntityId,)*
                $($($variadic_use: Vec<irony::EntityId>,)*)?
                $($attr: $attr_ty,)*
                $($region: irony::RegionId,)*
            ) -> Self {

                Self {
                    id: 0,
                    $($def,)*
                    $($use,)*
                    $($($variadic_use,)*)?
                    $($attr,)*
                    $($region,)*

                    constraints: vec![
                        $($constraint),*
                    ],
                    parent: None,

                }

            }
        }
    };
}

#[macro_export]
macro_rules! op_enum {
    ([attr = $attr:ty, constraint = $constraint:ty] $name:ident = $($variant:ident),*) => {
        #[derive(PartialEq, Debug)]
        pub enum $name {
            $($variant($variant)),*
        }

        $(
            impl Into<$name> for $variant {
                fn into(self) -> $name {
                    $name::$variant(self)
                }
            }
        )*

        impl irony::Id for $name {
            fn id(&self) -> usize {
                match self {
                    $($name::$variant(inner) => inner.id(),)*
                }
            }
            fn set_id(&mut self, id: usize) {
                match self {
                    $($name::$variant(inner) => inner.set_id(id),)*
                }
            }
        }

        impl irony::Op for $name {
            type AttributeT = $attr;
            type ConstraintT = $constraint;

            fn get_defs(&self) -> Vec<(String, Vec<irony::EntityId>)> {
                match self {
                    $($name::$variant(inner) => inner.get_defs()),*
                }
            }
            fn get_uses(&self) -> Vec<(String, Vec<irony::EntityId>)> {
                match self {
                    $($name::$variant(inner) => inner.get_uses()),*
                }
            }

            fn get_attrs(&self) -> Vec<(String, Vec<Self::AttributeT>)> {
                match self {
                    $($name::$variant(inner) => inner.get_attrs()),*
                }

            }
            fn get_constraints(&self) -> Vec<Self::ConstraintT> {
                match self {
                    $($name::$variant(inner) => inner.get_constraints()),*
                }

            }

            fn uses(&self, entity: irony::EntityId) -> bool {
                match self {
                    $($name::$variant(inner) => inner.uses(entity)),*
                }
            }
            fn defs(&self, entity: irony::EntityId) -> bool{
                match self {
                    $($name::$variant(inner) => inner.defs(entity)),*
                }
            }

            fn get_parent(&self) -> Option<irony::RegionId>{
                match self {
                    $($name::$variant(inner) => inner.get_parent()),*
                }
            }
            fn set_parent(&mut self, parent: irony::RegionId) {
                match self {
                    $($name::$variant(inner) => inner.set_parent(parent)),*
                }
            }

            fn get_regions(&self) -> Vec<(String, irony::RegionId)> {
                match self {
                    $($name::$variant(inner) => inner.get_regions()),*
                }
            }


            fn use_region(&self, region: irony::RegionId) -> bool{
                match self {
                    $($name::$variant(inner) => inner.use_region(region)),*
                }
            }
        }
    };
}
