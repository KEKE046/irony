use std::fmt::Debug;

pub use paste::paste;

use super::common::Id;
use super::entity::EntityId;
use crate::printer::OpPrinterTrait;
use crate::{ConstraintTrait, RegionId};

pub trait Op: Id + Debug {
    type DataTypeT;
    type AttributeT;
    type ConstraintT: ConstraintTrait<
        DataTypeT = Self::DataTypeT,
        AttributeT = Self::AttributeT,
    >;
    type PrinterT: OpPrinterTrait<
        DataTypeT = Self::DataTypeT,
        AttributeT = Self::AttributeT,
    >;

    fn get_defs(&self) -> Vec<(String, Vec<Option<EntityId>>)>;
    fn get_uses(&self) -> Vec<(String, Vec<Option<EntityId>>)>;

    fn get_attrs(&self) -> Vec<(String, Self::AttributeT)>;
    fn set_attrs(&mut self, attrs: Vec<(String, Self::AttributeT)>) -> ();
    fn get_constraints(&self) -> Vec<Self::ConstraintT>;

    fn uses(&self, entity: EntityId) -> bool;
    fn defs(&self, entity: EntityId) -> bool;

    fn get_parent(&self) -> Option<RegionId>;
    fn set_parent(&mut self, parent: Option<RegionId>);

    fn get_regions(&self) -> Vec<(String, RegionId)>;

    fn use_region(&self, region: RegionId) -> bool;

    fn get_op_name(&self) -> String;

    fn get_printer(&self) -> Self::PrinterT;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct OpId(pub usize);
impl From<usize> for OpId {
    fn from(value: usize) -> Self { Self(value) }
}
impl Id for OpId {
    fn id(&self) -> usize { self.0 }

