mod hw_test {
    use std::vec;

    use irony::{Environ, Region};

    use crate::*;

    pub fn create() -> (CmtEnv, EntityId, OpId) {
        let mut cmt = CmtEnv::default();

        let module_pass = cmt.add_entity(Module::new(None, Some("Pass".into()), None).into());
        let module_pass_body = cmt.add_region(Region::default());
        let module_pass_def = cmt.add_op(
            HwModule::new(
                Some(module_pass),
                Some(StringAttr("pass".into())),
                Some(vec![StringAttr("a".into())].into()),
                Some(vec![TypeAttr(DataTypeEnum::UInt(8.into()))].into()),
                Some(vec![StringAttr("b".into())].into()),
                Some(vec![TypeAttr(DataTypeEnum::UInt(8.into()))].into()),
                Some(module_pass_body),
            )
            .into(),
        );

        cmt.with_region(Some(module_pass_body), |cmt| {
            let a = cmt.add_entity(Wire::new( Some(DataTypeEnum::UInt(8.into())), Some("a".into())).into());
            // let c = circt.add_entity(Wire::new("c", DataTypeEnum::UInt(8.into())).into());
            cmt.add_op(HwInput::new(vec![a]).into());
            cmt.add_op(HwOutput::new(vec![a]).into());
        });

        assert!(cmt.verify_op(module_pass_def));

        let module = cmt.add_entity(Module::new(None, Some("top".into()), Some(true.into())).into());
        let module_body = cmt.add_region(Region::default());
        let module_def = cmt.add_op(
            HwModule::new(
                Some(module),
                Some(StringAttr("top".into())),
                Some(vec![StringAttr("a".into())].into()),
                Some(vec![TypeAttr(DataTypeEnum::UInt(8.into()))].into()),
                Some(vec![StringAttr("b".into())].into()),
                Some(vec![TypeAttr(DataTypeEnum::UInt(8.into()))].into()),
                Some(module_body),
            )
            .into(),
        );

        cmt.with_region(Some(module_body), |cmt| {
            let clk = cmt.add_entity(Wire::new(Some(DataTypeEnum::UInt(1.into())), Some("clk".into())).into());
            let a = cmt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8.into())), Some("a".into())).into());
            let b = cmt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8.into())), Some("b".into())).into());
            let c = cmt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8.into())), Some("c".into())).into());
            let d = cmt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8.into())), Some("d".into())).into());
            let e = cmt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8.into())), Some("e".into())).into());
            let cond = cmt.add_entity(Wire::new(Some(DataTypeEnum::UInt(1.into())), Some("cond".into())).into());
            let h = cmt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8.into())), Some("h".into())).into());
            let h_reg = cmt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8.into())), Some("h_reg".into())).into());

            cmt.add_op(HwInput::new(vec![a, clk]).into());

            let instance = cmt.add_op(
                HwInstance::new(
                    vec![b],
                    vec![a],
                    Some(UIntAttr(module_pass.id() as u32)),
                    Some(StringAttr("pass_inst".into())),
                )
                .into(),
            );
            
            assert!(cmt.verify_op(instance));


            cmt.add_op(HwConstant::new(Some(c), Some([1, 0, 0, 0].into())).into());
            cmt.add_op(CombVariadic::new(
                Some(d),
                vec![b, c].into(),
                Some(CombVariadicPredicate::Add),
            ).into());
            cmt.add_op(CombUnary::new(Some(e), Some(d), Some(CombUnaryPredicate::Not)).into());

            cmt.add_op(CombICmp::new(
                Some(cond),
                Some(e),
                Some(d),
                Some(CombICmpPredicate::EQ),
            ).into());

            cmt.add_op(CombMux2::new(
                Some(h),
                Some(cond),
                Some(d),
                Some(e),
            ).into());

            cmt.add_op(SeqCompReg::new(
                Some(h_reg),
                Some(h),
                Some(clk),
                None,
                None
            ).into());

            cmt.add_op(HwOutput::new(vec![h_reg]).into());

        });
        (cmt, module, module_def)
    }

    #[test]
    pub fn print_test() {
        let (mut cmt, _, _) = create();
        
        let no_parent = cmt.op_table.iter().filter(|(_, op)| op.get_parent().is_none()).map(|(id, _)| OpId(*id)).collect::<Vec<_>>();

        for op in no_parent.iter() {
            println!("{}", cmt.print_op(*op));
        }

        println!();
        println!("run pass: RenamePass\n");

 
        println!("no parent: {:?}", no_parent);

        cmt.pass_manager.add_passes(vec![PassEnum::RenamePass(RenamePass)], vec![no_parent.to_owned()]);
        
        cmt.run_passes();

        for op in no_parent.iter() {
            println!("{}", cmt.print_op(*op));
        }
        

    }

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
                    TypeAttr(DataTypeEnum::UInt(8.into())),
                    TypeAttr(DataTypeEnum::UInt(8.into())),
                ]
                .into()),
                Some(vec![StringAttr("c".into())].into()),
                Some(vec![TypeAttr(DataTypeEnum::UInt(8.into()))].into()),
                Some(module_body),
            )
            .into(),
        );

        circt.with_region(Some(module_body), |circt| {
            let a = circt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8.into())), Some("a".into())).into());
            let b = circt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8.into())), Some("b".into())).into());
            // let c = circt.add_entity(Wire::new("c", DataTypeEnum::UInt(8.into())).into());
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
                Some(vec![TypeAttr(DataTypeEnum::UInt(8.into()))].into()),
                Some(vec![StringAttr("b".into())].into()),
                Some(vec![TypeAttr(DataTypeEnum::UInt(8.into()))].into()),
                Some(module_pass_body),
            )
            .into(),
        );

        circt.with_region(Some(module_pass_body), |circt| {
            let a = circt.add_entity(Wire::new( Some(DataTypeEnum::UInt(8.into())), Some("a".into())).into());
            // let c = circt.add_entity(Wire::new("c", DataTypeEnum::UInt(8.into())).into());
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
                Some(vec![TypeAttr(DataTypeEnum::UInt(8.into()))].into()),
                Some(vec![StringAttr("b".into())].into()),
                Some(vec![TypeAttr(DataTypeEnum::UInt(8.into()))].into()),
                Some(module_body),
            )
            .into(),
        );

        circt.with_region(Some(module_body), |circt| {
            let a = circt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8.into())), Some("a".into())).into());
            let b = circt.add_entity(Wire::new(Some(DataTypeEnum::UInt(8.into())), Some("b".into())).into());

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
