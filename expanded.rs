#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod circt {
    use irony::{self, preclude::*};
    pub enum DataTypeEnum {
        UInt(usize),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DataTypeEnum {
        #[inline]
        fn clone(&self) -> DataTypeEnum {
            let _: ::core::clone::AssertParamIsClone<usize>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for DataTypeEnum {}
    #[automatically_derived]
    impl ::core::fmt::Debug for DataTypeEnum {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DataTypeEnum::UInt(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "UInt",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DataTypeEnum {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DataTypeEnum {
        #[inline]
        fn eq(&self, other: &DataTypeEnum) -> bool {
            match (self, other) {
                (DataTypeEnum::UInt(__self_0), DataTypeEnum::UInt(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
            }
        }
    }
    pub enum AttributeEnum {
        ConstValue(irony::ConstValueI32<DataTypeEnum>),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AttributeEnum {
        #[inline]
        fn clone(&self) -> AttributeEnum {
            let _: ::core::clone::AssertParamIsClone<irony::ConstValueI32<DataTypeEnum>>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AttributeEnum {}
    #[automatically_derived]
    impl ::core::fmt::Debug for AttributeEnum {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AttributeEnum::ConstValue(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ConstValue",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AttributeEnum {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AttributeEnum {
        #[inline]
        fn eq(&self, other: &AttributeEnum) -> bool {
            match (self, other) {
                (
                    AttributeEnum::ConstValue(__self_0),
                    AttributeEnum::ConstValue(__arg1_0),
                ) => *__self_0 == *__arg1_0,
            }
        }
    }
    impl irony::AttributeTrait<DataTypeEnum> for AttributeEnum {
        fn dtype(&self) -> DataTypeEnum {
            match self {
                AttributeEnum::ConstValue(inner) => inner.dtype(),
            }
        }
    }
    impl Into<AttributeEnum> for irony::ConstValueI32<DataTypeEnum> {
        fn into(self) -> AttributeEnum {
            AttributeEnum::ConstValue(self)
        }
    }
    type SameType = irony::SameTypeConstraint<DataTypeEnum, AttributeEnum>;
    pub enum ConstraintEnum {
        SameType(SameType),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstraintEnum {
        #[inline]
        fn clone(&self) -> ConstraintEnum {
            let _: ::core::clone::AssertParamIsClone<SameType>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ConstraintEnum {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstraintEnum {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ConstraintEnum::SameType(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SameType",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ConstraintEnum {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ConstraintEnum {
        #[inline]
        fn eq(&self, other: &ConstraintEnum) -> bool {
            match (self, other) {
                (
                    ConstraintEnum::SameType(__self_0),
                    ConstraintEnum::SameType(__arg1_0),
                ) => *__self_0 == *__arg1_0,
            }
        }
    }
    impl irony::ConstraintTrait for ConstraintEnum {
        type DataTypeT = DataTypeEnum;
        type AttributeT = AttributeEnum;
        fn verify<'env, E, EntityT: irony::Entity>(
            &self,
            env: &'env E,
            values: Vec<(String, Vec<Self::AttributeT>)>,
            uses: Vec<(String, Vec<irony::EntityId>)>,
            defs: Vec<(String, Vec<irony::EntityId>)>,
        ) -> bool
        where
            E: irony::Environ<EntityT = EntityT>,
            EntityT: irony::Entity<DataTypeT = Self::DataTypeT>,
        {
            match self {
                ConstraintEnum::SameType(inner) => inner.verify(env, values, uses, defs),
            }
        }
    }
    impl Into<ConstraintEnum> for SameType {
        fn into(self) -> ConstraintEnum {
            ConstraintEnum::SameType(self)
        }
    }
    pub struct Wire {
        pub id: usize,
        pub sym: irony::Symbol,
        pub parent: Option<irony::RegionId>,
        pub dtype: Option<DataTypeEnum>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Wire {
        #[inline]
        fn clone(&self) -> Wire {
            Wire {
                id: ::core::clone::Clone::clone(&self.id),
                sym: ::core::clone::Clone::clone(&self.sym),
                parent: ::core::clone::Clone::clone(&self.parent),
                dtype: ::core::clone::Clone::clone(&self.dtype),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Wire {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Wire",
                "id",
                &self.id,
                "sym",
                &self.sym,
                "parent",
                &self.parent,
                "dtype",
                &&self.dtype,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Wire {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Wire {
        #[inline]
        fn eq(&self, other: &Wire) -> bool {
            self.id == other.id && self.sym == other.sym && self.parent == other.parent
                && self.dtype == other.dtype
        }
    }
    impl irony::Entity for Wire {
        type DataTypeT = DataTypeEnum;
        fn get_def<E: irony::Environ>(&self, env: &E) -> Option<irony::OpId> {
            env.get_def(self.as_id())
        }
        fn get_uses<E: irony::Environ>(&self, env: &E) -> Vec<irony::OpId> {
            env.get_uses(self.as_id())
        }
        fn get_dtype(&self) -> Option<Self::DataTypeT> {
            self.dtype
        }
        fn as_id(&self) -> irony::EntityId {
            irony::EntityId(self.id)
        }
        fn get_parent(&self) -> Option<irony::RegionId> {
            self.parent
        }
        fn set_parent(&mut self, parent: irony::RegionId) {
            self.parent = Some(parent);
        }
    }
    impl irony::Id for Wire {
        fn id(&self) -> usize {
            self.id
        }
        fn set_id(&mut self, id: usize) {
            self.id = id;
        }
    }
    impl Wire {
        pub fn new(name: &str, dtype: DataTypeEnum) -> Self {
            Self {
                id: 0,
                sym: irony::Symbol::new(String::from(name)),
                dtype: Some(dtype),
                parent: None,
            }
        }
    }
    pub struct Module {
        pub id: usize,
        pub sym: irony::Symbol,
        pub parent: Option<irony::RegionId>,
        pub dtype: Option<DataTypeEnum>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Module {
        #[inline]
        fn clone(&self) -> Module {
            Module {
                id: ::core::clone::Clone::clone(&self.id),
                sym: ::core::clone::Clone::clone(&self.sym),
                parent: ::core::clone::Clone::clone(&self.parent),
                dtype: ::core::clone::Clone::clone(&self.dtype),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Module {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Module",
                "id",
                &self.id,
                "sym",
                &self.sym,
                "parent",
                &self.parent,
                "dtype",
                &&self.dtype,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Module {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Module {
        #[inline]
        fn eq(&self, other: &Module) -> bool {
            self.id == other.id && self.sym == other.sym && self.parent == other.parent
                && self.dtype == other.dtype
        }
    }
    impl irony::Entity for Module {
        type DataTypeT = DataTypeEnum;
        fn get_def<E: irony::Environ>(&self, env: &E) -> Option<irony::OpId> {
            env.get_def(self.as_id())
        }
        fn get_uses<E: irony::Environ>(&self, env: &E) -> Vec<irony::OpId> {
            env.get_uses(self.as_id())
        }
        fn get_dtype(&self) -> Option<Self::DataTypeT> {
            self.dtype
        }
        fn as_id(&self) -> irony::EntityId {
            irony::EntityId(self.id)
        }
        fn get_parent(&self) -> Option<irony::RegionId> {
            self.parent
        }
        fn set_parent(&mut self, parent: irony::RegionId) {
            self.parent = Some(parent);
        }
    }
    impl irony::Id for Module {
        fn id(&self) -> usize {
            self.id
        }
        fn set_id(&mut self, id: usize) {
            self.id = id;
        }
    }
    impl Module {
        pub fn new(name: &str) -> Self {
            Self {
                id: 0,
                sym: irony::Symbol::new(String::from(name)),
                dtype: None,
                parent: None,
            }
        }
    }
    pub enum EntityEnum {
        Wire(Wire),
        Module(Module),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EntityEnum {
        #[inline]
        fn clone(&self) -> EntityEnum {
            match self {
                EntityEnum::Wire(__self_0) => {
                    EntityEnum::Wire(::core::clone::Clone::clone(__self_0))
                }
                EntityEnum::Module(__self_0) => {
                    EntityEnum::Module(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EntityEnum {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                EntityEnum::Wire(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Wire",
                        &__self_0,
                    )
                }
                EntityEnum::Module(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Module",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EntityEnum {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EntityEnum {
        #[inline]
        fn eq(&self, other: &EntityEnum) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (EntityEnum::Wire(__self_0), EntityEnum::Wire(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (EntityEnum::Module(__self_0), EntityEnum::Module(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    impl irony::Entity for EntityEnum {
        type DataTypeT = DataTypeEnum;
        fn get_def<E: irony::Environ>(&self, env: &E) -> Option<irony::OpId> {
            match self {
                EntityEnum::Wire(inner) => inner.get_def(env),
                EntityEnum::Module(inner) => inner.get_def(env),
            }
        }
        fn get_uses<E: irony::Environ>(&self, env: &E) -> Vec<irony::OpId> {
            match self {
                EntityEnum::Wire(inner) => inner.get_uses(env),
                EntityEnum::Module(inner) => inner.get_uses(env),
            }
        }
        fn get_dtype(&self) -> Option<Self::DataTypeT> {
            match self {
                EntityEnum::Wire(inner) => inner.get_dtype(),
                EntityEnum::Module(inner) => inner.get_dtype(),
            }
        }
        fn as_id(&self) -> irony::EntityId {
            match self {
                EntityEnum::Wire(inner) => inner.as_id(),
                EntityEnum::Module(inner) => inner.as_id(),
            }
        }
        fn get_parent(&self) -> Option<irony::RegionId> {
            match self {
                EntityEnum::Wire(inner) => inner.get_parent(),
                EntityEnum::Module(inner) => inner.get_parent(),
            }
        }
        fn set_parent(&mut self, parent: irony::RegionId) {
            match self {
                EntityEnum::Wire(inner) => inner.set_parent(parent),
                EntityEnum::Module(inner) => inner.set_parent(parent),
            }
        }
    }
    impl irony::Id for EntityEnum {
        fn id(&self) -> usize {
            match self {
                EntityEnum::Wire(inner) => inner.id(),
                EntityEnum::Module(inner) => inner.id(),
            }
        }
        fn set_id(&mut self, id: usize) {
            match self {
                EntityEnum::Wire(inner) => inner.set_id(id),
                EntityEnum::Module(inner) => inner.set_id(id),
            }
        }
    }
    impl Into<EntityEnum> for Wire {
        fn into(self) -> EntityEnum {
            EntityEnum::Wire(self)
        }
    }
    impl Into<EntityEnum> for Module {
        fn into(self) -> EntityEnum {
            EntityEnum::Module(self)
        }
    }
    impl Into<Wire> for EntityEnum {
        fn into(self) -> Wire {
            match self {
                EntityEnum::Wire(inner) => inner,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "match fails, check variant {0} and enum {1}", "Wire",
                            "EntityEnum"
                        ),
                    )
                }
            }
        }
    }
    impl Into<Module> for EntityEnum {
        fn into(self) -> Module {
            match self {
                EntityEnum::Module(inner) => inner,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "match fails, check variant {0} and enum {1}", "Module",
                            "EntityEnum"
                        ),
                    )
                }
            }
        }
    }
    pub struct Constant {
        id: usize,
        lhs: irony::EntityId,
        rhs: AttributeEnum,
        constraints: Vec<ConstraintEnum>,
        parent: Option<irony::RegionId>,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Constant {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Constant {
        #[inline]
        fn eq(&self, other: &Constant) -> bool {
            self.id == other.id && self.lhs == other.lhs && self.rhs == other.rhs
                && self.constraints == other.constraints && self.parent == other.parent
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Constant {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "Constant",
                "id",
                &self.id,
                "lhs",
                &self.lhs,
                "rhs",
                &self.rhs,
                "constraints",
                &self.constraints,
                "parent",
                &&self.parent,
            )
        }
    }
    impl irony::Id for Constant {
        fn id(&self) -> usize {
            self.id
        }
        fn set_id(&mut self, id: usize) {
            self.id = id;
        }
    }
    impl irony::Op for Constant {
        type ConstraintT = ConstraintEnum;
        type AttributeT = AttributeEnum;
        fn get_defs(&self) -> Vec<(String, Vec<irony::EntityId>)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    (
                        {
                            let res = ::alloc::fmt::format(format_args!("{0}", "lhs"));
                            res
                        },
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([self.lhs]),
                        ),
                    ),
                ]),
            )
        }
        fn get_uses(&self) -> Vec<(String, Vec<irony::EntityId>)> {
            ::alloc::vec::Vec::new()
        }
        fn get_attrs(&self) -> Vec<(String, Vec<Self::AttributeT>)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    (
                        {
                            let res = ::alloc::fmt::format(format_args!("{0}", "rhs"));
                            res
                        },
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([self.rhs]),
                        ),
                    ),
                ]),
            )
        }
        fn get_constraints(&self) -> Vec<Self::ConstraintT> {
            self.constraints.clone()
        }
        fn uses(&self, entity: irony::EntityId) -> bool {
            self.get_uses()
                .iter()
                .flat_map(|(_, v)| v.iter())
                .any(|&x| x.id() == entity.id())
        }
        fn defs(&self, entity: irony::EntityId) -> bool {
            self.get_defs()
                .iter()
                .flat_map(|(_, v)| v.iter())
                .any(|&x| x.id() == entity.id())
        }
        fn get_parent(&self) -> Option<irony::RegionId> {
            self.parent
        }
        fn set_parent(&mut self, parent: irony::RegionId) {
            self.parent = Some(parent);
        }
        fn get_regions(&self) -> Vec<(String, irony::RegionId)> {
            ::alloc::vec::Vec::new()
        }
        fn use_region(&self, region: irony::RegionId) -> bool {
            self.get_regions().iter().any(|(_, id)| *id == region)
        }
    }
    impl Constant {
        pub fn new(lhs: irony::EntityId, rhs: AttributeEnum) -> Self {
            Self {
                id: 0,
                lhs,
                rhs,
                constraints: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([SameType::new().into()]),
                ),
                parent: None,
            }
        }
    }
    pub struct Assign {
        id: usize,
        lhs: irony::EntityId,
        rhs: irony::EntityId,
        constraints: Vec<ConstraintEnum>,
        parent: Option<irony::RegionId>,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Assign {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Assign {
        #[inline]
        fn eq(&self, other: &Assign) -> bool {
            self.id == other.id && self.lhs == other.lhs && self.rhs == other.rhs
                && self.constraints == other.constraints && self.parent == other.parent
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Assign {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "Assign",
                "id",
                &self.id,
                "lhs",
                &self.lhs,
                "rhs",
                &self.rhs,
                "constraints",
                &self.constraints,
                "parent",
                &&self.parent,
            )
        }
    }
    impl irony::Id for Assign {
        fn id(&self) -> usize {
            self.id
        }
        fn set_id(&mut self, id: usize) {
            self.id = id;
        }
    }
    impl irony::Op for Assign {
        type ConstraintT = ConstraintEnum;
        type AttributeT = AttributeEnum;
        fn get_defs(&self) -> Vec<(String, Vec<irony::EntityId>)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    (
                        {
                            let res = ::alloc::fmt::format(format_args!("{0}", "lhs"));
                            res
                        },
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([self.lhs]),
                        ),
                    ),
                ]),
            )
        }
        fn get_uses(&self) -> Vec<(String, Vec<irony::EntityId>)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    (
                        {
                            let res = ::alloc::fmt::format(format_args!("{0}", "rhs"));
                            res
                        },
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([self.rhs]),
                        ),
                    ),
                ]),
            )
        }
        fn get_attrs(&self) -> Vec<(String, Vec<Self::AttributeT>)> {
            ::alloc::vec::Vec::new()
        }
        fn get_constraints(&self) -> Vec<Self::ConstraintT> {
            self.constraints.clone()
        }
        fn uses(&self, entity: irony::EntityId) -> bool {
            self.get_uses()
                .iter()
                .flat_map(|(_, v)| v.iter())
                .any(|&x| x.id() == entity.id())
        }
        fn defs(&self, entity: irony::EntityId) -> bool {
            self.get_defs()
                .iter()
                .flat_map(|(_, v)| v.iter())
                .any(|&x| x.id() == entity.id())
        }
        fn get_parent(&self) -> Option<irony::RegionId> {
            self.parent
        }
        fn set_parent(&mut self, parent: irony::RegionId) {
            self.parent = Some(parent);
        }
        fn get_regions(&self) -> Vec<(String, irony::RegionId)> {
            ::alloc::vec::Vec::new()
        }
        fn use_region(&self, region: irony::RegionId) -> bool {
            self.get_regions().iter().any(|(_, id)| *id == region)
        }
    }
    impl Assign {
        pub fn new(lhs: irony::EntityId, rhs: irony::EntityId) -> Self {
            Self {
                id: 0,
                lhs,
                rhs,
                constraints: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([SameType::new().into()]),
                ),
                parent: None,
            }
        }
    }
    pub struct ModuleDef {
        id: usize,
        lhs: irony::EntityId,
        region: irony::RegionId,
        constraints: Vec<ConstraintEnum>,
        parent: Option<irony::RegionId>,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ModuleDef {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ModuleDef {
        #[inline]
        fn eq(&self, other: &ModuleDef) -> bool {
            self.id == other.id && self.lhs == other.lhs && self.region == other.region
                && self.constraints == other.constraints && self.parent == other.parent
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ModuleDef {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "ModuleDef",
                "id",
                &self.id,
                "lhs",
                &self.lhs,
                "region",
                &self.region,
                "constraints",
                &self.constraints,
                "parent",
                &&self.parent,
            )
        }
    }
    impl irony::Id for ModuleDef {
        fn id(&self) -> usize {
            self.id
        }
        fn set_id(&mut self, id: usize) {
            self.id = id;
        }
    }
    impl irony::Op for ModuleDef {
        type ConstraintT = ConstraintEnum;
        type AttributeT = AttributeEnum;
        fn get_defs(&self) -> Vec<(String, Vec<irony::EntityId>)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    (
                        {
                            let res = ::alloc::fmt::format(format_args!("{0}", "lhs"));
                            res
                        },
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([self.lhs]),
                        ),
                    ),
                ]),
            )
        }
        fn get_uses(&self) -> Vec<(String, Vec<irony::EntityId>)> {
            ::alloc::vec::Vec::new()
        }
        fn get_attrs(&self) -> Vec<(String, Vec<Self::AttributeT>)> {
            ::alloc::vec::Vec::new()
        }
        fn get_constraints(&self) -> Vec<Self::ConstraintT> {
            self.constraints.clone()
        }
        fn uses(&self, entity: irony::EntityId) -> bool {
            self.get_uses()
                .iter()
                .flat_map(|(_, v)| v.iter())
                .any(|&x| x.id() == entity.id())
        }
        fn defs(&self, entity: irony::EntityId) -> bool {
            self.get_defs()
                .iter()
                .flat_map(|(_, v)| v.iter())
                .any(|&x| x.id() == entity.id())
        }
        fn get_parent(&self) -> Option<irony::RegionId> {
            self.parent
        }
        fn set_parent(&mut self, parent: irony::RegionId) {
            self.parent = Some(parent);
        }
        fn get_regions(&self) -> Vec<(String, irony::RegionId)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    (
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("{0}", "region"),
                            );
                            res
                        },
                        self.region,
                    ),
                ]),
            )
        }
        fn use_region(&self, region: irony::RegionId) -> bool {
            self.get_regions().iter().any(|(_, id)| *id == region)
        }
    }
    impl ModuleDef {
        pub fn new(lhs: irony::EntityId, region: irony::RegionId) -> Self {
            Self {
                id: 0,
                lhs,
                region,
                constraints: ::alloc::vec::Vec::new(),
                parent: None,
            }
        }
    }
    pub struct HardFor {
        id: usize,
        stmt: irony::EntityId,
        loop_var: irony::EntityId,
        loop_bound: AttributeEnum,
        loop_body: irony::RegionId,
        constraints: Vec<ConstraintEnum>,
        parent: Option<irony::RegionId>,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for HardFor {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for HardFor {
        #[inline]
        fn eq(&self, other: &HardFor) -> bool {
            self.id == other.id && self.stmt == other.stmt
                && self.loop_var == other.loop_var && self.loop_bound == other.loop_bound
                && self.loop_body == other.loop_body
                && self.constraints == other.constraints && self.parent == other.parent
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for HardFor {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "stmt",
                "loop_var",
                "loop_bound",
                "loop_body",
                "constraints",
                "parent",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.stmt,
                &self.loop_var,
                &self.loop_bound,
                &self.loop_body,
                &self.constraints,
                &&self.parent,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "HardFor",
                names,
                values,
            )
        }
    }
    impl irony::Id for HardFor {
        fn id(&self) -> usize {
            self.id
        }
        fn set_id(&mut self, id: usize) {
            self.id = id;
        }
    }
    impl irony::Op for HardFor {
        type ConstraintT = ConstraintEnum;
        type AttributeT = AttributeEnum;
        fn get_defs(&self) -> Vec<(String, Vec<irony::EntityId>)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    (
                        {
                            let res = ::alloc::fmt::format(format_args!("{0}", "stmt"));
                            res
                        },
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([self.stmt]),
                        ),
                    ),
                ]),
            )
        }
        fn get_uses(&self) -> Vec<(String, Vec<irony::EntityId>)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    (
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("{0}", "loop_var"),
                            );
                            res
                        },
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([self.loop_var]),
                        ),
                    ),
                ]),
            )
        }
        fn get_attrs(&self) -> Vec<(String, Vec<Self::AttributeT>)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    (
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("{0}", "loop_bound"),
                            );
                            res
                        },
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([self.loop_bound]),
                        ),
                    ),
                ]),
            )
        }
        fn get_constraints(&self) -> Vec<Self::ConstraintT> {
            self.constraints.clone()
        }
        fn uses(&self, entity: irony::EntityId) -> bool {
            self.get_uses()
                .iter()
                .flat_map(|(_, v)| v.iter())
                .any(|&x| x.id() == entity.id())
        }
        fn defs(&self, entity: irony::EntityId) -> bool {
            self.get_defs()
                .iter()
                .flat_map(|(_, v)| v.iter())
                .any(|&x| x.id() == entity.id())
        }
        fn get_parent(&self) -> Option<irony::RegionId> {
            self.parent
        }
        fn set_parent(&mut self, parent: irony::RegionId) {
            self.parent = Some(parent);
        }
        fn get_regions(&self) -> Vec<(String, irony::RegionId)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    (
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("{0}", "loop_body"),
                            );
                            res
                        },
                        self.loop_body,
                    ),
                ]),
            )
        }
        fn use_region(&self, region: irony::RegionId) -> bool {
            self.get_regions().iter().any(|(_, id)| *id == region)
        }
    }
    impl HardFor {
        pub fn new(
            stmt: irony::EntityId,
            loop_var: irony::EntityId,
            loop_bound: AttributeEnum,
            loop_body: irony::RegionId,
        ) -> Self {
            Self {
                id: 0,
                stmt,
                loop_var,
                loop_bound,
                loop_body,
                constraints: ::alloc::vec::Vec::new(),
                parent: None,
            }
        }
    }
    pub enum OpEnum {
        Constant(Constant),
        Assign(Assign),
        ModuleDef(ModuleDef),
        HardFor(HardFor),
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for OpEnum {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for OpEnum {
        #[inline]
        fn eq(&self, other: &OpEnum) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (OpEnum::Constant(__self_0), OpEnum::Constant(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (OpEnum::Assign(__self_0), OpEnum::Assign(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (OpEnum::ModuleDef(__self_0), OpEnum::ModuleDef(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (OpEnum::HardFor(__self_0), OpEnum::HardFor(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OpEnum {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                OpEnum::Constant(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Constant",
                        &__self_0,
                    )
                }
                OpEnum::Assign(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Assign",
                        &__self_0,
                    )
                }
                OpEnum::ModuleDef(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ModuleDef",
                        &__self_0,
                    )
                }
                OpEnum::HardFor(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "HardFor",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl Into<OpEnum> for Constant {
        fn into(self) -> OpEnum {
            OpEnum::Constant(self)
        }
    }
    impl Into<OpEnum> for Assign {
        fn into(self) -> OpEnum {
            OpEnum::Assign(self)
        }
    }
    impl Into<OpEnum> for ModuleDef {
        fn into(self) -> OpEnum {
            OpEnum::ModuleDef(self)
        }
    }
    impl Into<OpEnum> for HardFor {
        fn into(self) -> OpEnum {
            OpEnum::HardFor(self)
        }
    }
    impl irony::Id for OpEnum {
        fn id(&self) -> usize {
            match self {
                OpEnum::Constant(inner) => inner.id(),
                OpEnum::Assign(inner) => inner.id(),
                OpEnum::ModuleDef(inner) => inner.id(),
                OpEnum::HardFor(inner) => inner.id(),
            }
        }
        fn set_id(&mut self, id: usize) {
            match self {
                OpEnum::Constant(inner) => inner.set_id(id),
                OpEnum::Assign(inner) => inner.set_id(id),
                OpEnum::ModuleDef(inner) => inner.set_id(id),
                OpEnum::HardFor(inner) => inner.set_id(id),
            }
        }
    }
    impl irony::Op for OpEnum {
        type AttributeT = AttributeEnum;
        type ConstraintT = ConstraintEnum;
        fn get_defs(&self) -> Vec<(String, Vec<irony::EntityId>)> {
            match self {
                OpEnum::Constant(inner) => inner.get_defs(),
                OpEnum::Assign(inner) => inner.get_defs(),
                OpEnum::ModuleDef(inner) => inner.get_defs(),
                OpEnum::HardFor(inner) => inner.get_defs(),
            }
        }
        fn get_uses(&self) -> Vec<(String, Vec<irony::EntityId>)> {
            match self {
                OpEnum::Constant(inner) => inner.get_uses(),
                OpEnum::Assign(inner) => inner.get_uses(),
                OpEnum::ModuleDef(inner) => inner.get_uses(),
                OpEnum::HardFor(inner) => inner.get_uses(),
            }
        }
        fn get_attrs(&self) -> Vec<(String, Vec<Self::AttributeT>)> {
            match self {
                OpEnum::Constant(inner) => inner.get_attrs(),
                OpEnum::Assign(inner) => inner.get_attrs(),
                OpEnum::ModuleDef(inner) => inner.get_attrs(),
                OpEnum::HardFor(inner) => inner.get_attrs(),
            }
        }
        fn get_constraints(&self) -> Vec<Self::ConstraintT> {
            match self {
                OpEnum::Constant(inner) => inner.get_constraints(),
                OpEnum::Assign(inner) => inner.get_constraints(),
                OpEnum::ModuleDef(inner) => inner.get_constraints(),
                OpEnum::HardFor(inner) => inner.get_constraints(),
            }
        }
        fn uses(&self, entity: irony::EntityId) -> bool {
            match self {
                OpEnum::Constant(inner) => inner.uses(entity),
                OpEnum::Assign(inner) => inner.uses(entity),
                OpEnum::ModuleDef(inner) => inner.uses(entity),
                OpEnum::HardFor(inner) => inner.uses(entity),
            }
        }
        fn defs(&self, entity: irony::EntityId) -> bool {
            match self {
                OpEnum::Constant(inner) => inner.defs(entity),
                OpEnum::Assign(inner) => inner.defs(entity),
                OpEnum::ModuleDef(inner) => inner.defs(entity),
                OpEnum::HardFor(inner) => inner.defs(entity),
            }
        }
        fn get_parent(&self) -> Option<irony::RegionId> {
            match self {
                OpEnum::Constant(inner) => inner.get_parent(),
                OpEnum::Assign(inner) => inner.get_parent(),
                OpEnum::ModuleDef(inner) => inner.get_parent(),
                OpEnum::HardFor(inner) => inner.get_parent(),
            }
        }
        fn set_parent(&mut self, parent: irony::RegionId) {
            match self {
                OpEnum::Constant(inner) => inner.set_parent(parent),
                OpEnum::Assign(inner) => inner.set_parent(parent),
                OpEnum::ModuleDef(inner) => inner.set_parent(parent),
                OpEnum::HardFor(inner) => inner.set_parent(parent),
            }
        }
        fn get_regions(&self) -> Vec<(String, irony::RegionId)> {
            match self {
                OpEnum::Constant(inner) => inner.get_regions(),
                OpEnum::Assign(inner) => inner.get_regions(),
                OpEnum::ModuleDef(inner) => inner.get_regions(),
                OpEnum::HardFor(inner) => inner.get_regions(),
            }
        }
        fn use_region(&self, region: irony::RegionId) -> bool {
            match self {
                OpEnum::Constant(inner) => inner.use_region(region),
                OpEnum::Assign(inner) => inner.use_region(region),
                OpEnum::ModuleDef(inner) => inner.use_region(region),
                OpEnum::HardFor(inner) => inner.use_region(region),
            }
        }
    }
    pub struct CirctEnv {
        op_table: irony::FxMapWithUniqueId<OpEnum>,
        entity_table: irony::FxMapWithUniqueId<EntityEnum>,
        region_table: irony::FxMapWithUniqueId<irony::Region>,
        parent_stack: Vec<irony::RegionId>,
    }
    #[automatically_derived]
    impl ::core::default::Default for CirctEnv {
        #[inline]
        fn default() -> CirctEnv {
            CirctEnv {
                op_table: ::core::default::Default::default(),
                entity_table: ::core::default::Default::default(),
                region_table: ::core::default::Default::default(),
                parent_stack: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CirctEnv {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "CirctEnv",
                "op_table",
                &self.op_table,
                "entity_table",
                &self.entity_table,
                "region_table",
                &self.region_table,
                "parent_stack",
                &&self.parent_stack,
            )
        }
    }
    impl irony::Environ for CirctEnv {
        type OpT = OpEnum;
        type EntityT = EntityEnum;
        type ConstraintT = ConstraintEnum;
        type AttributeT = AttributeEnum;
        fn get_def(&self, entity: irony::EntityId) -> Option<irony::OpId> {
            self.op_table
                .iter()
                .find(|tuple| tuple.1.defs(entity))
                .map(|tuple| irony::OpId::from(*tuple.0))
        }
        fn get_uses(&self, entity: irony::EntityId) -> Vec<irony::OpId> {
            {
                ::std::io::_print(
                    format_args!("try to run get_uses() for entity {0:?}\n\n", entity),
                );
            };
            let mut v = Vec::new();
            for (id, op) in self.op_table.get_map() {
                if op.uses(irony::EntityId::from(entity.to_owned())) {
                    v.push(irony::OpId::from(*id));
                }
            }
            v
        }
        fn get_entity(&self, id: irony::EntityId) -> &Self::EntityT {
            match self.entity_table.get(&id.id()) {
                Some(entity) => entity,
                None => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "get entity not in the table by id \ntable: {0:#?}\nentity: {1:#?}",
                            self.entity_table.get_map(), id.id()
                        ),
                    )
                }
            }
        }
        fn get_entities(&self, ids: &[irony::EntityId]) -> Vec<&Self::EntityT> {
            ids.iter().map(|id| self.get_entity(id.to_owned())).collect()
        }
        fn get_op(&self, id: irony::OpId) -> &Self::OpT {
            match self.op_table.get(&id.id()) {
                Some(op) => op,
                None => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "get op not in the table by id \ntable: {0:#?}\nop: {1:#?}",
                            self.op_table.get_map(), id.id()
                        ),
                    )
                }
            }
        }
        fn get_ops(&self, ids: &[irony::OpId]) -> Vec<&Self::OpT> {
            ids.iter().map(|id| self.get_op(id.to_owned())).collect()
        }
        fn add_entity(&mut self, entity: Self::EntityT) -> irony::EntityId {
            let (id, _) = self.entity_table.insert_with_id(entity);
            self.set_entity_parent(irony::EntityId::from(id));
            irony::EntityId(id)
        }
        fn get_region(&self, id: irony::RegionId) -> &irony::Region {
            match self.region_table.get(&id.id()) {
                Some(region) => region,
                None => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "get region not in the table by id \ntable: {0:#?}\nregion: {1:#?}",
                            self.region_table.get_map(), id.id()
                        ),
                    )
                }
            }
        }
        fn add_region(&mut self, region: irony::Region) -> irony::RegionId {
            let (id, _) = self.region_table.insert_with_id(region);
            irony::RegionId(id)
        }
        fn add_op(&mut self, op: Self::OpT) -> irony::OpId {
            let (id, op) = self.op_table.insert_with_id(op);
            self.set_op_parent(irony::OpId::from(id));
            irony::OpId(id)
        }
        fn set_entity_parent(&mut self, id: irony::EntityId) {
            if let Some(parent) = self.parent_stack.last() {
                self.entity_table
                    .entry(id.id())
                    .and_modify(|entity| entity.set_parent(parent.to_owned()));
                self.region_table
                    .entry(parent.id())
                    .and_modify(|region| {
                        region.add_entity_child(irony::EntityId(id.id()))
                    });
            }
        }
        fn set_op_parent(&mut self, id: irony::OpId) {
            if let Some(parent) = self.parent_stack.last() {
                self.op_table
                    .entry(id.id())
                    .and_modify(|entity| entity.set_parent(parent.to_owned()));
                self.region_table
                    .entry(parent.id())
                    .and_modify(|region| region.add_op_child(irony::OpId(id.id())));
            }
        }
        fn with_region<F: for<'a> Fn(&mut Self) -> ()>(
            &mut self,
            parent: irony::RegionId,
            f: F,
        ) {
            self.parent_stack.push(parent);
            f(self);
            self.parent_stack.pop();
        }
        fn get_region_use(&self, region: irony::RegionId) -> Option<irony::OpId> {
            self.op_table
                .iter()
                .find(|tuple| tuple.1.use_region(region))
                .map(|tuple| irony::OpId::from(*tuple.0))
        }
    }
}