    fn set_id(&mut self, id: usize) { self.0 = id }
}

#[macro_export]
macro_rules! op_def {
    (
        [data_type = $data_ty:ty, attr = $attr_ty:ty, constraint = $constraint_ty:ty]
        $name_enum:ident  = {
            $(
                $name:ident : {
                    defs: [$($def:ident),*$(;$($variadic_def:ident),*)?],
                    uses: [$($use:ident),*$(;$($variadic_use:ident),*)?],
                    $(attrs: [$($attr:ident:$attr_variant:ident($attr_inner_ty:ty)),*],)?
                    $(constraints: [$($constraint:expr),*],)?
                    $(regions: [$($region:ident),*],)?
                    print: ($($print_tt:tt)*)$(,)?
                }
            ),*
            $(,)?
        }
    ) => {

        $(
            irony::op_def_one! {
                [data_type = $data_ty, attr = $attr_ty, constraint = $constraint_ty]
                $name: {
                    defs : [$($def),*$(;$($variadic_def),+)?],
                    uses : [$($use),*$(;$($variadic_use),+)?],
                    $(attrs : [$($attr : $attr_variant($attr_inner_ty)),*],)?
                    $(constraints : [$($constraint),*],)?
                    $(regions: [$($region),*],)?
                    print: ($($print_tt)*)
                }
            }
        )*

        irony::op_enum! {
            [data_type = $data_ty, attr = $attr_ty, constraint = $constraint_ty]
            $name_enum = $($name),*
        }

        irony::op_printer! {
            [data_type = $data_ty, attr = $attr_ty]
            $name_enum = $($name),*
        }


    };
}
#[macro_export]
macro_rules! op_def_one {
    (
        [data_type = $data_ty:ty, attr = $attr_ty:ty, constraint = $constraint_ty:ty]
        $name:ident : {
            defs: [$($def:ident),*$(;$($variadic_def:ident),+)?],
            uses: [$($use:ident),*$(;$($variadic_use:ident),+)?],
            $(attrs: [$($attr:ident:$attr_variant:ident($attr_inner_ty:ty)),*],)?
            $(constraints: [$($constraint:expr),*],)?
            $(regions: [$($region:ident),*],)?
            print: ($($print_tt:tt)*)$(,)?
        }
    ) => {
        #[StructFields(pub)]
        #[derive(PartialEq, Debug)]
        pub struct $name  {
            id: usize,
            op_name: String,
            $($def: Option<irony::EntityId>,)*
            $($($variadic_def: Vec<irony::EntityId>,)*)?
            $($use:Option<irony::EntityId>,)*
            $($($variadic_use: Vec<irony::EntityId>,)*)?
            $($($attr: Option<$attr_inner_ty>,)*)?
            $($($region: Option<irony::RegionId>,)*)?
            constraints: Vec<$constraint_ty>,
            parent: Option<irony::RegionId>,
            printer: paste!([< $name Printer >]),
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
            type DataTypeT = $data_ty;
            type ConstraintT = $constraint_ty;
            type AttributeT = $attr_ty;
            type PrinterT = paste!([< $name Printer >]);

            fn get_defs(&self) -> Vec<(String, Vec<Option<irony::EntityId>>)> {
                vec![
                    $((format!("{}", stringify!($def)), vec![self.$def.to_owned()])),*
                    $($((format!("{}", stringify!($variadic_def)), self.$variadic_def.to_owned().into_iter().map(|x| Some(x)).collect()))*)?
                ]
            }

            fn get_uses(&self) -> Vec<(String, Vec<Option<irony::EntityId>>)> {
                vec![
                    $((format!("{}", stringify!($use)), vec![self.$use.to_owned()]),)*
                    $($((format!("{}", stringify!($variadic_use)), self.$variadic_use.to_owned().into_iter().map(|x| Some(x)).collect())),*)?
                ]

            }

            fn get_attrs(&self) -> Vec<(String, Self::AttributeT)> {
                vec![
                    $($((format!("{}", stringify!($attr)), self.$attr.to_owned().unwrap().into())),*)?
                ]
            }

            fn set_attrs(&mut self, attrs: Vec<(String, Self::AttributeT)>) ->() {
                $(
                    $(
                        self.$attr = attrs.iter().find(|(k, _)| k == &format!("{}", stringify!($attr))).map(|(_, v)| v.clone().into());
                    )*
                )?
            }

            fn get_constraints(&self) -> Vec<Self::ConstraintT> {
                self.constraints.clone()
            }

            fn uses(&self, entity: irony::EntityId) -> bool {
                self.get_uses().iter().flat_map(|(_, v)| v.iter()).any(|&x| {
                    if let Some(x) = x {
                        x.id() == entity.id()
                    } else {
                        false
                    }}
                )
            }

            fn defs(&self, entity: irony::EntityId) -> bool {
                self.get_defs().iter().flat_map(|(_, v)| v.iter()).any(|&x| {
                    if let Some(x) = x {
                        x.id() == entity.id()
                    } else {
                        false
                    }}
                )
            }


            fn get_parent(&self) -> Option<irony::RegionId> {
                self.parent
            }
            fn set_parent(&mut self, parent: Option<irony::RegionId>) {
                self.parent = parent;
            }

            fn get_regions(&self) -> Vec<(String, irony::RegionId)> {
                vec![
                    $($((format!("{}", stringify!($region)), self.$region.unwrap())),*)?
                ]
            }

            fn use_region(&self, region: irony::RegionId) -> bool{
                self.get_regions().iter().any(|(_, id)| *id == region)
            }

            fn get_op_name(&self) -> String {
                self.op_name.clone()
            }

            fn get_printer(&self) -> Self::PrinterT {
                self.printer.clone()
            }
        }


        impl $name {
            pub fn new(
                $($def: Option<irony::EntityId>,)*
                $($($variadic_def: Vec<irony::EntityId>,)*)?
                $($use: Option<irony::EntityId>,)*
                $($($variadic_use: Vec<irony::EntityId>,)*)?
                $($($attr: Option<$attr_inner_ty>,)*)?
                $($($region: Option<irony::RegionId>,)*)?
            ) -> Self {

                Self {
                    id: 0,
                    op_name: stringify!($name).to_owned(),
                    $($def,)*
                    $($($variadic_def,)*)?
                    $($use,)*
                    $($($variadic_use,)*)?
                    $($($attr,)*)?
                    $($($region,)*)?

                    constraints: vec![
                        $($($constraint),*)?
                    ],
                    parent: None,
                    printer: paste!([< $name Printer >]),
                }

            }
        }

        paste! {
            #[derive(Clone, Debug, PartialEq)]
            pub struct [< $name Printer >];

            impl OpPrinterTrait for [< $name Printer >] {
                type DataTypeT = $data_ty;
                type AttributeT = $attr_ty;

                fn print<'env, E, EntityT: Entity>(
                    &self,
                    env: &'env E,
                    attrs: Vec<(String, Self::AttributeT)>,
                    uses: Vec<(String, Vec<Option<irony::EntityId>>)>,
                    defs: Vec<(String, Vec<Option<irony::EntityId>>)>,
                    regions: Vec<(String, irony::RegionId)>,
                ) -> String
                where
                    E: Environ<EntityT = EntityT, AttributeT = Self::AttributeT>,
                    EntityT: Entity<DataTypeT = Self::DataTypeT, AttributeT = Self::AttributeT> {
                        let f = $($print_tt)*;
                        f(env, attrs, uses, defs, regions)
                    }
            }

        }

    };
}

