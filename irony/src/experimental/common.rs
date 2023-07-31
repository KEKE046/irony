#[cfg(test)]
mod tests {
    use crate::experimental::hash::FxIndexMap;
    use core::panic;
    use std::fmt::Debug;
    use std::marker::PhantomData;
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

    pub trait ConstantTrait<D> {
        fn dtype(&self) -> D;
    }
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ConstVal<D> {
        value: i32,
        dtyp: D,
    }

    impl<D: Copy> ConstantTrait<D> for ConstVal<D> {
        fn dtype(&self) -> D {
            self.dtyp
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum AttributeEnum {
        ConstVal(ConstVal<DataTypeEnum>)
    }

    impl ConstantTrait<DataTypeEnum> for AttributeEnum {
        fn dtype(&self) -> DataTypeEnum {
            match self {
                AttributeEnum::ConstVal(const_val) => const_val.dtype(),
            }
        }
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
            values: &[&Self::ConstantT],
            uses: &[&UseId],
            defs: &[&DefId],
        ) -> bool
        where
            E: Environ<EntityT = EntityT>,
            EntityT: Entity<DataTypeT = Self::DataTypeT>;
    }

    #[derive(PartialEq, Clone, Copy, Debug)]
    pub struct SameTypeConstraint<ConstantT, DataTypeT> {
        _marker: PhantomData<(ConstantT, DataTypeT)>,
    }

    impl<C: ConstantTrait<D>, D: PartialEq> ConstraintTrait for SameTypeConstraint<C, D> {
        type ConstantT = C;
        type DataTypeT = D;
        fn verify<'env, E, UseId: Id, DefId: Id, EntityT: Entity>(
            &self,
            env: &E,
            values: &[&Self::ConstantT],
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
                .map(|val| Some(val.dtype()))
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

    impl<C, D> SameTypeConstraint<C, D> {
        pub fn into_enum() -> ConstraintEnum {
            ConstraintEnum::SameType(SameTypeConstraint {
                _marker: PhantomData,
            })
        }
    }

    #[derive(PartialEq, Clone, Copy, Debug)]
    pub enum ConstraintEnum {
        SameType(SameTypeConstraint<ConstVal<DataTypeEnum>, DataTypeEnum>),
    }

    impl ConstraintTrait for ConstraintEnum {
        type ConstantT = ConstVal<DataTypeEnum>;
        type DataTypeT = DataTypeEnum;

        fn verify<'env, E, UseId: Id, DefId: Id, EntityT: Entity>(
            &self,
            env: &'env E,
            values: &[&Self::ConstantT],
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

    // pub struct Refered<'rfd, T: ?Sized + Id>(&'rfd T);

    pub trait Environ {
        type OpT: Op; //  = OpEnum;
        type EntityT: Entity; // = EntityEnum;
        type ConstraintT: ConstraintTrait;
        type ConstantT;

        fn get_def<ID: Id>(&self, id: &ID) -> Option<OpId>;
        fn get_uses<ID: Id>(&self, id: &ID) -> Vec<OpId>;
        fn get_entitiy<ID: Id>(&self, id: &ID) -> &Self::EntityT;
        fn get_entities<ID: Id>(&self, ids: &[&ID]) -> Vec<&Self::EntityT>;
        fn add_entity(&mut self, entity: Self::EntityT) -> EntityId;
        fn get_region<ID: Id>(&mut self, id:&ID) -> &Region;
        fn add_region(&mut self, region: Region) -> RegionId;
        fn add_op(&mut self, op: Self::OpT) -> OpId;
        fn set_entity_parent<ID: Id>(&mut self, id: &ID);
        fn set_op_parent<ID: Id>(&mut self, id: &ID);

        fn with_region<F: for<'a> Fn(&mut Self) -> ()>(&mut self, parent: RegionId, f: F);
    }

    pub trait Op: Id {
        type ConstantT;
        type ConstraintT;

        fn get_defs<E: Environ>(&self, env: &E) -> Vec<EntityId>;
        fn get_uses<E: Environ>(&self, env: &E) -> Vec<EntityId>;

        fn get_values(&self) -> Vec<&Self::ConstantT>;

        fn uses<ID: Id>(&self, entity: &ID) -> bool;
        fn defs<ID: Id>(&self, entity: &ID) -> bool;

        fn get_verifiers(&self) -> Vec<Self::ConstraintT>;

        fn get_parent(&self) -> Option<RegionId>;
        fn set_parent(&mut self, parent: RegionId);
    }
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct OpId(usize);

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

