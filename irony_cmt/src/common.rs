

#[derive(Clone, Debug, PartialEq)]
pub struct UIntType(pub usize);

// TODO: fix this
impl std::fmt::Display for UIntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "i{}", self.0)
    }
}

impl Into<UIntType> for usize {
    fn into(self) -> UIntType {
        UIntType(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructType(pub Vec<(String, Box<DataTypeEnum>)>);

// TODO: fix this
impl std::fmt::Display for StructType {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayType(pub Box<DataTypeEnum>, pub usize);

// TODO: fix this
impl std::fmt::Display for ArrayType {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UArrayType(pub Box<DataTypeEnum>, pub usize);

// TODO: fix this
impl std::fmt::Display for UArrayType {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SeqHlmemType(pub Box<DataTypeEnum>, pub Vec<usize>);

// TODO: fix this
impl std::fmt::Display for SeqHlmemType {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum CombVariadicPredicate {
    Add,
    Mul,
    And,
    Or,
    Xor,
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

impl std::fmt::Display for CombVariadicPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_str())
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum CombUnaryPredicate {
    Not,
    Neg, 
}

impl CombUnaryPredicate {
    pub fn get_str(&self) -> &'static str {
        match self {
            CombUnaryPredicate::Not => "not",
            CombUnaryPredicate::Neg => "neg",
        }
    }
}

impl std::fmt::Display for CombUnaryPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_str())
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

impl std::fmt::Display for CombBinaryPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_str())
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

impl std::fmt::Display for CombICmpPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_str())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringAttr(pub String);


impl Into<StringAttr> for &str {
    fn into(self) -> StringAttr {
        StringAttr(self.to_string())
    }
}

impl Into<StringAttr> for String {
    fn into(self) -> StringAttr {
        StringAttr(self)
    }
}

impl std::fmt::Display for StringAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoolAttr(pub bool);

impl Into<BoolAttr> for bool {
    fn into(self) -> BoolAttr {
        BoolAttr(self)
    }
}

impl std::fmt::Display for BoolAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 { "1" } else { "0" })
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct UIntAttr(pub u32);


impl Into<UIntAttr> for u32 {
    fn into(self) -> UIntAttr {
        UIntAttr(self)
    }
}
impl Into<UIntAttr> for usize {
    fn into(self) -> UIntAttr {
        UIntAttr(self as u32)
    }
}

impl std::fmt::Display for UIntAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAttr(pub DataTypeEnum);

impl std::fmt::Display for TypeAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantAttr(pub Vec<bool>);
impl std::fmt::Display for ConstantAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for b in &self.0 {
            s.push_str(if *b { "1" } else { "0" });
        }
        write!(f, "{}", s)
    }
}
impl<const N:usize> Into<ConstantAttr> for [u32;N] {
    fn into(self) -> ConstantAttr {
        let mut v = Vec::new();
        for i in 0..N {
            v.push(self[i] != 0);
        }
        ConstantAttr(v)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayAttr(pub Vec<AttributeEnum>);
impl std::fmt::Display for ArrayAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sub_str = Vec::new();
        for b in &self.0 {
            sub_str.push(format!("{}", b));
        }
        write!(f, "{}", sub_str.join(","))
    }
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

irony::data_type_enum![
    DataTypeEnum = {
        UInt(UIntType),
        Struct(StructType),
        Array(ArrayType),
        UArray(UArrayType),
        SeqHlmem(SeqHlmemType),
    }
];

irony::attribute_enum! {
    [data_type = DataTypeEnum]
    AttributeEnum = {
        ConstantAttr(ConstantAttr),
        BoolAttr(BoolAttr),
        UIntAttr(UIntAttr), 
        StringAttr(StringAttr), 
        TypeAttr(TypeAttr), 
        ArrayAttr(ArrayAttr), 
        CombVariadicPredicate(CombVariadicPredicate), 
        CombBinaryPredicate(CombBinaryPredicate), 
        CombUnaryPredicate(CombUnaryPredicate),
        CombICmpPredicate(CombICmpPredicate)
    }
}
