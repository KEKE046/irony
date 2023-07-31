use crate::{Region, RegionId};

use super::constraint::ConstraintTrait;
use super::entity::{Entity, EntityId};
use super::operation::{Op, OpId};


pub trait Environ {
    type OpT: Op;
    type EntityT: Entity;
    type ConstraintT: ConstraintTrait;
    type AttributeT;

    fn get_def(&self, id: EntityId) -> Option<OpId>;
    fn get_uses(&self, id: EntityId) -> Vec<OpId>;
    fn get_entity(&self, id: EntityId) -> &Self::EntityT;
    fn get_entities(&self, ids: &[EntityId]) -> Vec<&Self::EntityT>;
    fn get_op(&self, id: OpId) -> &Self::OpT;
    fn get_ops(&self, ids: &[OpId]) -> Vec<&Self::OpT>;
    fn add_entity(&mut self, entity: Self::EntityT) -> EntityId;
    fn get_region(&self, id: RegionId) -> &Region;
    fn add_region(&mut self, region: Region) -> RegionId;
    fn add_op(&mut self, op: Self::OpT) -> OpId;
    fn set_entity_parent(&mut self, id: EntityId);
    fn set_op_parent(&mut self, id: OpId);
    fn get_region_use(&self, region: RegionId) -> Option<OpId>;

    fn with_region<F: for<'a> Fn(&mut Self) -> ()>(&mut self, parent: RegionId, f: F);
}

#[macro_export]
macro_rules! environ_def {
    (
        [entity = $entity_ty:ty, op = $op_ty:ty, attr = $attr_ty:ty, constraint = $constraint_ty:ty]
        struct $name:ident;
    ) => {
        irony::environ_def! {
            @inner
            ENTITY: $entity_ty;
            OP: $op_ty;
            ATTR: $attr_ty;
            CONSTRAINT: $constraint_ty;
            NAME: $name;
            FIELDS: ;
        }
    };

    (
        [entity = $entity_ty:ty, op = $op_ty:ty, attr = $attr_ty:ty, constraint = $constraint_ty:ty]
        struct $name:ident {
            $(
                $field_vis:vis $field_name:ident : $field_ty:ty
            ),*
        }
    ) => {
        irony::environ_def! {
            @inner
            ENTITY: $entity_ty;
            OP: $op_ty;
            ATTR: $attr_ty;
            CONSTRAINT: $constraint_ty;
            NAME: $name;
            FIELDS: $($field_vis $field_name : $field_ty)*;
        }
    };

    (@inner
        ENTITY: $entity_ty:ty;
        OP: $op_ty:ty;
        ATTR: $attr_ty:ty;
        CONSTRAINT: $constraint_ty:ty;
        NAME: $name:ident ;
        FIELDS: $($field_vis:vis $field_name:ident : $field_ty:ty)* ;
    ) => {

        #[derive(Default, Debug)]
        pub struct $name {
            op_table: irony::FxMapWithUniqueId<$op_ty>,
            entity_table: irony::FxMapWithUniqueId<$entity_ty>,
            region_table: irony::FxMapWithUniqueId<irony::Region>,
            parent_stack: Vec<irony::RegionId>,

            $($field_vis $field_name: $field_ty,)*
        }

        impl irony::Environ for $name {
            type OpT = $op_ty;
        
            type EntityT = $entity_ty;
        
            type ConstraintT = $constraint_ty;
        
            type AttributeT = $attr_ty;
        
            fn get_def(&self, entity: irony::EntityId) -> Option<irony::OpId> {
                self.op_table
                .iter()
                .find(|tuple| tuple.1.defs(entity))
                .map(|tuple| irony::OpId::from(*tuple.0))
            }
        
            fn get_uses(&self, entity: irony::EntityId) -> Vec<irony::OpId> {
                println!("try to run get_uses() for entity {:?}\n", entity);
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
                    None => panic!(
                        "get entity not in the table by id \ntable: {:#?}\nentity: {:#?}",
                        self.entity_table.get_map(),
                        id.id()
                    ),
                }
            }
        
            fn get_entities(&self, ids: &[irony::EntityId]) -> Vec<&Self::EntityT> {
                ids.iter()
                .map(|id| self.get_entity(id.to_owned()))
                .collect()
            }

            fn get_op(&self, id: irony::OpId) -> &Self::OpT {
                match self.op_table.get(&id.id()) {
                    Some(op) =>op,
                    None => panic!(
                        "get op not in the table by id \ntable: {:#?}\nop: {:#?}",
                        self.op_table.get_map(),
                        id.id()
                    ),
                }
            }
            fn get_ops(&self, ids: &[irony::OpId]) -> Vec<&Self::OpT> {
                ids.iter()
                .map(|id| self.get_op(id.to_owned()))
                .collect()
            }
        
            fn add_entity(&mut self, entity: Self::EntityT) -> irony::EntityId {
                let (id, _) = self.entity_table.insert_with_id(entity);
                self.set_entity_parent(irony::EntityId::from(id));
                irony::EntityId(id)
            }
        
            fn get_region(&self, id: irony::RegionId) -> &irony::Region {
                match self.region_table.get(&id.id()) {
                    Some(region) => region,
                    None => panic!(
                        "get region not in the table by id \ntable: {:#?}\nregion: {:#?}",
                        self.region_table.get_map(),
                        id.id()
                    ),
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
                    self.region_table.entry(parent.id()).and_modify(|region|
                        region.add_entity_child(irony::EntityId(id.id()))
                    );
                }
            }
        
            fn set_op_parent(&mut self, id: irony::OpId) {
                if let Some(parent) = self.parent_stack.last() {
                    self.op_table
                        .entry(id.id())
                        .and_modify(|entity| entity.set_parent(parent.to_owned()));
                    self.region_table.entry(parent.id()).and_modify(|region|
                        region.add_op_child(irony::OpId(id.id()))
                    );
                }
            }
        
            fn with_region<F: for<'a> Fn(&mut Self) -> ()>(&mut self, parent: irony::RegionId, f: F) {
                self.parent_stack.push(parent);
                f(self);
                self.parent_stack.pop();
            }

            fn get_region_use(&self, region: irony::RegionId) -> Option<irony::OpId> {
                self.op_table.iter().find(|tuple| tuple.1.use_region(region))
                .map(|tuple| irony::OpId::from(*tuple.0))
            }
        }


    };
}