#[macro_export]
macro_rules! op_enum {
    ([data_type = $data_ty:ty, attr = $attr:ty, constraint = $constraint:ty] $name:ident = $($variant:ident),*) => {
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
            type DataTypeT = $data_ty;
            type AttributeT = $attr;
            type ConstraintT = $constraint;
            type PrinterT = paste!([< $name Printer >]);

            fn get_defs(&self) -> Vec<(String, Vec<Option<irony::EntityId>>)> {
                match self {
                    $($name::$variant(inner) => inner.get_defs()),*
                }
            }
            fn get_uses(&self) -> Vec<(String, Vec<Option<irony::EntityId>>)> {
                match self {
                    $($name::$variant(inner) => inner.get_uses()),*
                }
            }

            fn get_attrs(&self) -> Vec<(String, Self::AttributeT)> {
                match self {
                    $($name::$variant(inner) => inner.get_attrs()),*
                }
            }

            fn set_attrs(&mut self, attrs: Vec<(String, Self::AttributeT)>) -> () {
                match self {
                    $($name::$variant(inner) => inner.set_attrs(attrs)),*
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
            fn set_parent(&mut self, parent: Option<irony::RegionId>) {
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

            fn get_op_name(&self) -> String {
                match self {
                    $($name::$variant(inner) => inner.get_op_name()),*
                }
            }

            fn get_printer(&self) -> Self::PrinterT {
                match self {
                    $($name::$variant(inner) => inner.get_printer().into()),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! op_printer {
    (
        [data_type = $data_ty:ty, attr = $attr:ty]
        $name:ident = $($variant:ident),*
    ) => {
        paste! {
            pub enum [<$name Printer>] {
                $($variant([<$variant Printer>])),*
            }

            impl irony::OpPrinterTrait for [<$name Printer>] {
                type DataTypeT = $data_ty;
                type AttributeT = $attr;

                fn print<'env, E, EntityT: Entity>(
                    &self,
                    env: &'env E,
                    attrs: Vec<(String, Self::AttributeT)>,
                    uses: Vec<(String, Vec<Option<irony::EntityId>>)>,
                    defs: Vec<(String, Vec<Option<irony::EntityId>>)>,
                    regions: Vec<(String, irony::RegionId)>,
                ) -> String
                where
                    E: Environ<EntityT = EntityT, AttributeT = Self::AttributeT>,
                    EntityT: Entity<DataTypeT = Self::DataTypeT, AttributeT = Self::AttributeT> {
                        match self {
                            $([<$name Printer>]::$variant(inner) => inner.print(env, attrs, uses, defs, regions)),*
                        }
                    }
            }

            $(

                impl Into<[<$name Printer>]> for [<$variant Printer>] {
                    fn into(self) -> [<$name Printer>] {
                        [<$name Printer>]::$variant(self)
                    }
                }

            )*
        }
    };
}
