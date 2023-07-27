#[cfg(test)]
mod tests {
    use crate::experimental::hash::FxIndexMap;
    use core::panic;
    use std::fmt::Debug;
    use std::num::ParseIntError;
    use std::ops::{Deref, DerefMut};

    #[derive(Clone, Debug, PartialEq)]
    pub struct Symbol {
        name: Option<String>,
    }

    impl Symbol {
        pub fn new(name: String) -> Self {
            Self { name: Some(name) }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ConstVal {
        value: i32,
        dtyp: DataTypeEnum,
    }

    // pub struct DataType;
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum DataTypeEnum {
        Uint(usize),
    }

    pub trait Id {
        fn id(&self) -> usize;
        fn set_id(&mut self, id: usize);
    }

    impl Id for usize {
        fn id(&self) -> usize {
            *self
        }
        fn set_id(&mut self, id: usize) {
            panic!("cannot set id to usize")
        }
    }

    pub trait ConstraintTrait {
        type DataTypeT;
        type ConstantT;
        fn verify<'env, E, UseId: Id, DefId: Id, EntityT: Entity>(
            &self,
            env: &'env E,
            values: &[&ConstVal],
            uses: &[&UseId],
            defs: &[&DefId],
        ) -> bool
        where
            E: Environ<EntityT = EntityT>,
            EntityT: Entity<DataTypeT = Self::DataTypeT>;
    }

    #[derive(PartialEq, Clone, Copy, Debug)]
    pub struct SameTypeConstraint;

    impl ConstraintTrait for SameTypeConstraint {
        type ConstantT = ConstVal;
        type DataTypeT = DataTypeEnum;
        fn verify<'env, E, UseId: Id, DefId: Id, EntityT: Entity>(
            &self,
            env: &E,
            values: &[&ConstVal],
            uses: &[&UseId],
            defs: &[&DefId],
        ) -> bool
        where
            E: Environ<EntityT = EntityT>,
            EntityT: Entity<DataTypeT = Self::DataTypeT>,
        {
            // <<EntityT as Entity<DataTypeT = DataTypeEnum>>, {
            let mut dtype_collect = values
                .iter()
                .map(|val| Some(val.dtyp))
                .chain(
                    env.get_entities(uses)
                        .into_iter()
                        .map(|entity| entity.get_dtype()),
                )
                .chain(
                    env.get_entities(defs)
                        .into_iter()
                        .map(|entity| entity.get_dtype()),
                );

            if let Some(first) = dtype_collect.next() {
                dtype_collect.all(|item| item == first)
            } else {
                true
            }
        }
    }

    impl SameTypeConstraint {
        pub fn into_enum() -> ConstraintEnum {
            ConstraintEnum::SameType(SameTypeConstraint)
        }
    }

    #[derive(PartialEq, Clone, Copy, Debug)]
    pub enum ConstraintEnum {
        SameType(SameTypeConstraint),
    }

    impl ConstraintTrait for ConstraintEnum {
        type ConstantT = ConstVal;
        type DataTypeT = DataTypeEnum;

