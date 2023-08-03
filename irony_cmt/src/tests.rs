mod hw_test {
    use irony::{Environ, Region};

    use crate::*;
    #[test]
    pub fn module_constraint_test() {
        let mut circt = CmtEnv::default();
        let module = circt.add_entity(Module::new(None, Some("top".into()), Some(true.into())).into());
        let module_body = circt.add_region(Region::default());
        let module_def = circt.add_op(
            HwModule::new(
                Some(module),
                Some(StringAttr("top".into())),
                Some(vec![StringAttr("a".into()), StringAttr("b".into())].into()),
                Some(vec![
                    TypeAttr(DataTypeEnum::UInt(8)),
                    TypeAttr(DataTypeEnum::UInt(8)),
                ]
                .into()),
                Some(vec![StringAttr("c".into())].into()),
                Some(vec![TypeAttr(DataTypeEnum::UInt(8))].into()),
                Some(module_body),
            )
            .into(),
        );

        circt.with_region(module_body, |circt| {
            let a = circt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8)), Some("a".into())).into());
            let b = circt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8)), Some("b".into())).into());
            // let c = circt.add_entity(Wire::new("c", DataTypeEnum::UInt(8)).into());
            circt.add_op(HwInput::new(vec![a, b]).into());
            circt.add_op(HwOutput::new(vec![a]).into());
        });

        assert!(circt.verify_op(module_def))
    }

    #[test]
    pub fn instance_constraint_test() {
        let mut circt = CmtEnv::default();

        let module_pass = circt.add_entity(Module::new(None, Some("Pass".into()), None).into());
        let module_pass_body = circt.add_region(Region::default());
        let module_pass_def = circt.add_op(
            HwModule::new(
                Some(module_pass),
                Some(StringAttr("pass".into())),
                Some(vec![StringAttr("a".into())].into()),
                Some(vec![TypeAttr(DataTypeEnum::UInt(8))].into()),
                Some(vec![StringAttr("b".into())].into()),
                Some(vec![TypeAttr(DataTypeEnum::UInt(8))].into()),
                Some(module_pass_body),
            )
            .into(),
        );

        circt.with_region(module_pass_body, |circt| {
            let a = circt.add_entity(Wire::new( Some(DataTypeEnum::UInt(8)), Some("a".into())).into());
            // let c = circt.add_entity(Wire::new("c", DataTypeEnum::UInt(8)).into());
            circt.add_op(HwInput::new(vec![a]).into());
            circt.add_op(HwOutput::new(vec![a]).into());
        });

        assert!(circt.verify_op(module_pass_def));

        let module = circt.add_entity(Module::new(None, Some("top".into()), Some(true.into())).into());
        let module_body = circt.add_region(Region::default());
        circt.add_op(
            HwModule::new(
                Some(module),
                Some(StringAttr("top".into())),
                Some(vec![StringAttr("a".into())].into()),
                Some(vec![TypeAttr(DataTypeEnum::UInt(8))].into()),
                Some(vec![StringAttr("b".into())].into()),
                Some(vec![TypeAttr(DataTypeEnum::UInt(8))].into()),
                Some(module_body),
            )
            .into(),
        );

        circt.with_region(module_body, |circt| {
            let a = circt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8)), Some("a".into())).into());
            let b = circt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8)), Some("b".into())).into());

            circt.add_op(HwInput::new(vec![a]).into());
            circt.add_op(HwOutput::new(vec![b]).into());

            let instance = circt.add_op(
                HwInstance::new(
                    vec![b],
                    vec![a],
                    Some(UIntAttr(module_pass.id() as u32)),
                    Some(StringAttr("pass_inst".into())),
                )
                .into(),
            );

            assert!(circt.verify_op(instance))
        });
    }
}
