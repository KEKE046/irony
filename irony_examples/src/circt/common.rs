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

impl AttributeTrait<DataTypeEnum> for BinaryOpAttr {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
}

impl AttributeTrait<DataTypeEnum> for String {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
}

impl AttributeTrait<DataTypeEnum> for u32 {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAttr(pub DataTypeEnum);
impl AttributeTrait<DataTypeEnum> for TypeAttr {
    fn dtype(&self) -> DataTypeEnum {
        self.0.clone()
    }
}

pub type ConstAttr = irony::ConstValueI32<DataTypeEnum>;
pub type StringAttr = String;
pub type UIntAttr = u32;

pub type ArrayAttr = Vec<Box<AttributeEnum>>;

impl AttributeTrait<DataTypeEnum> for ArrayAttr {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
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