        fn verify<'env, E, UseId: Id, DefId: Id, EntityT: Entity>(
            &self,
            env: &'env E,
            values: &[&ConstVal],
            uses: &[&UseId],
            defs: &[&DefId],
        ) -> bool
        where
            E: Environ<EntityT = EntityT>,
            EntityT: Entity<DataTypeT = Self::DataTypeT>,
        {
            match self {
                ConstraintEnum::SameType(same_type) => same_type.verify(env, values, uses, defs),
            }
        }
    }

    pub struct Refered<'rfd, T: ?Sized + Id>(&'rfd T);

    pub trait Environ {
        type OpT: Op; //  = OpEnum;
        type EntityT: Entity; // = EntityEnum;
        type ConstraintT: ConstraintTrait;
        type ConstantT;

        fn get_def<'a, 't: 'a, ID: Id>(&'t self, id: &'a ID) -> Option<&'a Self::OpT>;
        fn get_uses<'a, 't: 'a, ID: Id>(&'t self, id: &'a ID) -> Vec<&'a Self::OpT>;
        fn get_entities<ID: Id>(&self, ids: &[&ID]) -> Vec<&Self::EntityT>;
        fn add_entity(&mut self, entity: Self::EntityT) -> EntityId;
        fn add_op(&mut self, op: Self::OpT) -> OpId;
        fn set_entity_parent<ID: Id>(&mut self, id: &ID);
        fn set_op_parent<ID: Id>(&mut self, id: &ID);

        fn with_region<F: for<'a> Fn(&mut Self) -> () >(&mut self, parent:EntityId, f: F);
    }

    pub trait Op: Id {
        type EntityT;
        type OpT;
        type ConstraintT;
        type ConstantT;

        fn get_defs<'env: 't, 't, E: Environ<EntityT = Self::EntityT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::EntityT>;
        fn get_uses<'env: 't, 't, E: Environ<EntityT = Self::EntityT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::EntityT>;
        fn get_values(&self) -> Vec<&Self::ConstantT>;

        fn uses<ID: Id>(&self, entity: &ID) -> bool;
        fn defs<ID: Id>(&self, entity: &ID) -> bool;

        fn verify<
            'env,
            E: Environ<
                EntityT = Self::EntityT,
                ConstraintT = Self::ConstraintT,
                ConstantT = Self::ConstantT,
            >,
        >(
            &self,
            env: &'env E,
        ) -> bool;

        fn into_enum(self) -> Self::OpT;

        fn get_parent(&self) -> Option<EntityId>;
        fn set_parent(&mut self, parent: EntityId);
    }
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct OpId(usize);
    impl Id for OpId {
        fn id(&self) -> usize {
            self.0
        }
        fn set_id(&mut self, id: usize) {
            self.0 = id
        }
    }

    pub trait Entity: Id {
        type EnumT;
        type OpEnumT;
        type DataTypeT;
        fn get_dtype(&self) -> Option<Self::DataTypeT>;
        fn get_def<'env: 't, 't, E>(&'t self, env: &'env E) -> Option<&'t Self::OpEnumT>
        where
            E: Environ<OpT = Self::OpEnumT>;
        fn get_uses<'env: 't, 't, E: Environ<OpT = Self::OpEnumT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::OpEnumT>;
        fn into_enum(self) -> Self::EnumT;
        fn as_enum(&self) -> Self::EnumT;
        fn as_id(&self) -> EntityId;
        fn get_parent(&self) -> Option<EntityId>;
        fn set_parent(&mut self, parent: EntityId);
    }

    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    pub struct EntityId(usize);
    impl Id for EntityId {
        fn id(&self) -> usize {
            self.0
        }
        fn set_id(&mut self, id: usize) {
            self.0 = id
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Region {
        id: usize,
        op_children: Vec<OpId>,
        entity_children: Vec<EntityId>,
    }

    impl Id for Region {
        fn id(&self) -> usize {
            self.id
        }
        fn set_id(&mut self, id: usize) {
            self.id = id
        }
    }

    impl Entity for Region {
        type EnumT = EntityEnum;

        type OpEnumT = OpEnum;

        type DataTypeT = DataTypeEnum;

        fn get_dtype(&self) -> Option<Self::DataTypeT> {
            None
        }

        fn get_def<'env: 't, 't, E>(&'t self, env: &'env E) -> Option<&'t Self::OpEnumT>
        where
            E: Environ<OpT = Self::OpEnumT>,
        {
            None
        }

        fn get_uses<'env: 't, 't, E: Environ<OpT = Self::OpEnumT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::OpEnumT> {
            env.get_uses(self)
        }

        fn into_enum(self) -> Self::EnumT {
            Self::EnumT::Region(self)
        }

        fn as_enum(&self) -> Self::EnumT {
            todo!()
        }

        fn as_id(&self) -> EntityId {
            EntityId(self.id)
        }

        // Region does not have parent. It should be used be operation to represent hierarchy
        fn get_parent(&self) -> Option<EntityId> {
            None
        }

        fn set_parent(&mut self, parent: EntityId) {
            panic!("Region cannot have parent")
        }
    }

    impl Region {
        pub fn new() -> Self {
            Self {
                id: 0,
                op_children: vec![],
                entity_children: vec![],
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Wire {
        id: usize,
        sym: Symbol,
        dtype: DataTypeEnum,
        parent: Option<EntityId>,
    }

    impl Id for Wire {
        fn id(&self) -> usize {
            self.id
        }
        fn set_id(&mut self, id: usize) {
            self.id = id
        }
    }

    impl Entity for Wire {
        type EnumT = EntityEnum;
        type OpEnumT = OpEnum;
        type DataTypeT = DataTypeEnum;

        fn get_def<'env: 't, 't, E>(&'t self, env: &'env E) -> Option<&'t Self::OpEnumT>
        where
            E: Environ<OpT = Self::OpEnumT>,
        {
            env.get_def(self)
        }

        fn get_uses<'env: 't, 't, E: Environ<OpT = Self::OpEnumT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::OpEnumT> {
            env.get_uses(self)
        }

        fn into_enum(self) -> Self::EnumT {
            Self::EnumT::Wire(self)
        }

        fn as_enum(&self) -> Self::EnumT {
            Self::EnumT::Wire(self.to_owned())
        }

        fn get_dtype(&self) -> Option<Self::DataTypeT> {
            Some(self.dtype)
        }

        fn as_id(&self) -> EntityId {
            EntityId(self.id)
        }

        fn get_parent(&self) -> Option<EntityId> {
            self.parent
        }

        fn set_parent(&mut self, parent: EntityId) {
            self.parent = Some(parent)
        }
    }

    impl Wire {
        pub fn new(name: &str, dtype: DataTypeEnum) -> Self {
            Self {
                id: 0,
                sym: Symbol::new(String::from(name)),
                dtype,
                parent: None,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Module {
        id: usize,
        sym: Symbol,
        parent: Option<EntityId>,
    }

    impl Id for Module {
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
                sym: Symbol::new(String::from(name)),
                parent: None,
            }
        }
    }

    impl Entity for Module {
        type EnumT = EntityEnum;
        type OpEnumT = OpEnum;
        type DataTypeT = DataTypeEnum;

        fn get_def<'env: 't, 't, E>(&'t self, env: &'env E) -> Option<&'t Self::OpEnumT>
        where
            E: Environ<OpT = Self::OpEnumT>,
        {
            env.get_def(self)
        }
        fn get_uses<'env: 't, 't, E: Environ<OpT = Self::OpEnumT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::OpEnumT> {
            env.get_uses(self)
        }

        fn into_enum(self) -> Self::EnumT {
            Self::EnumT::Module(self)
        }

        fn as_enum(&self) -> Self::EnumT {
            Self::EnumT::Module(self.to_owned())
        }

        fn get_dtype(&self) -> Option<Self::DataTypeT> {
            None
        }

        fn as_id(&self) -> EntityId {
            EntityId(self.id)
        }

        fn get_parent(&self) -> Option<EntityId> {
            None
        }

        fn set_parent(&mut self, parent: EntityId) {
            panic!("Module cannot have parent")
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum EntityEnum {
        Wire(Wire),
        Module(Module),
        Region(Region),
    }

    impl Id for EntityEnum {
        fn id(&self) -> usize {
            match self {
                EntityEnum::Wire(wire) => wire.id(),
                EntityEnum::Module(module) => module.id(),
                EntityEnum::Region(region) => region.id(),
            }
        }
        fn set_id(&mut self, id: usize) {
            match self {
                EntityEnum::Wire(wire) => wire.set_id(id),
                EntityEnum::Module(module) => module.set_id(id),
                EntityEnum::Region(region) => region.set_id(id),
            }
        }
    }

    impl Entity for EntityEnum {
        type EnumT = EntityEnum;

        type OpEnumT = OpEnum;

        type DataTypeT = DataTypeEnum;

        fn get_dtype(&self) -> Option<Self::DataTypeT> {
            match self {
                EntityEnum::Wire(wire) => wire.get_dtype(),
                EntityEnum::Module(module) => module.get_dtype(),
                EntityEnum::Region(region) => region.get_dtype(),
            }
        }

        fn get_def<'env: 't, 't, E>(&'t self, env: &'env E) -> Option<&'t Self::OpEnumT>
        where
            E: Environ<OpT = Self::OpEnumT>,
        {
            match self {
                EntityEnum::Wire(wire) => wire.get_def(env),
                EntityEnum::Module(module) => module.get_def(env),
                EntityEnum::Region(region) => region.get_def(env),
            }
        }

        fn get_uses<'env: 't, 't, E: Environ<OpT = Self::OpEnumT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::OpEnumT> {
            match self {
                EntityEnum::Wire(wire) => wire.get_uses(env),
                EntityEnum::Module(module) => module.get_uses(env),
                EntityEnum::Region(region) => region.get_uses(env),
            }
        }

        fn into_enum(self) -> Self::EnumT {
            match self {
                EntityEnum::Wire(wire) => wire.into_enum(),
                EntityEnum::Module(module) => module.into_enum(),
                EntityEnum::Region(region) => region.into_enum(),
            }
        }

        fn as_enum(&self) -> Self::EnumT {
            match self {
                EntityEnum::Wire(wire) => wire.as_enum(),
                EntityEnum::Module(module) => module.as_enum(),
                EntityEnum::Region(region) => region.as_enum(),
            }
        }

        fn as_id(&self) -> EntityId {
            match self {
                EntityEnum::Wire(wire) => wire.as_id(),
                EntityEnum::Module(module) => module.as_id(),
                EntityEnum::Region(region) => region.as_id(),
            }
        }

        fn get_parent(&self) -> Option<EntityId> {
            match self {
                EntityEnum::Wire(wire) => wire.get_parent(),
                EntityEnum::Module(module) => module.get_parent(),
                EntityEnum::Region(region) => region.get_parent(),
            }
        }

        fn set_parent(&mut self, parent: EntityId) {
            match self {
                EntityEnum::Wire(wire) => wire.set_parent(parent),
                EntityEnum::Module(module) => module.set_parent(parent),
                EntityEnum::Region(region) => region.set_parent(parent),
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct Assign {
        id: usize,
        lhs: EntityId,
        rhs: EntityId,
        verifiers: Vec<ConstraintEnum>,
        parent: Option<EntityId>,
    }

    impl Id for Assign {
        fn id(&self) -> usize {
            self.id
        }

        fn set_id(&mut self, id: usize) {
            self.id = id
        }
    }

    impl Op for Assign {
        type EntityT = EntityEnum;
        type OpT = OpEnum;
        type ConstraintT = ConstraintEnum;
        type ConstantT = ConstVal;

        fn get_defs<'env: 't, 't, E>(&'t self, env: &'env E) -> Vec<&'t Self::EntityT>
        where
            E: Environ<EntityT = Self::EntityT>,
        {
            env.get_entities(&[&self.lhs])
        }

        fn get_uses<'env: 't, 't, E: Environ<EntityT = Self::EntityT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::EntityT> {
            env.get_entities(&[&self.rhs])
        }

        fn get_values(&self) -> Vec<&Self::ConstantT> {
            vec![]
        }

        fn uses<ID: Id>(&self, entity: &ID) -> bool {
            self.rhs.id() == entity.id()
        }
        fn defs<ID: Id>(&self, entity: &ID) -> bool {
            self.lhs.id() == entity.id()
        }

        fn verify<
            'env,
            E: Environ<
                EntityT = Self::EntityT,
                ConstraintT = Self::ConstraintT,
                ConstantT = Self::ConstantT,
            >,
        >(
            &self,
            env: &'env E,
        ) -> bool {
            self.verifiers
                .iter()
                .map(|verifier| {
                    verifier.verify(
                        env,
                        self.get_values().as_slice(),
                        self.get_uses(env).as_slice(),
                        self.get_defs(env).as_slice(),
                    )
                })
                .fold(true, |acc, x| acc && x)
        }

        fn into_enum(self) -> Self::OpT {
            Self::OpT::Assign(self)
        }

        fn get_parent(&self) -> Option<EntityId> {
            self.parent
        }

        fn set_parent(&mut self, parent: EntityId) {
            self.parent = Some(parent)
        }
    }


    impl Assign {
        pub fn new(lhs: EntityId, rhs:EntityId) -> Self {
            Self { id: 0, lhs, rhs, verifiers: vec![SameTypeConstraint::into_enum()], parent:None }
        }
        
    }


    #[derive(PartialEq, Debug)]
    pub struct Constant {
        id: usize,
        lhs: EntityId,
        rhs: ConstVal,
        verifiers: Vec<ConstraintEnum>,
        parent: Option<EntityId>,
    }

    impl Id for Constant {
        fn id(&self) -> usize {
            self.id
        }

        fn set_id(&mut self, id: usize) {
            self.id = id;
        }
    }

    impl Op for Constant {
        type EntityT = EntityEnum;
        type OpT = OpEnum;

        type ConstraintT = ConstraintEnum;

        type ConstantT = ConstVal;

        fn get_defs<'env: 't, 't, E: Environ<EntityT = Self::EntityT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::EntityT> {
            env.get_entities(&[&self.lhs])
        }
        fn get_uses<'env: 't, 't, E: Environ<EntityT = Self::EntityT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::EntityT> {
            vec![]
        }

        fn get_values(&self) -> Vec<&Self::ConstantT> {
            vec![&self.rhs]
        }

        fn uses<ID: Id>(&self, entity: &ID) -> bool {
            false
        }
        fn defs<ID: Id>(&self, entity: &ID) -> bool {
            self.lhs.id() == entity.id()
        }

        fn verify<
            'env,
            E: Environ<
                EntityT = Self::EntityT,
                ConstraintT = Self::ConstraintT,
                ConstantT = Self::ConstantT,
            >,
        >(
            &self,
            env: &'env E,
        ) -> bool {
            self.verifiers
                .iter()
                .map(|verifier| {
                    verifier.verify(
                        env,
                        self.get_values().as_slice(),
                        self.get_uses(env).as_slice(),
                        self.get_defs(env).as_slice(),
                    )
                })
                .fold(true, |acc, x| acc && x)
        }

        fn into_enum(self) -> Self::OpT {
            Self::OpT::Constant(self)
        }

        fn get_parent(&self) -> Option<EntityId> {
            self.parent
        }

        fn set_parent(&mut self, parent: EntityId) {
            self.parent = Some(parent)
        }
    }


    impl Constant {
        pub fn new(lhs: EntityId, rhs: ConstVal) -> Self{
            Self {
                id: 0,
                lhs,
                rhs,
                verifiers: vec![SameTypeConstraint::into_enum()],
                parent: None,
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct ModuleDef {
        id: usize,
        lhs: EntityId,
        region: EntityId,
        verifiers: Vec<ConstraintEnum>,
        parent: Option<EntityId>,
    }

    impl Id for ModuleDef {
        fn id(&self) -> usize {
            self.id
        }

        fn set_id(&mut self, id: usize) {
            self.id = id
        }
    }

    impl Op for ModuleDef {
        type EntityT = EntityEnum;
        type OpT = OpEnum;

        type ConstraintT = ConstraintEnum;

        type ConstantT = ConstVal;

        fn get_defs<'env: 't, 't, E: Environ<EntityT = Self::EntityT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::EntityT> {
            env.get_entities(&[&self.lhs])
        }

        fn get_uses<'env: 't, 't, E: Environ<EntityT = Self::EntityT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::EntityT> {
            vec![]
        }

        fn get_values(&self) -> Vec<&Self::ConstantT> {
            vec![]
        }

        fn uses<ID: Id>(&self, entity: &ID) -> bool {
            false
        }

        fn defs<ID: Id>(&self, entity: &ID) -> bool {
            self.lhs.id() == entity.id()
        }

        fn verify<
            'env,
            E: Environ<
                EntityT = Self::EntityT,
                ConstraintT = Self::ConstraintT,
                ConstantT = Self::ConstantT,
            >,
        >(
            &self,
            env: &'env E,
        ) -> bool {
            true
        }

        fn into_enum(self) -> Self::OpT {
            Self::OpT::ModuleDef(self)
        }
        fn get_parent(&self) -> Option<EntityId> {
            None
        }
        fn set_parent(&mut self, parent: EntityId) {
            panic!("ModuleDef cannot have parent")
        }
    }

    impl ModuleDef {
        pub fn new(lhs: EntityId, region: EntityId) -> Self {
            Self {
                id: 0,
                lhs,
                region,
                verifiers: vec![],
                parent: None,
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub enum OpEnum {
        Assign(Assign),
        Constant(Constant),
        ModuleDef(ModuleDef),
    }

    impl Op for OpEnum {
        type EntityT = EntityEnum;
        type OpT = OpEnum;
        type ConstraintT = ConstraintEnum;

        type ConstantT = ConstVal;

        fn get_defs<'env: 't, 't, E: Environ<EntityT = Self::EntityT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::EntityT> {
            match self {
                OpEnum::Assign(assign) => assign.get_defs(env),
                OpEnum::Constant(constant) => constant.get_defs(env),
                OpEnum::ModuleDef(mdef) => mdef.get_defs(env),
            }
        }

        fn get_uses<'env: 't, 't, E: Environ<EntityT = Self::EntityT>>(
            &'t self,
            env: &'env E,
        ) -> Vec<&'t Self::EntityT> {
            match self {
                OpEnum::Assign(assign) => assign.get_uses(env),
                OpEnum::Constant(constant) => constant.get_uses(env),
                OpEnum::ModuleDef(mdef) => mdef.get_uses(env),
            }
        }

        fn get_values(&self) -> Vec<&Self::ConstantT> {
            match self {
                OpEnum::Assign(assign) => assign.get_values(),
                OpEnum::Constant(constant) => constant.get_values(),
                OpEnum::ModuleDef(mdef) => mdef.get_values(),
            }
        }

        fn uses<ID: Id>(&self, entity: &ID) -> bool {
            match self {
                OpEnum::Assign(assign) => assign.uses(entity),
                OpEnum::Constant(constant) => constant.uses(entity),
                OpEnum::ModuleDef(mdef) => mdef.uses(entity),
            }
        }
        fn defs<ID: Id>(&self, entity: &ID) -> bool {
            match self {
                OpEnum::Assign(assign) => assign.defs(entity),
                OpEnum::Constant(constant) => constant.defs(entity),
                OpEnum::ModuleDef(mdef) => mdef.defs(entity),
            }
        }

        fn verify<
            'env,
            E: Environ<
                EntityT = Self::EntityT,
                ConstraintT = Self::ConstraintT,
                ConstantT = Self::ConstantT,
            >,
        >(
            &self,
            env: &'env E,
        ) -> bool {
            match self {
                OpEnum::Assign(assign) => assign.verify(env),
                OpEnum::Constant(constant) => constant.verify(env),
                OpEnum::ModuleDef(mdef) => mdef.verify(env),
            }
        }

        fn get_parent(&self) -> Option<EntityId> {
            match self {
                OpEnum::Assign(assign) => assign.get_parent(),
                OpEnum::Constant(constant) => constant.get_parent(),
                OpEnum::ModuleDef(mdef) => mdef.get_parent(),
            }
        }

        fn into_enum(self) -> Self::OpT {
            match self {
                OpEnum::Assign(assign) => assign.into_enum(),
                OpEnum::Constant(constant) => constant.into_enum(),
                OpEnum::ModuleDef(mdef) => mdef.into_enum(),
            }
        }

        fn set_parent(&mut self, parent: EntityId) {
            match self {
                OpEnum::Assign(assign) => assign.set_parent(parent),
                OpEnum::Constant(constant) => constant.set_parent(parent),
                OpEnum::ModuleDef(mdef) => mdef.set_parent(parent),
            }
        }
    }

    impl Id for OpEnum {
        fn id(&self) -> usize {
            match self {
                OpEnum::Assign(assign) => assign.id(),
                OpEnum::Constant(constant) => constant.id(),
                OpEnum::ModuleDef(mdef) => mdef.id(),
            }
        }
        fn set_id(&mut self, id: usize) {
            match self {
                OpEnum::Assign(assign) => assign.set_id(id),
                OpEnum::Constant(constant) => constant.set_id(id),
                OpEnum::ModuleDef(mdef) => mdef.set_id(id),
            }
        }
    }

    #[derive(Debug)]
    pub struct FxMapWithUniqueId<V> {
        indexmap: FxIndexMap<usize, V>,
        next_id: usize,
    }

    impl<V> Default for FxMapWithUniqueId<V> {
        fn default() -> Self {
            Self {
                indexmap: Default::default(),
                next_id: Default::default(),
            }
        }
    }

    impl<V> Deref for FxMapWithUniqueId<V> {
        type Target = FxIndexMap<usize, V>;

        fn deref(&self) -> &Self::Target {
            &self.indexmap
        }
    }

    impl<V> DerefMut for FxMapWithUniqueId<V> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.indexmap
        }
    }

    impl<V> FxMapWithUniqueId<V>
    where
        V: PartialEq + Debug + Id,
    {
        pub fn get_map(&self) -> &FxIndexMap<usize, V> {
            &self.indexmap
        }
        pub fn get_map_mut(&mut self) -> &mut FxIndexMap<usize, V> {
            &mut self.indexmap
        }

        pub fn insert_with_id<'a, 't: 'a>(&'t mut self, mut value: V) -> (usize, &'a V) {
            let cur_id = self.next_id;
            self.next_id += 1;

            value.set_id(cur_id);

            let option = self.indexmap.insert(cur_id, value);
            assert_eq!(option, None);

            (cur_id, self.indexmap.get(&cur_id).unwrap())
        }
    }

    #[derive(Default, Debug)]
    struct CirctEnv {
        op_table: FxMapWithUniqueId<OpEnum>,
        entity_table: FxMapWithUniqueId<EntityEnum>,
        parent: Option<EntityId>,
    }

    impl Environ for CirctEnv {
        type OpT = OpEnum;

        type EntityT = EntityEnum;

        type ConstraintT = ConstraintEnum;

        type ConstantT = ConstVal;

        // TODO: Add specific data structure for def-use storage
        fn get_def<'a, 't: 'a, ID: Id>(&'t self, id: &'a ID) -> Option<&'a Self::OpT> {
            self.op_table
                .iter()
                .find(|tuple| tuple.1.defs(id))
                .map(|tuple| tuple.1)
        }

        fn get_uses<'a, 't: 'a, ID: Id>(&'t self, id: &'a ID) -> Vec<&'a Self::OpT> {
            let mut v = Vec::new();
            for (_, op) in self.op_table.get_map() {
                if op.uses(id) {
                    v.push(op);
                }
            }
            v
        }

        fn get_entities<ID: Id>(&self, ids: &[&ID]) -> Vec<&Self::EntityT> {
            ids.iter()
                .map(|id| match self.entity_table.get(&id.id()) {
                    Some(entity) => entity,
                    None => panic!(
                        "get entity not in the table by id \ntable: {:#?}\nentity: {:#?}",
                        self.entity_table.get_map(),
                        id.id()
                    ),
                })
                .collect()
        }

        fn set_entity_parent<ID: Id>(&mut self, id: &ID) {
            if let Some(parent) = self.parent {
                self.entity_table
                    .entry(id.id())
                    .and_modify(|entity| entity.set_parent(parent));
            }
        }

        fn set_op_parent<ID: Id>(&mut self, id: &ID) {
            if let Some(parent) = self.parent {
                self.op_table
                    .entry(id.id())
                    .and_modify(|entity| entity.set_parent(parent));
            }
        }

        fn add_entity(&mut self, entity: Self::EntityT) -> EntityId {
            let (id, _) = self.entity_table.insert_with_id(entity);
            self.set_entity_parent(&id);
            EntityId(id)
        }
        fn add_op(&mut self, op: Self::OpT) -> OpId {
            let (id, op) = self.op_table.insert_with_id(op);
            self.set_op_parent(&id);
            OpId(id)
        }

        fn with_region<F: for<'a> Fn(&mut Self) -> () >(&mut self, parent:EntityId, f: F) {
            self.parent = Some(parent);
            f(self);
            self.parent = None;
        }
    }

    impl CirctEnv {}

    #[test]
    fn test_circt_ir() {
        let mut env = CirctEnv::default();
        let module = env.add_entity(Module::new("default").into_enum());
        let region = env.add_entity(Region::new().into_enum());
        let module_def = env.add_op(ModuleDef::new(module, region).into_enum());

        env.with_region(region, |env| {
            let wire0 = env.add_entity(Wire::new("w0", DataTypeEnum::Uint(8)).into_enum());
            let wire1 = env.add_entity(Wire::new("w1", DataTypeEnum::Uint(8)).into_enum());
            let constant = env.add_op(Constant::new(wire0, ConstVal { value: 1, dtyp: DataTypeEnum::Uint(8) }).into_enum());
            let assign = env.add_op(Assign::new(wire1, wire0).into_enum());
        });

        println!("{:#?}", env);
    }
}

#[cfg(test)]
mod sketch {}
