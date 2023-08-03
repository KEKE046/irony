use irony::AttributeTrait;

#[derive(Clone, Debug, PartialEq)]
pub struct StructType(pub Vec<(String, Box<DataTypeEnum>)>);

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayType(pub Box<DataTypeEnum>, pub usize);

#[derive(Clone, Debug, PartialEq)]
pub struct UArrayType(pub Box<DataTypeEnum>, pub usize);

#[derive(Clone, Debug, PartialEq)]
pub enum CombVariadicPredicate {
    Add,
    Mul,
    And,
    Or,
    Xor,
}
impl AttributeTrait for CombVariadicPredicate {
    type DataTypeT = DataTypeEnum;
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
}

impl CombVariadicPredicate {
    pub fn get_str(&self) -> &'static str {
        match self {
            CombVariadicPredicate::Add => "add",
            CombVariadicPredicate::Mul => "mul",
            CombVariadicPredicate::And => "and",
            CombVariadicPredicate::Or => "or",
            CombVariadicPredicate::Xor => "xor",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CombBinaryPredicate {
    DivU,
    DivS,
    ModU,
    ModS,
    Shl,
    ShrU,
    ShrS,
    Sub,
}
impl AttributeTrait for CombBinaryPredicate {
    type DataTypeT = DataTypeEnum;
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
}

impl CombBinaryPredicate {
    pub fn get_str(&self) -> &'static str {
        match self {
            CombBinaryPredicate::DivU => "divu",
            CombBinaryPredicate::DivS => "divs",
            CombBinaryPredicate::ModU => "modu",
            CombBinaryPredicate::ModS => "mods",
            CombBinaryPredicate::Shl => "shl",
            CombBinaryPredicate::ShrU => "shru",
            CombBinaryPredicate::ShrS => "shrs",
            CombBinaryPredicate::Sub => "sub",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CombICmpPredicate {
    EQ,
    NE,
    SLT,
    SLE,
    SGT,
    SGE,
    ULT,
    ULE,
    UGT,
    UGE,
    CEQ,
    CNE,
    WEQ,
    WNE,
}
impl AttributeTrait for CombICmpPredicate {
    type DataTypeT = DataTypeEnum;
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
}

impl CombICmpPredicate {
    pub fn get_str(&self) -> &'static str {
        match self {
            CombICmpPredicate::EQ => "eq",
            CombICmpPredicate::NE => "ne",
            CombICmpPredicate::SLT => "slt",
            CombICmpPredicate::SLE => "sle",
            CombICmpPredicate::SGT => "sgt",
            CombICmpPredicate::SGE => "sge",
            CombICmpPredicate::ULT => "ult",
            CombICmpPredicate::ULE => "ule",
            CombICmpPredicate::UGT => "ugt",
            CombICmpPredicate::UGE => "uge",
            CombICmpPredicate::CEQ => "ceq",
            CombICmpPredicate::CNE => "cne",
            CombICmpPredicate::WEQ => "weq",
            CombICmpPredicate::WNE => "wne",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringAttr(pub String);
impl AttributeTrait for StringAttr {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
    type DataTypeT = DataTypeEnum;
}

impl Into<StringAttr> for &str {
    fn into(self) -> StringAttr {
        StringAttr(self.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoolAttr(pub bool);
impl AttributeTrait for BoolAttr {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
    type DataTypeT = DataTypeEnum;
}

impl Into<BoolAttr> for bool {
    fn into(self) -> BoolAttr {
        BoolAttr(self)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct UIntAttr(pub u32);
impl AttributeTrait for UIntAttr {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
    type DataTypeT = DataTypeEnum;
}

impl Into<UIntAttr> for u32 {
    fn into(self) -> UIntAttr {
        UIntAttr(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAttr(pub DataTypeEnum);
impl AttributeTrait for TypeAttr {
    fn dtype(&self) -> DataTypeEnum {
        self.0.clone()
    }
    type DataTypeT = DataTypeEnum;
}


#[derive(Clone, Debug, PartialEq)]
pub struct ConstantAttr(pub Vec<bool>);
impl AttributeTrait for ConstantAttr {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
    type DataTypeT = DataTypeEnum;
}


#[derive(Clone, Debug, PartialEq)]
pub struct ArrayAttr(pub Vec<AttributeEnum>);
impl AttributeTrait for ArrayAttr {
    fn dtype(&self) -> DataTypeEnum {
        DataTypeEnum::None
    }
    type DataTypeT = DataTypeEnum;
}

impl<I: Into<AttributeEnum>> Into<ArrayAttr> for Vec<I> {
    fn into(self) -> ArrayAttr {
        ArrayAttr(self.into_iter().map(|x| x.into()).collect())
    }
}

impl Into<ArrayAttr> for () {
    fn into(self) -> ArrayAttr {
        ArrayAttr(Vec::<AttributeEnum>::new())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SeqHlmemType(pub Box<DataTypeEnum>, pub Vec<usize>);

irony::data_type_enum![
    DataTypeEnum = {
        UInt(usize),
        Struct(StructType),
        Array(ArrayType),
        UArray(UArrayType),
        SeqHlmem(SeqHlmemType),
        None
    }
];

irony::attribute_enum! {
    [data_type = DataTypeEnum]
    AttributeEnum = {
        ConstaintAttr(ConstantAttr),
        BoolAttr(BoolAttr),
        UIntAttr(UIntAttr), 
        StringAttr(StringAttr), 
        TypeAttr(TypeAttr), 
        ArrayAttr(ArrayAttr), 
        CombVariadicPredicate(CombVariadicPredicate), 
        CombBinaryPredicate(CombBinaryPredicate), 
        CombICmpPredicate(CombICmpPredicate)
    }
}
