mod basics {
    use crate::circt_naive::*;
    use irony::*;

    #[test]
    fn build() -> Result<(),()> {
        let mut env = CirctEnv::default();
        let module = env.add_entity(Module::new(None, Some("default".into())).into());
        let region = env.add_region(Region::new().into());
        let module_def = env.add_op(ModuleDef::new(Some(module), Some(region)).into());

        env.with_region(region, |env| {

            let wire0 = env.add_entity(Wire::new(Some(DataTypeEnum::UInt(8)), Some("w0".into())).into());
            let wire1 = env.add_entity(Wire::new(Some( DataTypeEnum::UInt(8)), Some("w1".into())).into());
            let constant = env.add_op(
                Constant::new(
                    Some(wire0),
                    Some(ConstValueU32::<DataTypeEnum> {
                        value: 1,
                        dtype: DataTypeEnum::UInt(8),
                    }.into()),
                )
                .into(),
            );
            let assign = env.add_op(Assign::new(Some(wire1), Some(wire0)).into());

            assert_eq!(wire0.get(env).get_uses(env), vec![assign]);
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

