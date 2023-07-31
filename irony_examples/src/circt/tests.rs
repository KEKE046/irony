mod basics {
    use crate::circt::*;
    use irony::*;


    #[test]
    fn build() -> Result<(),()> {
        let mut env = CirctEnv::default();
        let module = env.add_entity(Module::new("default").into());
        let region = env.add_region(Region::new().into());
        let module_def = env.add_op(ModuleDef::new(module, region).into());

        env.with_region(region, |env| {
            let child_region = env.add_region(Region::new().into());
            env.with_region(child_region, |env| {
                let wire_grand = env.add_entity(Wire::new("w_grand", DataTypeEnum::UInt(8)).into());
                assert_eq!(env.get_entity(wire_grand).get_parent(), Some(child_region));
            });
            let wire0 = env.add_entity(Wire::new("w0", DataTypeEnum::UInt(8)).into());
            let wire1 = env.add_entity(Wire::new("w1", DataTypeEnum::UInt(8)).into());
            let constant = env.add_op(
                Constant::new(
                    wire0,
                    ConstValueI32::<DataTypeEnum> {
                        value: 1,
                        dtype: DataTypeEnum::UInt(8),
                    }.into(),
                )
                .into(),
            );
            let assign = env.add_op(Assign::new(wire1, wire0).into());

            assert_eq!(env.get_entity(wire0).get_uses(env), vec![assign]);
            assert_eq!(env.get_entity(wire0).get_def(env), Some(constant));

            assert_eq!(env.get_entity(wire1).get_uses(env), vec![]);
            assert_eq!(env.get_entity(wire1).get_def(env), Some(assign));

            assert_eq!(env.get_entity(wire1).get_parent(), Some(region));
        });

        assert_eq!(env.get_entity(module).get_def(&env), Some(module_def));
        assert_eq!(env.get_op(module_def).get_regions(), vec![(format!("{}", "region"), region)]);
        assert_eq!(env.get_region(region).get_use(&env), Some(module_def));

        Ok(())
    }
}