    pub trait Entity: Id {
        // type EnumT;
        // type OpEnumT;
        type DataTypeT;
        fn get_dtype(&self) -> Option<Self::DataTypeT>;
        // fn get_def<'env: 't, 't, E>(&'t self, env: &'env E) -> Option<&'t Self::OpEnumT>
        // where
        //     E: Environ<OpT = Self::OpEnumT>;
        // fn get_uses<'env: 't, 't, E: Environ<OpT = Self::OpEnumT>>(
        //     &'t self,
        //     env: &'env E,
        // ) -> Vec<&'t Self::OpEnumT>;

        fn get_def<E: Environ>(&self, env: &E) -> Option<OpId>;
        fn get_uses<E: Environ>(&self, env: &E) -> Vec<OpId>;
        fn as_id(&self) -> EntityId;
        fn get_parent(&self) -> Option<RegionId>;
        fn set_parent(&mut self, parent: RegionId);
    }

    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    pub struct EntityId(usize);
    impl From<usize> for EntityId {
        fn from(value: usize) -> Self {
            Self(value)
        }
    }
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

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct RegionId(usize);
    impl Id for RegionId {
        fn id(&self) -> usize {
            self.0
        }

        fn set_id(&mut self, id: usize) {
            self.0 = id
        }
    }

    impl Region {
        fn get_uses<E: Environ>(&self, env: &E) -> Vec<OpId> {
            env.get_uses(self)
        }

        fn as_id(&self) -> RegionId {
            RegionId(self.id)
        }

        pub fn new() -> Self {
            Self {
                id: 0,
                op_children: vec![],
                entity_children: vec![],
            }
        }

        pub fn add_op_child(&mut self, op: OpId) {
            if let Some(_) = self.op_children.iter().find(|&op_exist| op_exist.id() == op.id()) {
                panic!("{} has already been in the op_children of {}", op.id(), self.id())
            } else {
                self.op_children.push(op)
            }
        }

