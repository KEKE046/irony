#![feature(macro_metavar_expr)]
#[allow(unused_variables)]
pub use irony::{self, preclude::*};

/// define types and attributes
mod common;
mod constraints;
mod passes;

pub use common::*;
pub use constraints::*;
pub use indexmap;
pub use passes::*;

mod utils;

// pub use interpret::*;

irony::entity_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum]

    EntityEnum = {
        NONE: [],
        Event: [name: StringAttr(StringAttr), debug: BoolAttr(BoolAttr), location: LocationAttr(LocationAttr)],
        Sqn: [name: StringAttr(StringAttr), debug: BoolAttr(BoolAttr), location: LocationAttr(LocationAttr)],
        Prpt: [name: StringAttr(StringAttr), debug: BoolAttr(BoolAttr), location: LocationAttr(LocationAttr)],
        Wire: [name: StringAttr(StringAttr), debug: BoolAttr(BoolAttr), location: LocationAttr(LocationAttr)],
    }
}

irony::op_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum, constraint = ConstraintEnum]

    OpEnum = {

        // ------ BEGIN: define the operations in `event` dialect -------

        EventDef: {
            defs: [lhs],
            uses: [],
            print: (
                |env: &E, _, _, def: Vec<(String, Vec<Option<EntityId>>)>, _|  {
                    let lhs = env.print_entity(def[0].1[0].unwrap());
                    format!("{} = event.define", lhs)
                }
            )
        },

        EventFrom: {
            defs: [lhs],
            uses: [rhs],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _|  {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let rhs = env.print_entity(uses[0].1[0].unwrap());
                    format!("{} = event.from {}", lhs, rhs)
                }
            )
        },

        EventEval: {
            defs: [lhs],
            uses: [rhs],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _|  {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let rhs = env.print_entity(uses[0].1[0].unwrap());
                    format!("{} = event.eval {}", lhs, rhs)
                }
            )
        },

        EventBlockDef: {
            defs: [],
            uses: [event],
            regions: [body],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, _,  regions: Vec<(String, Vec<RegionId>)>| {
                    let event = env.print_entity(uses[0].1[0].unwrap());

                    let body = env.print_region(regions[0].1[0]);

                    format!("event.block {} {{\n{}\n}}",  event, body)
                }
            )
        },

        EventUnion: {
            defs: [],
            uses: [father, son],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, _, _| {
                    let father = env.print_entity(uses[0].1[0].unwrap());
                    let son = env.print_entity(uses[1].1[0].unwrap());

                    format!("event.union {} <- {}", father, son)
                }
            )
        },

        EventElseOf: {
            defs: [],
            uses: [e, t],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, _, _| {
                    let e = env.print_entity(uses[0].1[0].unwrap());
                    let t = env.print_entity(uses[1].1[0].unwrap());

                    format!("event.else_of {} <- {}", e, t)
                }
            )
        },

        // ------ END: define the operations in `event` dialect -------

        // ------ BEGIN: define the operations in `sequence` dialect -------
        SqnFromEvent: {
            defs: [lhs],
            uses: [rhs],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs:Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let rhs = env.print_entity(uses[0].1[0].unwrap());

                    format!("{} = sequence.from_event {}", lhs, rhs)
                }
            )
        },

        SqnDelay: {
            defs: [lhs],
            uses: [rhs],
            attrs: [lb: IdAttr(IdAttr)(*), ub: IdAttr(IdAttr)(*)],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, uses: Vec<(String, Vec<Option<EntityId>>)>, defs:Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let rhs = env.print_entity(uses[0].1[0].unwrap());
                    let lb = irony::utils::extract_vec(&attrs, "lb").unwrap();
                    let ub = match irony::utils::extract_vec(&attrs, "ub") {
                        Some(x) => format!("{}", x),
                        None => format!(""),
                    };
                    format!("{} = sequence.delay {} [{}:{}]", lhs, rhs, lb, ub)
                }
            )
        },

        SqnConcat: {
            defs: [lhs],
            uses: [s0, s1],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs:Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let s0 = env.print_entity(uses[0].1[0].unwrap());
                    let s1 = env.print_entity(uses[1].1[0].unwrap());

                    format!("{} = sequence.concat {}, {}", lhs, s0, s1)
                }
            )
        },

        // ------ END: define the operations in `sequence` dialect -------

        // ------ BEGIN: define the operations in `property` dialect -------

        PrptFromSqn: {
            defs: [lhs],
            uses: [rhs],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs:Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let rhs = env.print_entity(uses[0].1[0].unwrap());

                    format!("{} = property.from_sequence {}", lhs, rhs)
                }
            )
        },

        PrptNexttime: {
            defs: [rst],
            uses: [rhs],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs:Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let rhs = env.print_entity(uses[0].1[0].unwrap());

                    format!("{} = property.nexttime {}", lhs, rhs)
                }
            )
        },

        PrptAlways: {
            defs: [rst],
            uses: [rhs],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs:Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let rhs = env.print_entity(uses[0].1[0].unwrap());

                    format!("{} = property.always {}", lhs, rhs)
                }
            )
        },

        PrptEventually: {
            defs: [rst],
            uses: [rhs],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs:Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let rhs = env.print_entity(uses[0].1[0].unwrap());

                    format!("{} = property.eventually {}", lhs, rhs)
                }
            )
        },

        PrptUntil: {
            defs: [rst],
            uses: [a, b],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs:Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let a = env.print_entity(uses[0].1[0].unwrap());
                    let b = env.print_entity(uses[1].1[0].unwrap());

                    format!("{} = property.until {}, {}", lhs, a, b)
                }
            )
        },

        PrptConjunction: {
            defs: [rst],
            uses: [a, b],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs:Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let a = env.print_entity(uses[0].1[0].unwrap());
                    let b = env.print_entity(uses[1].1[0].unwrap());

                    format!("{} = property.and {}, {}", lhs, a, b)
                }
            )
        },

        PrptImplica: {
            defs: [rst],
            uses: [a, b],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs:Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let a = env.print_entity(uses[0].1[0].unwrap());
                    let b = env.print_entity(uses[1].1[0].unwrap());

                    format!("{} = property.implica {}, {}", lhs, a, b)
                }
            )
        },

        PrptSynth: {
            defs: [],
            uses: [property],
            print: (
                |env: &E, _attrs: Vec<(String, AttributeEnum)>, uses: Vec<(String, Vec<Option<EntityId>>)>, _, _| {
                    let property = env.print_entity(uses[0].1[0].unwrap());

                    let attr = "";

                    format!("property.synthesize {} {}", property, attr)
                }

            )
        },


        // ------ END: define the operations in `property` dialect -------

        // ------ BEGIN: define the operations in `temporary` dialect -------

        CasesOutput: {
            defs: [],
            uses: [; outputs],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, _, _| {
                    let outputs = uses[0].1.iter().map(|id| {
                        format!("{}", env.print_entity((*id).unwrap()))
                    }).collect::<Vec<_>>().join(", ");
                    let output_types = uses[0].1.iter().map(|id| {
                        format!("{}", env.get_entity((*id).unwrap()).get_dtype().unwrap())
                    }).collect::<Vec<_>>().join(", ");
                    format!("cases.output {}: {}", outputs, output_types)
                }
            )
        },

        Cases: {
            defs: [; results],
            uses: [; conds],
            attrs: [ onehot: BoolAttr(BoolAttr)(*)],
            regions: [dflt; bodies],
            constraints: [/* TODO */],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, regions: Vec<(String, Vec<RegionId>)>| {

                    let results = defs[0].1.iter().map(|id| {
                        format!("{}", env.print_entity((*id).unwrap()))
                    }).collect::<Vec<_>>().join(", ");

                    let mode = if let AttributeEnum::BoolAttr(BoolAttr(x)) = irony::utils::extract_vec(&attrs, "onehot").unwrap() {
                        if x {
                            "onehot"
                        } else {
                            "priority"
                        }
                    } else {
                        "priority"
                    };

                    let cases =
                        uses[0].1.iter().zip(regions[1].1.iter()).map(|(cond, body)| {
                            format!("{} : {{\n{}\n}}", env.print_entity(cond.unwrap()), env.print_region(*body))
                        }).collect::<Vec<_>>().join(", \n");

                    let default =
                        format!("default : {{\n{}\n}}", env.print_region(regions[0].1[0]));

                    let res_typs = defs[0].1.iter().map(|id| {
                        format!("{}", env.get_entity((*id).unwrap()).get_dtype().unwrap())
                    }).collect::<Vec<_>>().join(", ");

                    format!("{} = cases {} {{\n{}\n{}\n}} : {}", results, mode, irony::utils::print::tab(cases), irony::utils::print::tab(default), res_typs)
                }
            )
        },

        Select: {
            defs: [lhs],
            uses: [default; conds, values],
            attrs: [ onehot: BoolAttr(BoolAttr)(*)],
            constraints: [/* TODO */],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());

                    let mode = if let AttributeEnum::BoolAttr(BoolAttr(x)) = irony::utils::extract_vec(&attrs, "onehot").unwrap() {
                        if x {
                            "onehot"
                        } else {
                            "priority"
                        }
                    } else {
                        "priority"
                    };

                    let candidates = uses[2].1.iter().zip(uses[1].1.iter()).map(|(value, cond)| {
                        format!("\t{} : {}", env.print_entity(cond.unwrap()), env.print_entity(value.unwrap()))
                    }).collect::<Vec<_>>().join(", \n");


                    let default =  if let Some(default) = uses[0].1[0] {
                        format!("\tdefault : {}\n", env.print_entity(default))
                    } else  { String::default() } ;

                    let typ = env.get_entity(defs[0].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = ILLEGAL.select {} {{\n{}\n{}}} : {}", lhs, mode, candidates, default, typ)
                }
            )
        },

        CombUnary: {
            defs: [lhs],
            uses: [op],
            attrs: [predicate: CombUnaryPredicate(CombUnaryPredicate)(*)],
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

        Invalid: {
            defs: [lhs],
            uses: [],
            print: (
                |env: &E, _, _, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let typ = env.get_entity(defs[0].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = ILLEGAL.invalid : {}", lhs, typ)
                }
            )
        },

        // ------ END: define the operations in `temporary` dialect -------


        // ------ BEGIN: define the operations in `hw` dialect -------
        Assign: {
            defs: [lhs],
            uses: [rhs],
            constraints: [SameType::new().into()],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>,  defs:Vec<(String, Vec<Option<EntityId>>)>, _ | {
                    // format!("{} = {}", env.print_entity(defs[0].1[0].unwrap()), env.print_entity(uses[0].1[0].unwrap()))
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let rhs = env.print_entity(uses[0].1[0].unwrap());
                    let typ = env.get_entity(defs[0].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = hw.wire {} : {}", lhs, rhs, typ)
                }
            )
        },

        HwModule: {
            defs: [],
            uses: [],
            attrs: [name: StringAttr(StringAttr), top: BoolAttr(BoolAttr), arg_names: ArrayAttr(ArrayAttr), arg_types: ArrayAttr(ArrayAttr)(*), output_names: ArrayAttr(ArrayAttr), output_types: ArrayAttr(ArrayAttr)(*)],
            regions: [body],
            constraints: [ModuleConstraint::default().into()],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, _ , _, regions: Vec<(String, Vec<RegionId>)>| {
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
                    format!("hw.module @{}({}) -> ({}) {{\n{}\n}}", name, args, outputs, env.print_region(regions[0].1[0]))
                }
            )
        },

        // TODO: Support EXT_W_PARAMS ?
        HwInstance: {
            defs: [; outputs],
            uses: [; inputs],
            attrs: [target_op_id: OpIdAttr(OpIdAttr)(*), name: StringAttr(StringAttr)],
            constraints: [InstanceConstraint::default().into()],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let AttributeEnum::OpIdAttr(target_op_id) = irony::utils::extract_vec(&attrs, "target_op_id").unwrap() else { panic!("")};
                    let module_attrs = env.get_op(target_op_id.into()).get_attrs();
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
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let rhs = env.print_entity(uses[0].1[0].unwrap());
                    let rhs_typ = env.get_entity(uses[0].1[0].unwrap()).get_dtype().unwrap();
                    let lhs_typ = env.get_entity(defs[0].1[0].unwrap()).get_dtype().unwrap();

                    format!("{} = hw.bitcast {}: ({}) -> {}", lhs, rhs, rhs_typ, lhs_typ)
                }
            )
        },

        // TODO: support super large constant and boolean constant
        HwConstant: {
            defs: [lhs],
            uses: [],
            attrs: [value: ConstantAttr(ConstantAttr)(*)],
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
            attrs: [attrs: ArrayAttr(ArrayAttr)(*)],
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
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {

                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let operands = uses[0].1.iter().map(|id| {
                        format!("{}", env.print_entity((*id).unwrap()))
                    }).collect::<Vec<_>>().join(", ");
                    let lhs_ty = env.get_entity(defs[0].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = hw.struct_create ({}) : {}", lhs, operands, lhs_ty)
                }
            )
        },

        HwStructExtract: {
            defs: [lhs],
            uses: [struct_input],
            attrs: [field: StringAttr(StringAttr)(*)],
            constraints: [StructExtractConstraint::default().into()],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let struct_input = env.print_entity(uses[0].1[0].unwrap());
                    let field = irony::utils::extract_vec(&attrs, "field").unwrap();
                    let struct_ty = env.get_entity(uses[0].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = hw.struct_extract {}[\"{}\"] : {}", lhs, struct_input, field, struct_ty)
                }
            )
        },

        HwStructInject: {
            defs: [lhs],
            uses: [struct_input, new_value],
            attrs: [field: StringAttr(StringAttr)(*)],
            constraints: [StructInjectConstraint::default().into()],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let lhs = env.print_entity(defs[0].1[0].unwrap());
                    let struct_input = env.print_entity(uses[0].1[0].unwrap());
                    let new_value = env.print_entity(uses[1].1[0].unwrap());
                    let field = irony::utils::extract_vec(&attrs, "field").unwrap();
                    let struct_ty = env.get_entity(uses[0].1[0].unwrap()).get_dtype().unwrap();
                    format!("{} = hw.struct_inject {}[\"{}\"], {} : {}", lhs, struct_input, field, new_value, struct_ty)
                }
            )
        },

        HwStructExplode: {
            defs: [; outputs],
            uses: [struct_input],
            constraints: [StructExplodeConstraint::default().into()],
            print: (
                |env: &E, _, uses: Vec<(String, Vec<Option<EntityId>>)>, defs: Vec<(String, Vec<Option<EntityId>>)>, _| {
                    let outputs = defs[0].1.iter().map(|id| {
                        format!("{}", env.print_entity((*id).unwrap()))
                    }).collect::<Vec<_>>().join(", ");
                    let struct_input = env.print_entity(uses[0].1[0].unwrap());
                    let struct_ty = env.get_entity(uses[0].1[0].unwrap()).get_dtype().unwrap();

                    format!("{} = hw.struct_explode {} : {}", outputs, struct_input, struct_ty)
                }
            )
        },

        // ------ END: define the operations in `hw` dialect -------

        // ------ BEGIN: define the operations in `comb` dialect -------
        // TODO: Add more constraints for safer usage
        CombVariadic: {
            defs: [lhs],
            uses: [; operands],
            attrs: [predicate: CombVariadicPredicate(CombVariadicPredicate)(*)],
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
            attrs: [predicate: CombBinaryPredicate(CombBinaryPredicate)(*)],
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

        CombICmp: {
            defs: [lhs],
            uses: [op0, op1],
            attrs: [predicate: CombICmpPredicate(CombICmpPredicate)(*)],
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
        // CombParity: {
        //     defs: [lhs],
        //     uses: [rhs],
        //     constraints: [/* TODO: fill this */],
        //     print: (
        //         |_, _, _, _, _| {
        //             unimplemented!()
        //         }
        //     )
        // },
        // CombExtract: {
        //     defs: [lhs],
        //     uses: [input, low],
        //     constraints: [/* TODO: fill this */],
        //     print: (
        //         |_, _, _, _, _| {
        //             unimplemented!()
        //         }
        //     )
        // },
        // CombConcat: {
        //     defs: [lhs],
        //     uses: [; operands],
        //     constraints: [/* TODO: fill this */],
        //     print: (
        //         |_, _, _, _, _| {
        //             unimplemented!()
        //         }
        //     )
        // },
        // CombReplicate: {
        //     defs: [lhs],
        //     uses: [rhs],
        //     constraints: [/* TODO: fill this */],
        //     print: (
        //         |_, _, _, _, _| {
        //             unimplemented!()
        //         }
        //     )
        // },
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

        // SeqHlmem: {
        //     defs: [handle],
        //     uses: [clk, reset],
        //     constraints: [/* TODO: fill this */],
        //     print: (
        //         |_, _, _, _, _| {
        //             format!("")
        //         }
        //     )
        // },

        // SeqRead: {
        //     defs: [rdata],
        //     uses: [mem, renable; address],
        //     attrs: [latency: IdAttr(IdAttr)],
        //     print: (
        //         |_, _, _, _, _| {
        //             format!("")
        //         }
        //     )
        // },

        // SeqWrite: {
        //     defs: [],
        //     uses: [mem, wenable, wdata; address],
        //     attrs: [latency: IdAttr(IdAttr)],
        //     print: (
        //         |_, _, _, _, _| {
        //             format!("")
        //         }
        //     )
        // },

        // ------ END: define the operations in `seq` dialect -------


        // ------ BEGIN: define the operations in `interpret` dialect -------
        ItprtCondCheck: {
            defs: [],
            uses: [; conds],
            attrs: [has_default: BoolAttr(BoolAttr)(*), onehot: BoolAttr(BoolAttr)(*)],
            print: (
                |env: &E, attrs: Vec<(String, AttributeEnum)>, uses: Vec<(String, Vec<Option<EntityId>>)>, _, _| {

                    let conds = uses[0].1.iter().map(|id| {
                        format!("{}", env.print_entity((*id).unwrap()))
                    }).collect::<Vec<_>>().join(", ");

                    let has_default = irony::utils::extract_vec(&attrs, "has_default").unwrap();

                    let onehot = irony::utils::extract_vec(&attrs, "onehot").unwrap();
                    format!("itprt.cond_check {} {{has_default = {}, onehot = {}}}", conds, has_default, onehot)
                }
            )
        },

        // ------ END: define the operations in `interpret` dialect -------


    }
}

