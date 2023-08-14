

#[allow(unused_variables)]
pub use irony::{self, preclude::*};

/// define types and attributes
mod common;
mod constraints;
mod passes;

pub use common::*;
pub use constraints::*;
pub use passes::*;

pub use indexmap;

mod utils;

irony::entity_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum]

    EntityEnum = {
        Wire: [name: StringAttr(StringAttr)],
        Module: [name: StringAttr(StringAttr), top: BoolAttr(BoolAttr)],
    }
}

irony::op_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum, constraint = ConstraintEnum]

    OpEnum = {
        // ------ BEGIN: define the operations in `hw` dialect -------
        Assign: {
            defs: [lhs],
            uses: [rhs],
            attrs: [invalid: BoolAttr(BoolAttr)],   // TODO: This should be marked at pass manager rather than the operation itself
            constraints: [SameType::new().into()],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>,  defs:Vec<(String, Vec<Option<EntityId>>)>, _ | {
                    format!("{} = {}", env.print_entity(defs[0].1[0].unwrap()), env.print_entity(uses[0].1[0].unwrap()))
                }
            )
        },
        // ------ END: define the operations in `hw` dialect -------

        // ------ BEGIN: define the operations in `hw` dialect -------
        HwModule: {
            defs: [lhs],
            uses: [],
            attrs: [name: StringAttr(StringAttr), arg_names: ArrayAttr(ArrayAttr), arg_types: ArrayAttr(ArrayAttr), output_names: ArrayAttr(ArrayAttr), output_types: ArrayAttr(ArrayAttr)],
            constraints: [ModuleConstraint::default().into()],
            regions: [body],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, _ , _, regions: Vec<(String, RegionId)>| {
                    let AttributeEnum::ArrayAttr(arg_names) = irony::utils::extract_vec(&attrs, "arg_names").unwrap() else { panic!("")};
                    let AttributeEnum::ArrayAttr(arg_types) = irony::utils::extract_vec(&attrs, "arg_types").unwrap() else { panic!("")};

                    let AttributeEnum::ArrayAttr(output_names) = irony::utils::extract_vec(&attrs, "output_names").unwrap() else { panic!("")};
                    let AttributeEnum::ArrayAttr(output_types) = irony::utils::extract_vec(&attrs, "output_types").unwrap() else { panic!("")};
                    let name = irony::utils::extract_vec(&attrs, "name").unwrap();

                    let args = arg_names.0.iter().zip(arg_types.0.iter()).map(|(name, ty)| {
                        format!("%{}: {}", name, ty)
                    }).collect::<Vec<_>>().join(", ");

                    let outputs = output_names.0.iter().zip(output_types.0.iter()).map(|(name, ty)| {
                        format!("{}: {}", name, ty)
                    }).collect::<Vec<_>>().join(", ");
                    format!("hw.module @{}({}) -> ({}) {}", name, args, outputs, env.print_region(regions[0].1))
                }
            )
        },

        // TODO: Support EXT_W_PARAMS ?
        HwInstance: {
            defs: [; outputs],
            uses: [; inputs],
            attrs: [target_id: IdAttr(IdAttr), name: StringAttr(StringAttr)],
            constraints: [InstanceConstraint::default().into()],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let AttributeEnum::IdAttr(target_id) = irony::utils::extract_vec(&attrs, "target_id").unwrap() else { panic!("")};
                    let module_attrs = env.get_op(env.get_entity(EntityId(target_id.0 as usize)).get_defs(env)[0]).get_attrs();
                    let AttributeEnum::StringAttr(instance_name) = irony::utils::extract_vec(&attrs, "name").unwrap() else { panic!("")};

                    let AttributeEnum::ArrayAttr(arg_names) = irony::utils::extract_vec(&module_attrs, "arg_names").unwrap() else { panic!("")};
                    let AttributeEnum::ArrayAttr(arg_types) = irony::utils::extract_vec(&module_attrs, "arg_types").unwrap() else { panic!("")};

                    let AttributeEnum::ArrayAttr(output_names) = irony::utils::extract_vec(&module_attrs, "output_names").unwrap() else { panic!("")};
                    let AttributeEnum::ArrayAttr(output_types) = irony::utils::extract_vec(&module_attrs, "output_types").unwrap() else { panic!("")};
                    let AttributeEnum::StringAttr(mod_name) = irony::utils::extract_vec(&module_attrs, "name").unwrap() else { panic!("")};

                    let outputs = defs[0].1.iter().map(|id| {
                        env.print_entity((*id).unwrap())
                    }).collect::<Vec<_>>().join(", ");

                    let output_types = output_names.0.iter().zip(output_types.0.iter()).map(|(name, ty)| {
                        format!("{}: {}", name, ty)
                    }).collect::<Vec<_>>().join(", ");

                    let args = arg_names.0.iter().zip(uses[0].1.iter()).zip(arg_types.0.iter()).map(|((name, id), ty)| {
                        format!("{} : {} : {}", name, env.print_entity((*id).unwrap()), ty)
                    }).collect::<Vec<_>>().join(", ");

                    format!("{} = hw.instance \"{}\" @{}({}) -> ({})", outputs, instance_name, mod_name, args, output_types)
                }
            )
        },

        HwInput: {
            defs: [; inputs],
            uses: [],
            print: (
                |_, _, _, _, _| {
                    format!("")
                }
            )
        },

        HwOutput: {
            defs: [],
            uses: [; outputs],
            constraints:[],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, _, _| {
                    let outputs = uses[0].1.iter().map(|id| {
                        format!("{}", env.print_entity((*id).unwrap()))
                    }).collect::<Vec<_>>().join(", ");
                    let output_types = uses[0].1.iter().map(|id| {
                        format!("{}", env.get_entity((*id).unwrap()).get_dtype().unwrap())
                    }).collect::<Vec<_>>().join(", ");
                    format!("hw.output {}: {}", outputs, output_types)
                }
            )
        },

        HwBitCast: {
            defs: [lhs],
            uses: [rhs],
            print: (
                |_, _, _, _, _| {
                    unimplemented!()
                }
            )
        },

        // TODO: support super large constant and boolean constant
        HwConstant: {
            defs: [lhs],
            uses: [],
            attrs: [value: ConstantAttr(ConstantAttr)],
            constraints: [SameTypeConstant::default().into()],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, _, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let AttributeEnum::ConstantAttr(value) = irony::utils::extract_vec(&attrs, "value").unwrap() else { panic!("")};
                    let value = irony::utils::arith::from_bits_to_u32(value.0);
                    let names = defs[0].1.iter().map(|id| {
                        format!("{}", env.print_entity((*id).unwrap()))
                    }).collect::<Vec<_>>().join(", ");
                    let types = defs[0].1.iter().map(|id| {
                        format!("{}", env.get_entity((*id).unwrap()).get_dtype().unwrap())
                    }).collect::<Vec<_>>().join(", ");
                    format!("{} = hw.constant {}: {}", names, value, types)
                }
            )
        },

        HwAggregateConstant: {
            defs: [lhs],
            uses: [],
            attrs: [attrs: ArrayAttr(ArrayAttr)],
            constraints: [SameTypeAggregate::default().into()],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, _, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let attrs = irony::utils::extract_vec(&attrs, "attrs").unwrap();
                    let name = format!("{}", env.print_entity(defs[0].1[0].unwrap()));
                    let types = format!("{}", env.get_entity(defs[0].1[0].unwrap()).get_dtype().unwrap());
                    let values = attrs.print_for_aggregate_constant(env.get_entity(defs[0].1[0].unwrap()).get_dtype().unwrap());
                    format!("{} = hw.aggregate_constant {} : {}", name, values, types)
                }
            )
        },

        HwArrayConcat: {
            defs: [lhs],
            uses: [; operands],
            constraints: [ArrayConcatConstraint::default().into()],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let rst = env.print_entity(defs[0].1[0].unwrap());
                    let operands = uses[0].1.iter().map(|id| {
                        format!("{}", env.print_entity((*id).unwrap()))
                    }).collect::<Vec<_>>().join(", ");
                    let sub_typs = uses[0].1.iter().map(|id| {
                        format!("{}", env.get_entity((*id).unwrap()).get_dtype().unwrap())
                    }).collect::<Vec<_>>().join(", ");
                    format!("{} = hw.array_concat {} : {}", rst, operands, sub_typs)
                }
            )
        },

        HwArrayCreate: {
            defs: [lhs],
            uses: [; operands],
            constraints: [ArrayCreateConstraint::default().into(), SameTypeOperands::new().into()],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let rst = env.print_entity(defs[0].1[0].unwrap());
                    let operands = uses[0].1.iter().map(|id| {
                        format!("{}", env.print_entity((*id).unwrap()))
                    }).collect::<Vec<_>>().join(", ");
                    let sub_typ = env.get_entity(uses[0].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = hw.array_create {} : {}", rst, operands, sub_typ)
                }
            )
        },

        HwArrayGet: {
            defs: [lhs],
            uses: [array, index],
            constraints: [ArrayGetConstraint::default().into()],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let rst = env.print_entity(defs[0].1[0].unwrap());
                    let array = env.print_entity(uses[0].1[0].unwrap());
                    let index = env.print_entity(uses[1].1[0].unwrap());
                    let array_typ = env.get_entity(uses[0].1[0].unwrap()).get_dtype().unwrap();
                    let index_typ = env.get_entity(uses[1].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = hw.array_get {}[{}] : {}, {}", rst, array, index, array_typ, index_typ)
                }
            )
        },

        HwArraySlice: {
            defs: [lhs],
            uses: [array, index],
            constraints: [ArraySliceConstraint::default().into()],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let rst = env.print_entity(defs[0].1[0].unwrap());
                    let array = env.print_entity(uses[0].1[0].unwrap());
                    let index = env.print_entity(uses[1].1[0].unwrap());
                    let old_typ = env.get_entity(uses[0].1[0].unwrap()).get_dtype().unwrap();
                    let new_typ = env.get_entity(defs[0].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = hw.array_slice {}[{}] : ({}) -> {}", rst, array, index, old_typ, new_typ)
                }
            )
        },

        HwStructCreate: {
            defs: [lhs],
            uses: [; operands],
            constraints: [StructCreateConstraint::default().into()],
            print: (
                |_, _, _, _, _| {
                    unimplemented!()
                }
            )
        },

        HwStructExtract: {
            defs: [lhs],
            uses: [struct_input, field],
            constraints: [StructExtractConstraint::default().into()],
            print: (
                |_, _, _, _, _| {
                    unimplemented!()
                }
            )
        },

        HwStructInject: {
            defs: [lhs],
            uses: [struct_input, field, new_value],
            constraints: [StructInjectConstraint::default().into()],
            print: (
                |_, _, _, _, _| {
                    unimplemented!()
                }
            )
        },

        HwStructExplode: {
            defs: [; outputs],
            uses: [struct_input],
            constraints: [StructExplodeConstraint::default().into()],
            print: (
                |_, _, _, _, _| {
                    unimplemented!()
                }
            )
        },

        // ------ END: define the operations in `hw` dialect -------

        // ------ BEGIN: define the operations in `comb` dialect -------
        // TODO: Add more constraints for safer usage
        CombVariadic: {
            defs: [lhs],
            uses: [; operands],
            attrs: [predicate: CombVariadicPredicate(CombVariadicPredicate)],
            constraints: [SameType::new().into()],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, uses: Vec<(String, Vec<Option<EntityId>>)>,  defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let def = env.print_entity(defs[0].1[0].unwrap());
                    let uses = uses[0].1.iter().map(|id| {
                        format!("{}", env.print_entity((*id).unwrap()))
                    }).collect::<Vec<_>>().join(", ");
                    let AttributeEnum::CombVariadicPredicate(predicate) = irony::utils::extract_vec(&attrs, "predicate").unwrap() else { panic!("")};
                    let typ = env.get_entity(defs[0].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = comb.{} {} : {}", def, predicate, uses, typ)
                }
            )
        },
        CombBinary: {
            defs: [lhs],
            uses: [op0, op1],
            attrs: [predicate: CombBinaryPredicate(CombBinaryPredicate)],
            constraints: [SameType::new().into()],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let def = env.print_entity(defs[0].1[0].unwrap());
                    let uses = vec![env.print_entity(uses[0].1[0].unwrap()), env.print_entity(uses[1].1[0].unwrap())].join(", ");
                    let AttributeEnum::CombBinaryPredicate(predicate) = irony::utils::extract_vec(&attrs, "predicate").unwrap() else { panic!("")};
                    let typ = env.get_entity(defs[0].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = comb.{} {} : {}", def, predicate, uses, typ)
                }
            )
        },
        CombUnary: {
            defs: [lhs],
            uses: [op],
            attrs: [predicate: CombUnaryPredicate(CombUnaryPredicate)],
            constraints: [SameType::new().into()],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let def = env.print_entity(defs[0].1[0].unwrap());
                    let uses = vec![env.print_entity(uses[0].1[0].unwrap())].join(", ");
                    let AttributeEnum::CombUnaryPredicate(predicate) = irony::utils::extract_vec(&attrs, "predicate").unwrap() else { panic!("")};
                    let typ = env.get_entity(defs[0].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = ILLEGAL.{} {} : {}", def, predicate, uses, typ)
                }
            )
        },
        CombICmp: {
            defs: [lhs],
            uses: [op0, op1],
            attrs: [predicate: CombICmpPredicate(CombICmpPredicate)],
            constraints: [SameTypeOperands::new().into()],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String,Vec<Option<EntityId>>)>, _| {
                    let def = env.print_entity(defs[0].1[0].unwrap());
                    let inputs = vec![env.print_entity(uses[0].1[0].unwrap()), env.print_entity(uses[1].1[0].unwrap())].join(", ");
                    let AttributeEnum::CombICmpPredicate(predicate) = irony::utils::extract_vec(&attrs, "predicate").unwrap() else { panic!("")};
                    let typ = env.get_entity(uses[0].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = comb.icmp {} {} : {}", def, predicate, inputs, typ)
                }
            )
        },
        CombParity: {
            defs: [lhs],
            uses: [rhs],
            constraints: [/* TODO: fill this */],
            print: (
                |_, _, _, _, _| {
                    unimplemented!()
                }
            )
        },
        CombExtract: {
            defs: [lhs],
            uses: [input, low],
            constraints: [/* TODO: fill this */],
            print: (
                |_, _, _, _, _| {
                    unimplemented!()
                }
            )
        },
        CombConcat: {
            defs: [lhs],
            uses: [; operands],
            constraints: [/* TODO: fill this */],
            print: (
                |_, _, _, _, _| {
                    unimplemented!()
                }
            )
        },
        CombReplicate: {
            defs: [lhs],
            uses: [rhs],
            constraints: [/* TODO: fill this */],
            print: (
                |_, _, _, _, _| {
                    unimplemented!()
                }
            )
        },
        CombMux2: {
            defs: [lhs],
            uses: [cond, op0, op1],
            constraints: [/* TODO: fill this */],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let def = env.print_entity(defs[0].1[0].unwrap());
                    let uses = uses.iter().map(|(_, ids)| {
                        format!("{}", env.print_entity(ids[0].unwrap()))
                    }).collect::<Vec<_>>().join(", ");
                    let typ = env.get_entity(defs[0].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = comb.mux {} : {}", def, uses, typ)
                }
            )
        },
        // ------ END: define the operations in `comb` dialect -------

        // ------ BEGIN: define the operations in `seq` dialect -------
        SeqCompReg: {
            defs: [output],
            uses: [input, clk,reset,reset_val],
            attrs: [/*name: StringAttr(StringAttr)*/],
            constraints: [/* TODO: fill this */],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let output_name = env.print_entity(defs[0].1[0].unwrap());
                    let input_name = match uses.iter().find(|(name, _)| name == "input").and_then(|(_, ids)| Some(env.print_entity(ids[0].unwrap()))) {
                        Some(name) => name,
                        None => format!(""),
                    };
                    let clk = match uses.iter().find(|(name, _)| name == "clk").and_then(|(_, ids)| Some(env.print_entity(ids[0].unwrap()))) {
                        Some(name) => name,
                        None => format!(""),
                    };
                    let reset = match uses.iter().find(|(name, _)| name == "reset").and_then(|(_, ids)| {
                        if let Some(id) = ids[0] {
                            Some(env.print_entity(id))
                        } else {
                            None
                        }}) {
                        Some(name) => name,
                        None => format!(""),
                    };
                    let reset_val = match uses.iter().find(|(name, _)| name == "reset_val").and_then(|(_, ids)| {
                        if let Some(id) = ids[0] {
                            Some(env.print_entity(id))
                        } else {
                            None
                        }}) {
                        Some(name) => name,
                        None => format!(""),
                    };

                    let typ = env.get_entity(defs[0].1[0].unwrap()).get_dtype().unwrap();

                    format!("{} = seq.compreg {} {} {} {} : {}", output_name, input_name, clk, reset, reset_val, typ)
                }
            )
        },

        SeqHlmem: {
            defs: [handle],
            uses: [clk, reset],
            constraints: [/* TODO: fill this */],
            print: (
                |_, _, _, _, _| {
                    format!("")
                }
            )
        },

        SeqRead: {
            defs: [rdata],
            uses: [mem, renable; address],
            attrs: [latency: IdAttr(IdAttr)],
            print: (
                |_, _, _, _, _| {
                    format!("")
                }
            )
        },

        SeqWrite: {
            defs: [],
            uses: [mem, wenable, wdata; address],
            attrs: [latency: IdAttr(IdAttr)],
            print: (
                |_, _, _, _, _| {
                    format!("")
                }
            )
        },

    }
}

irony::environ_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum, entity = EntityEnum, op = OpEnum, constraint = ConstraintEnum, pm = PassManager]
    struct CmtEnv;
}

#[cfg(test)]
mod tests;
