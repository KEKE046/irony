use irony::preclude::*;

irony::data_type_enum![DataTypeEnum = { UInt(usize)}];

pub type ConstValue = irony::ConstValueU32<DataTypeEnum>;

#[derive(Debug, Clone, PartialEq)]
pub struct StringAttr(pub String);

impl Into<StringAttr> for &str {
    fn into(self) -> StringAttr { StringAttr(self.to_string()) }
}

impl std::fmt::Display for StringAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

irony::attribute_enum! {
    [data_type = DataTypeEnum]
    AttributeEnum = { ConstValue(ConstValue), StringAttr(StringAttr)}
}

type SameType = irony::SameTypeConstraint<DataTypeEnum, AttributeEnum>;
irony::constraint_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum]
    ConstraintEnum = {
        SameType(SameType),
    }
}

irony::entity_def! {
    [data_type = DataTypeEnum, attr=AttributeEnum]

    EntityEnum = {
        Wire: [name: StringAttr(StringAttr)],
        Module: [name: StringAttr(StringAttr)],
    }
}

irony::op_def! {

    [data_type = DataTypeEnum, attr = AttributeEnum, constraint = ConstraintEnum]

    OpEnum = {
        Constant : {
            defs: [lhs],
            uses: [],
            attrs: [rhs: ConstValue(ConstValue)],
            constraints: [SameType::new().into()],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, _attrs, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    format!("{} = {}", env.print_entity(defs[0].1[0].unwrap()), attrs[0].1)
                }
            ),
        }
    }
}


irony::environ_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum, entity = EntityEnum, op = OpEnum, constraint = ConstraintEnum]
    struct CirctEnv;
}


pub fn main() {
    let mut env = CirctEnv::default();
    let wire = env.add_entity(Wire::new(Some(DataTypeEnum::UInt(8)), Some("a".into())).into());
    let constant = env.add_op(
        Constant::new(
            Some(wire),
            Some(ConstValueU32::<DataTypeEnum> { value: 1, dtype: DataTypeEnum::UInt(8) }.into()),
        )
        .into(),
    );

    println!("{}", env.print_op(constant));

}