irony::environ_def! {
    [data_type = DataTypeEnum, attr = AttributeEnum, entity = EntityEnum, op = OpEnum, constraint = ConstraintEnum, pm = PassManager]
    struct CmtIR;
}

pub(crate) const NONE: NONE = NONE::const_new(None);

#[derive(Default, Debug)]
pub struct IdReducer {
    entity_set: FxHashMap<EntityId, usize>,
    op_set: FxHashMap<OpId, usize>,
}

impl ReducerTrait for IdReducer {
    fn reduce_entity(&mut self, id: EntityId) -> usize {
        let len = self.entity_set.len();
        match self.entity_set.entry(id) {
            std::collections::hash_map::Entry::Occupied(entry) => *entry.get(),
            std::collections::hash_map::Entry::Vacant(entry) => {
                let new_id = len;
                entry.insert(new_id);
                new_id
            },
        }
    }

    fn reduce_op(&mut self, id: OpId) -> usize {
        let len = self.op_set.len();
        match self.op_set.entry(id) {
            std::collections::hash_map::Entry::Occupied(entry) => *entry.get(),
            std::collections::hash_map::Entry::Vacant(entry) => {
                let new_id = len;
                entry.insert(new_id);
                new_id
            },
        }
    }
}

impl CmtIR {
    pub fn new() -> Self {
        let mut this = Self::default();
        this.add_entity(NONE.into());

        this.begin_region(None);
        this
    }

    pub fn hash_op(&mut self, op: OpId) -> Option<OpId> {
        self.hasher.replace(irony::FxHasherBuilder::default().build_hasher());
        let mut id_reducer = IdReducer::default();

        self.get_op(op).hash_with_reducer(self, &mut id_reducer);

        let hash_value = self.hasher.borrow_mut().finish();

        let parent = self.get_op(op).get_parent();

        // println!("hash_op op: {:#?}, parent: {:#?}, hash_value: {:#?}", op, parent, hash_value);

        let (deletion, final_op_id) =
            match self.op_hash_table.entry(OpHashT(parent, hash_value)) {
                std::collections::hash_map::Entry::Occupied(entry) => {
                    (true, Some(OpId::from(*entry.get())))
                },
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(op);
                    (false, Some(op))
                },
            };

        if deletion {
            self.delete_op(op);
        }

        final_op_id
    }
}

impl Drop for CmtIR {
    fn drop(&mut self) { self.end_region(); }
}

#[cfg(test)]
mod tests;