        pub fn add_entity_child(&mut self, entity: EntityId) {
            if let Some(_) = self.entity_children.iter().find(|&entity_exist| entity_exist.id() == entity.id()) {
                panic!("{} has already been in the entity_children of {}", entity.id(), self.id())
            } else {
                self.entity_children.push(entity)
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Wire {
        id: usize,
        sym: Symbol,
        dtype: DataTypeEnum,
        parent: Option<RegionId>,
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
        type DataTypeT = DataTypeEnum;

        fn get_def<E: Environ>(&self, env: &E) -> Option<OpId> {
            env.get_def(self)
        }

        fn get_uses<E: Environ>(&self, env: &E) -> Vec<OpId> {
            env.get_uses(self)
        }

        fn get_dtype(&self) -> Option<Self::DataTypeT> {
            Some(self.dtype)
        }

        fn as_id(&self) -> EntityId {
            EntityId(self.id)
        }

        fn get_parent(&self) -> Option<RegionId> {
            self.parent
        }

        fn set_parent(&mut self, parent: RegionId) {
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
        type DataTypeT = DataTypeEnum;

        fn get_def<E: Environ>(&self, env: &E) -> Option<OpId> {
            env.get_def(self)
        }

        fn get_uses<E: Environ>(&self, env: &E) -> Vec<OpId> {
            env.get_uses(self)
        }

        fn get_dtype(&self) -> Option<Self::DataTypeT> {
            None
        }

        fn as_id(&self) -> EntityId {
            EntityId(self.id)
        }

        fn get_parent(&self) -> Option<RegionId> {
            None
        }

        fn set_parent(&mut self, parent: RegionId) {
            panic!("Module cannot have parent")
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum EntityEnum {
        Wire(Wire),
        Module(Module),
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

    impl Id for EntityEnum {
        fn id(&self) -> usize {
            match self {
                EntityEnum::Wire(wire) => wire.id(),
                EntityEnum::Module(module) => module.id(),
            }
        }
        fn set_id(&mut self, id: usize) {
            match self {
                EntityEnum::Wire(wire) => wire.set_id(id),
                EntityEnum::Module(module) => module.set_id(id),
            }
        }
    }

    impl Entity for EntityEnum {
        type DataTypeT = DataTypeEnum;

        fn get_dtype(&self) -> Option<Self::DataTypeT> {
            match self {
                EntityEnum::Wire(wire) => wire.get_dtype(),
                EntityEnum::Module(module) => module.get_dtype(),
            }
        }

        fn get_def<E: Environ>(&self, env: &E) -> Option<OpId> {
            match self {
                EntityEnum::Wire(wire) => wire.get_def(env),
                EntityEnum::Module(module) => module.get_def(env),
            }
        }

        fn get_uses<E: Environ>(&self, env: &E) -> Vec<OpId> {
            match self {
                EntityEnum::Wire(wire) => wire.get_uses(env),
                EntityEnum::Module(module) => module.get_uses(env),
            }
        }

        fn as_id(&self) -> EntityId {
            match self {
                EntityEnum::Wire(wire) => wire.as_id(),
                EntityEnum::Module(module) => module.as_id(),
            }
        }

        fn get_parent(&self) -> Option<RegionId> {
            match self {
                EntityEnum::Wire(wire) => wire.get_parent(),
                EntityEnum::Module(module) => module.get_parent(),
            }
        }

        fn set_parent(&mut self, parent: RegionId) {
            match self {
                EntityEnum::Wire(wire) => wire.set_parent(parent),
                EntityEnum::Module(module) => module.set_parent(parent),
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct Assign {
        id: usize,
        lhs: EntityId,
        rhs: EntityId,
        verifiers: Vec<ConstraintEnum>,
        parent: Option<RegionId>,
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
        type ConstraintT = ConstraintEnum;
        type ConstantT = ConstVal<DataTypeEnum>;

        fn get_defs<E: Environ>(&self, env: &E) -> Vec<EntityId> {
            vec![self.lhs]
        }

        fn get_uses<E: Environ>(&self, env: &E) -> Vec<EntityId> {
            vec![self.rhs]
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

        fn get_verifiers(&self) -> Vec<Self::ConstraintT> {
            self.verifiers.to_owned()
        }

        fn get_parent(&self) -> Option<RegionId> {
            self.parent
        }

        fn set_parent(&mut self, parent: RegionId) {
            self.parent = Some(parent)
        }
    }

    impl Assign {
        pub fn new(lhs: EntityId, rhs: EntityId) -> Self {
            Self {
                id: 0,
                lhs,
                rhs,
                verifiers: vec![
                    SameTypeConstraint::<ConstVal<DataTypeEnum>, DataTypeEnum>::into_enum(),
                ],
                parent: None,
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct Constant {
        id: usize,
        lhs: EntityId,
        rhs: ConstVal<DataTypeEnum>,
        verifiers: Vec<ConstraintEnum>,
        parent: Option<RegionId>,
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
        type ConstraintT = ConstraintEnum;

        type ConstantT = ConstVal<DataTypeEnum>;

        fn get_defs<E: Environ>(&self, env: &E) -> Vec<EntityId> {
            vec![self.lhs]
        }

        fn get_uses<E: Environ>(&self, env: &E) -> Vec<EntityId> {
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

        fn get_verifiers(&self) -> Vec<Self::ConstraintT> {
            self.verifiers.to_owned()
        }

        fn get_parent(&self) -> Option<RegionId> {
            self.parent
        }

        fn set_parent(&mut self, parent: RegionId) {
            self.parent = Some(parent)
        }
    }

    impl Constant {
        pub fn new(lhs: EntityId, rhs: ConstVal<DataTypeEnum>) -> Self {
            Self {
                id: 0,
                lhs,
                rhs,
                verifiers: vec![
                    SameTypeConstraint::<ConstVal<DataTypeEnum>, DataTypeEnum>::into_enum(),
                ],
                parent: None,
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct ModuleDef {
        id: usize,
        lhs: EntityId,
        region: RegionId,
        // inputs: EntityId,
        // outputs: EntityId,
        // input_attributes:
        // ouptut_attributes:
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
        type ConstraintT = ConstraintEnum;

        type ConstantT = ConstVal<DataTypeEnum>;

        fn get_defs<E: Environ>(&self, env: &E) -> Vec<EntityId> {
            vec![self.lhs]
        }

        fn get_uses<E: Environ>(&self, env: &E) -> Vec<EntityId> {
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

        fn get_verifiers(&self) -> Vec<Self::ConstraintT> {
            vec![]
        }

        fn get_parent(&self) -> Option<RegionId> {
            None
        }
        fn set_parent(&mut self, parent: RegionId) {
            panic!("ModuleDef cannot have parent")
        }
    }

    impl ModuleDef {
        pub fn new(lhs: EntityId, region: RegionId) -> Self {
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

    impl Into<OpEnum> for Assign {
        fn into(self) -> OpEnum {
            OpEnum::Assign(self)
        }
    }

    impl Into<OpEnum> for Constant {
        fn into(self) -> OpEnum {
            OpEnum::Constant(self)
        }
    }

    impl Into<OpEnum> for ModuleDef {
        fn into(self) -> OpEnum {
            OpEnum::ModuleDef(self)
        }
    }

    impl Op for OpEnum {
        type ConstraintT = ConstraintEnum;

        type ConstantT = ConstVal<DataTypeEnum>;

        fn get_defs<E: Environ>(&self, env: &E) -> Vec<EntityId> {
            match self {
                OpEnum::Assign(assign) => assign.get_defs(env),
                OpEnum::Constant(constant) => constant.get_defs(env),
                OpEnum::ModuleDef(mdef) => mdef.get_defs(env),
            }
        }

        fn get_uses<E: Environ>(&self, env: &E) -> Vec<EntityId> {
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

        fn get_verifiers(&self) -> Vec<Self::ConstraintT> {
            match self {
                OpEnum::Assign(assign) => assign.get_verifiers(),
                OpEnum::Constant(constant) => constant.get_verifiers(),
                OpEnum::ModuleDef(mdef) => mdef.get_verifiers(),
            }
        }

        fn get_parent(&self) -> Option<RegionId> {
            match self {
                OpEnum::Assign(assign) => assign.get_parent(),
                OpEnum::Constant(constant) => constant.get_parent(),
                OpEnum::ModuleDef(mdef) => mdef.get_parent(),
            }
        }

        fn set_parent(&mut self, parent: RegionId) {
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
        region_table: FxMapWithUniqueId<Region>,
        parent_stack: Vec<RegionId>,
    }

    impl Environ for CirctEnv {
        type OpT = OpEnum;

        type EntityT = EntityEnum;

        type ConstraintT = ConstraintEnum;

        type ConstantT = ConstVal<DataTypeEnum>;

        // TODO: Add specific data structure for def-use storage
        fn get_def<ID: Id>(&self, id: &ID) -> Option<OpId> {
            self.op_table
                .iter()
                .find(|tuple| tuple.1.defs(id))
                .map(|tuple| OpId::from(*tuple.0))
        }

        // fn get_uses<'a, 't: 'a, ID: Id>(&'t self, id: &'a ID) -> Vec<&'a Self::OpT> {
        fn get_uses<ID: Id>(&self, id: &ID) -> Vec<OpId> {
            let mut v = Vec::new();
            for (id, op) in self.op_table.get_map() {
                if op.uses(id) {
                    v.push(OpId::from(*id));
                }
            }
            v
        }

        fn get_entitiy<ID: Id>(&self, id: &ID) -> &Self::EntityT {
            match self.entity_table.get(&id.id()) {
                Some(entity) => entity,
                None => panic!(
                    "get entity not in the table by id \ntable: {:#?}\nentity: {:#?}",
                    self.entity_table.get_map(),
                    id.id()
                ),
            }
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
            if let Some(parent) = self.parent_stack.last() {
                self.entity_table
                    .entry(id.id())
                    .and_modify(|entity| entity.set_parent(parent.to_owned()));
                self.region_table.entry(parent.id()).and_modify(|region|
                    region.add_entity_child(EntityId(id.id()))
                );
            }
        }

        fn set_op_parent<ID: Id>(&mut self, id: &ID) {
            if let Some(parent) = self.parent_stack.last() {
                self.op_table
                    .entry(id.id())
                    .and_modify(|entity| entity.set_parent(parent.to_owned()));
                self.region_table.entry(parent.id()).and_modify(|region|
                    region.add_op_child(OpId(id.id()))
                );
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

        fn with_region<F: for<'a> Fn(&mut Self) -> ()>(&mut self, parent: RegionId, f: F) {
            self.parent_stack.push(parent);
            f(self);
            self.parent_stack.pop();
        }

        fn get_region<ID: Id>(&mut self, id:&ID) -> &Region {
            match self.region_table.get(&id.id()) {
                Some(region) => region,
                None => panic!(
                    "get region not in the table by id \ntable: {:#?}\nregion: {:#?}",
                    self.region_table.get_map(),
                    id.id()
                ),
            }
        }

        // TODO: Support nested region
        fn add_region(&mut self, region: Region) -> RegionId {
            let (id, _) = self.region_table.insert_with_id(region);
            RegionId(id)
        }
    }

    impl CirctEnv {}

    #[test]
    fn test_circt_ir() {
        let mut env = CirctEnv::default();
        let module = env.add_entity(Module::new("default").into());
        let region = env.add_region(Region::new().into());
        let module_def = env.add_op(ModuleDef::new(module, region).into());

        env.with_region(region, |env| {
            let child_region = env.add_region(Region::new().into());
            env.with_region(child_region, |env| {
                let wire_grand = env.add_entity(Wire::new("w_grand", DataTypeEnum::Uint(8)).into());
            });
            let wire0 = env.add_entity(Wire::new("w0", DataTypeEnum::Uint(8)).into());
            let wire1 = env.add_entity(Wire::new("w1", DataTypeEnum::Uint(8)).into());
            let constant = env.add_op(
                Constant::new(
                    wire0,
                    ConstVal {
                        value: 1,
                        dtyp: DataTypeEnum::Uint(8),
                    },
                )
                .into(),
            );
            let assign = env.add_op(Assign::new(wire1, wire0).into());
        });

        println!("{:#?}", env);
    }
}

#[cfg(test)]
mod sketch {}
