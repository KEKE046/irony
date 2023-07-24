use std::{collections::HashSet, hash::Hash};


#[derive(Hash,PartialEq,Eq,Clone)]
pub struct I(pub i32);
#[derive(Hash,PartialEq,Eq,Clone)]
pub struct Array<'ctx>(TypeRef<'ctx>, pub i32);
#[derive(Hash,PartialEq,Eq,Clone)]
pub struct Struct<'ctx>(pub Vec<(String,TypeRef<'ctx>)>);

#[derive(Hash,PartialEq,Eq,Clone)]
pub enum Type<'ctx> {
  I(I),
  Array(Array<'ctx>),
  Struct(Struct<'ctx>)
}

#[derive(Clone,Copy)]
pub struct TypeRef<'ctx>(&'ctx Type<'ctx>);
impl<'ctx> TypeRef<'ctx> {
  pub fn new(tpe: &'ctx Type<'ctx>) -> Self { Self(tpe) }
}
impl<'ctx> std::ops::Deref for TypeRef<'ctx> {
  type Target = Type<'ctx>;
  fn deref(&self) -> &Self::Target {return self.0}
}
impl<'ctx> PartialEq<TypeRef<'ctx>> for TypeRef<'ctx> {
  fn eq(&self, other: &TypeRef<'ctx>) -> bool {
    std::ptr::eq(self.0, other.0)
  }
}
impl<'ctx> Eq for TypeRef<'ctx> {}
impl<'ctx> Hash for TypeRef<'ctx> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    std::ptr::hash(self.0, state)
  }
}

pub struct TypeInterner<'ctx> {
  data: HashSet<Box<Type<'ctx>>>
}

impl<'ctx> TypeInterner<'ctx> {
  pub fn get(&'ctx mut self, tpe: Type<'ctx>) -> TypeRef<'ctx> {
    let boxed = Box::new(tpe);
    if ! self.data.contains(&boxed) {
      self.data.insert(boxed.clone());
    }
    TypeRef::new(&**self.data.get(&boxed).unwrap())
  }
}
