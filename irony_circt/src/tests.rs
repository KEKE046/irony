use irony::{Environ, Region};

use crate::*;

#[test]
pub fn module_constraint_test() {
    let mut circt = CirctEnv::default();
    let module = circt.add_entity(Module::new("top").into());
    let module_body = circt.add_region(Region::default());
    let module_def = circt.add_op(
        ModuleDef::new(
            module,
            StringAttr("top".into()),
            vec![StringAttr("a".into()), StringAttr("b".into())].into(),
            vec![
                TypeAttr(DataTypeEnum::UInt(8)),
                TypeAttr(DataTypeEnum::UInt(8)),
            ]
            .into(),
            vec![StringAttr("c".into())].into(),
            vec![TypeAttr(DataTypeEnum::UInt(8))].into(),
            module_body,
        )
        .into(),
    );

    circt.with_region(module_body, |circt| {
        let a = circt.add_entity(Wire::new("a", DataTypeEnum::UInt(8)).into());
        let b = circt.add_entity(Wire::new("b", DataTypeEnum::UInt(8)).into());
        // let c = circt.add_entity(Wire::new("c", DataTypeEnum::UInt(8)).into());
        circt.add_op(Input::new(vec![a, b]).into());
        circt.add_op(Output::new(vec![a]).into());
    });

    assert!(circt.verify_op(module_def))
}

#[test]
pub fn instance_constraint_test() {
    let mut circt = CirctEnv::default();

    let module_pass = circt.add_entity(Module::new("pass").into());
    let module_pass_body = circt.add_region(Region::default());
    let module_pass_def = circt.add_op(
        ModuleDef::new(
            module_pass,
            StringAttr("pass".into()),
            vec![StringAttr("a".into())].into(),
            vec![TypeAttr(DataTypeEnum::UInt(8))].into(),
            vec![StringAttr("b".into())].into(),
            vec![TypeAttr(DataTypeEnum::UInt(8))].into(),
            module_pass_body,
        )
        .into(),
    );
    
    circt.with_region(module_pass_body, |circt| {
        let a = circt.add_entity(Wire::new("a", DataTypeEnum::UInt(8)).into());
        // let c = circt.add_entity(Wire::new("c", DataTypeEnum::UInt(8)).into());
        circt.add_op(Input::new(vec![a]).into());
        circt.add_op(Output::new(vec![a]).into());
    });

    assert!(circt.verify_op(module_pass_def));


    let module = circt.add_entity(Module::new("top").into());
    let module_body = circt.add_region(Region::default());
    circt.add_op(
        ModuleDef::new(
            module,
            StringAttr("top".into()),
            vec![StringAttr("a".into())].into(),
            vec![TypeAttr(DataTypeEnum::UInt(8))].into(),
            vec![StringAttr("b".into())].into(),
            vec![TypeAttr(DataTypeEnum::UInt(8))].into(),
            module_body,
        )
        .into(),
    );


    circt.with_region(module_body, |circt| {
        let a = circt.add_entity(Wire::new("a", DataTypeEnum::UInt(8)).into());
        let b = circt.add_entity(Wire::new("b", DataTypeEnum::UInt(8)).into());

        circt.add_op(Input::new(vec![a]).into());
        circt.add_op(Output::new(vec![b]).into());

        let instance = circt.add_op(Instance::new(vec![b], vec![a], UIntAttr(module_pass.id() as u32), StringAttr("pass_inst".into())).into());

        assert!(circt.verify_op(instance))

    });
}
