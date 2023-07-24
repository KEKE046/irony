use std::{rc::Rc, slice, ops::{Deref, DerefMut}, hash::Hash};

use super::types::TypeRef;

pub struct Value<'ctx> {
  tpe: TypeRef<'ctx>
}

#[derive(Clone,Copy)]
pub struct ValueRef<'ctx>(&'ctx Value<'ctx>);
impl<'ctx> ValueRef<'ctx> {
  fn new(value: &'op Value<'ctx>) -> Self {Self(value)}
}
impl<'ctx> PartialEq for ValueRef<'ctx> {
  fn eq(&self, other: &Self) -> bool { std::ptr::eq(self.0, other.0) }
}
impl<'ctx> Eq for ValueRef<'ctx> {}
impl<'ctx> Deref for ValueRef<'ctx> {
  type Target = Value<'ctx>;
  fn deref(&self) -> &Self::Target {&self.0}
}
impl<'ctx> Hash for ValueRef<'ctx> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    std::ptr::hash(self.0, state)
  }
}

pub fn type_iter<'ctx, I>(val_iter: I) -> impl Iterator<Item=TypeRef<'ctx>> 
where I: Iterator<Item=&'ctx ValueRef<'ctx>> {
  val_iter.map(|x|{x.tpe})
}

struct GenericOp<'ctx> {
  operands: Vec<ValueRef<'ctx>>,
  results: Vec<ValueRef<'ctx>>,
}

pub trait Op<'ctx> {
  fn get_results(&'ctx self) -> impl Iterator<Item=&ValueRef<'ctx>>;
  fn get_operands_mut(&'ctx mut self) -> impl Iterator<Item=&mut ValueRef<'ctx>>;
  fn get_num_results(&'ctx self) -> usize {self.get_results().count()}
  fn get_result_types(&'ctx self) -> impl Iterator<Item=TypeRef<'ctx>> { type_iter(self.get_results()) }
  fn get_operands(&'ctx self) -> impl Iterator<Item=&ValueRef<'ctx>>;
  fn get_num_operands(&'ctx self) -> usize {self.get_operands().count()}
  fn get_operand_types(&'ctx self) -> impl Iterator<Item=TypeRef<'ctx>> { type_iter(self.get_operands()) }
}

macro_rules! variadic_op {
  ($($name:ident),*) => {$(
    struct $name<'ctx> {
      operands: Vec<ValueRef<'ctx>>,
      result: ValueRef<'ctx>
    }
    impl<'ctx> Op<'ctx> for $name<'ctx> {
      fn get_results(&'ctx self) -> impl Iterator<Item=&ValueRef<'ctx>> {
        slice::from_ref(&self.result).iter()
      }
      fn get_operands(&'ctx self) -> impl Iterator<Item=&ValueRef<'ctx>> {
        self.operands.iter()
      }
      fn get_operands_mut(&'ctx mut self) -> impl Iterator<Item=&mut ValueRef<'ctx>> {
        self.operands.iter_mut()
      }
    }
  )*}
}

macro_rules! binary_op {
  ($($name:ident),*) => {$(
    struct $name<'ctx> {
      operands: [ValueRef<'ctx>; 2],
      result: ValueRef<'ctx>
    }
    impl<'ctx> Op<'ctx> for $name<'ctx> {
      fn get_results(&'ctx self) -> impl Iterator<Item=&ValueRef<'ctx>> {
        slice::from_ref(&self.result).iter()
      }
      fn get_operands(&'ctx self) -> impl Iterator<Item=&ValueRef<'ctx>> {
        self.operands.iter()
      }
      fn get_operands_mut(&'ctx mut self) -> impl Iterator<Item=&mut ValueRef<'ctx>> {
        self.operands.iter_mut()
      }
    }
    impl<'ctx> $name<'ctx> {
      pub fn get_lhs(&self) -> &ValueRef<'ctx> {&self.operands[0]}
      pub fn get_lhs_mut(&mut self) -> &mut ValueRef<'ctx> {&mut self.operands[0]}
      pub fn get_rhs(&self) -> &ValueRef<'ctx> {&self.operands[1]}
      pub fn get_rhs_mut(&mut self) -> &mut ValueRef<'ctx> {&mut self.operands[1]}
    }
  )*}
}

variadic_op!(AddOp, MulOp, AndOp, XorOp, OrOp, ConcatOp);
binary_op!(SubOp, DivOp, RemOp);

struct ConstantOp<'ctx> {
  data: i32,
  result: ValueRef<'ctx>
}