use irony::AttributeTrait;

#[derive(Clone, Debug, PartialEq)]
pub struct StructType(pub Vec<(String, Box<DataTypeEnum>)>);

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayType(pub Box<DataTypeEnum>, pub usize);

#[derive(Clone, Debug, PartialEq)]
pub struct UArrayType(pub Box<DataTypeEnum>, pub usize);

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOpAttr {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Shl,
    Shr,
    And,
    Or,
    Xor,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Slt,
    Sle,
    Sgt,
    Sge,
}

impl AttributeTrait for BinaryOpAttr {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }

    type DataTypeT=DataTypeEnum;
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringAttr(pub String);
impl AttributeTrait for StringAttr {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
    type DataTypeT=DataTypeEnum;
}

#[derive(Clone, Debug, PartialEq)]
pub struct UIntAttr(pub u32);
impl AttributeTrait for UIntAttr {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
    type DataTypeT=DataTypeEnum;
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAttr(pub DataTypeEnum);
impl AttributeTrait for TypeAttr {
    fn dtype(&self) -> DataTypeEnum {
        self.0.clone()
    }
    type DataTypeT=DataTypeEnum;
}

pub type ConstAttr = irony::ConstValueI32<DataTypeEnum>;



#[derive(Clone, Debug, PartialEq)]
pub struct ArrayAttr(pub Vec<AttributeEnum>);
impl AttributeTrait for ArrayAttr {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
    type DataTypeT=DataTypeEnum;
}

impl<I: Into<AttributeEnum>> Into<ArrayAttr> for Vec<I> {
    fn into(self) -> ArrayAttr {
        ArrayAttr(self.into_iter().map(|x| x.into()).collect())
    }
}

irony::data_type_enum![
    DataTypeEnum = UInt(usize),
    Struct(StructType),
    Array(ArrayType),
    UArray(UArrayType),
    None
];

irony::attribute_enum! {
    [data_type = DataTypeEnum]
    AttributeEnum = ConstAttr(ConstAttr), UIntAttr(UIntAttr), StringAttr(StringAttr), TypeAttr(TypeAttr), ArrayAttr(ArrayAttr), BinaryOpAttr(BinaryOpAttr)
}
