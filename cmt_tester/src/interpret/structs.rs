use std::fmt::Debug;

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct ItprtClk(pub usize);

#[derive(Debug, Default)]
pub struct ItprtSig {
    pub op_id: OpId,
    pub name: String,
    pub arg_names: Vec<String>,
    pub arg_types: Vec<DataTypeEnum>,
    pub output_names: Vec<String>,
    pub output_types: Vec<DataTypeEnum>,
}

impl ItprtSig {
    pub fn new(
        op_id: OpId, name: &Option<StringAttr>, arg_names: &Option<ArrayAttr>,
        arg_types: &Option<ArrayAttr>, output_names: &Option<ArrayAttr>,
        output_types: &Option<ArrayAttr>,
    ) -> Self {
        match (name, arg_names, arg_types, output_names, output_types) {
            (
                Some(StringAttr(name)),
                Some(ArrayAttr(arg_names)),
                Some(ArrayAttr(arg_types)),
                Some(ArrayAttr(output_names)),
                Some(ArrayAttr(output_types)),
            ) => {
                let arg_names = arg_names
                    .iter()
                    .map(|arg_name| match arg_name {
                        AttributeEnum::StringAttr(StringAttr(arg_name)) => {
                            arg_name.clone()
                        },
                        _ => panic!("arg_names should be StringAttr"),
                    })
                    .collect::<Vec<_>>();
                let arg_types = arg_types
                    .iter()
                    .map(|arg_type| match arg_type {
                        AttributeEnum::TypeAttr(TypeAttr(data_type)) => data_type.clone(),
                        _ => panic!("arg_types should be DataTypeAttr"),
                    })
                    .collect::<Vec<_>>();
                let output_names = output_names
                    .iter()
                    .map(|arg_name| match arg_name {
                        AttributeEnum::StringAttr(StringAttr(arg_name)) => {
                            arg_name.clone()
                        },
                        _ => panic!("arg_names should be StringAttr"),
                    })
                    .collect::<Vec<_>>();
                let output_types = output_types
                    .iter()
                    .map(|arg_type| match arg_type {
                        AttributeEnum::TypeAttr(TypeAttr(data_type)) => data_type.clone(),
                        _ => panic!("arg_types should be DataTypeAttr"),
                    })
                    .collect::<Vec<_>>();

                Self {
                    op_id,
                    name: name.clone(),
                    arg_names,
                    arg_types,
                    output_names,
                    output_types,
                }
            },
            _ => panic!("ItprtSig should be built from StringAttr and ArrayAttr"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ItprtWireId(pub usize);

#[derive(Debug, Clone)]
pub struct ItprtWire {
    pub empty: bool,
    pub idx: usize,
    pub data_type: DataTypeEnum,
}

impl ItprtWire {
    pub fn default() -> Self {
        Self {
            empty: true,
            idx: 0,
            data_type: DataTypeEnum::UInt(0.into()),
        }
    }

    pub fn new(data_type: DataTypeEnum, idx: usize) -> Self {
        Self { empty: false, idx, data_type }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ItprtReg {
    pub op_id: OpId,
    pub input: ItprtWireId,
    pub output: ItprtWireId,
    pub clk: ItprtClk,
}

impl ItprtReg {
    pub fn new(
        op_id: OpId, input: Option<EntityId>, output: Option<EntityId>,
        clk: Option<EntityId>,
    ) -> Self {
        match (input, output, clk) {
            (Some(input), Some(output), Some(clk)) => {
                let input = ItprtWireId(input.id());
                let output = ItprtWireId(output.id());
                let clk = ItprtClk(clk.id());
                Self { op_id, input, output, clk }
            },
            _ => panic!("ItprtReg should be built from EntityId"),
        }
    }
}

#[derive(Debug)]
pub struct ItprtInst {
    pub name: String,
    pub target_mod: ItprtMod,
    pub inputs: Vec<ItprtWireId>,
    pub outputs: Vec<ItprtWireId>,
}

impl ItprtInst {
    pub fn new(
        name: Option<StringAttr>, target_mod: ItprtMod, inputs: Vec<EntityId>,
        outputs: Vec<EntityId>,
    ) -> Self {
        match name {
            Some(StringAttr(name)) => {
                let inputs =
                    inputs.iter().map(|input| ItprtWireId(input.0)).collect::<Vec<_>>();
                let outputs = outputs
                    .iter()
                    .map(|output| ItprtWireId(output.0))
                    . collect::<Vec<_>>();
                Self {
                    name: name.clone(),
                    target_mod,
                    inputs,
                    outputs,
                }
            },
            None => panic!("ItprtInst should be built from StringAttr"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ItprtGuard {
    None,
    Wire(ItprtWireId),
    And(Vec<ItprtGuard>),
    Or(Vec<ItprtGuard>),
    Not(Box<ItprtGuard>),
}

impl ItprtGuard {
    pub fn or(self, that: Self) -> Self { ItprtGuard::Or(vec![self, that]) }

    pub fn and(self, that: Self) -> Self { ItprtGuard::And(vec![self, that]) }
}

#[derive(Debug)]
pub struct ItprtOp {
    idx: usize,
    op: OpEnum, // with reduced def/use id
}

impl ItprtOp {
    pub fn new(idx: usize, op: OpEnum) -> Self { Self { idx, op } }
}

#[derive(Default)]
pub struct Dependency {
    pub max_from_idx: usize,
    pub dependency: Vec<Vec<usize>>,
}
impl Debug for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dependency")
            .field("dependency", &self.dependency)
            .finish()
    }
}

impl Dependency {
    pub fn add(&mut self, from: usize, to: usize) {
        if from > self.max_from_idx {
            self.max_from_idx = from;
            self.dependency.resize_with(from + 1, Vec::new);
        }
        self.dependency[from].push(to);
    }
}

#[derive(Default)]
pub struct ItprtMod {
    pub sig: ItprtSig,
    pub inputs: Vec<ItprtWireId>,
    pub outputs: Vec<ItprtWireId>,
    pub instances: Vec<ItprtInst>,
    pub registers: Vec<ItprtReg>,
    pub wires: Vec<ItprtWire>,
    pub ops: Vec<ItprtOp>,

    pub dependency: Dependency,

    pub wire_guards: Vec<ItprtGuard>,
}

impl Debug for ItprtMod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ItprtMod")
            .field("sig", &self.sig)
            .field("inputs", &self.inputs)
            .field("outputs", &self.outputs)
            .field("instances", &self.instances)
            .field("registers", &self.registers)
            .field("ops", &self.ops)
            .field("dependency", &self.dependency)
            .field("wires", &self.wires)
            .field("wire_guards", &self.wire_guards)
            .finish()
    }
}

impl ItprtMod {
    pub fn from_cmt(env: &CmtEnv, op_id: OpId) -> Self {
        // println!("try to create ItprtMod from {:?}", op_id);

        let op = env.get_op(op_id);
        let mut reducer = IdReducer::default();
        match op {
            OpEnum::HwModule(HwModule {
                id,
                name,
                arg_names,
                arg_types,
                output_names,
                output_types,
                body,
                ..
            }) => {
                let sig = ItprtSig::new(
                    OpId(*id),
                    name,
                    arg_names,
                    arg_types,
                    output_names,
                    output_types,
                );

                let mut this = ItprtMod { sig, ..ItprtMod::default() };

                this.explore(env, body.unwrap(), &mut reducer, ItprtGuard::None);

                this
            },
            _ => {
                panic!(
                    "ItprtMod can only be built from Module op, op {:?} is {:?}",
                    op_id,
                    op.get_op_name()
                )
            },
        }
    }

    pub fn add_wire(
        &mut self, entity_id: EntityId, data_type: DataTypeEnum, guard: ItprtGuard,
    ) {
        println!("\t\tadd wire with id {}", entity_id.id());
        if matches!(data_type, DataTypeEnum::Clk(_)) {
            return;
        }

        let reduced = entity_id.id();
        if reduced >= self.wires.len() {
            self.wires.resize(reduced + 1, ItprtWire::default());
            self.wires[reduced] = ItprtWire::new(data_type, reduced);
            self.wire_guards.resize(reduced + 1, ItprtGuard::None);
            self.wire_guards[reduced] = guard;
        } else {
            if self.wires[reduced].empty {
                self.wires[reduced] = ItprtWire::new(data_type, reduced);
                self.wire_guards[reduced] = guard;
            } else {
                self.wire_guards[reduced] =
                    self.wire_guards[reduced].to_owned().or(guard);
            }
        }
    }

    pub fn add_wires(
        &mut self, entity_id: Vec<EntityId>, data_type: Vec<DataTypeEnum>,
        guard: ItprtGuard,
    ) {
        for (entity_id, data_type) in entity_id.into_iter().zip(data_type.into_iter()) {
            self.add_wire(entity_id, data_type, guard.to_owned());
        }
    }

    fn explore(
        &mut self, env: &CmtEnv, region: RegionId, reducer: &mut IdReducer,
        guard: ItprtGuard,
    ) {
        println!("explore region {:?}", region);
        for op_id in env.get_region(region).get_op_children() {
            println!("\texplore op: {:?}", env.print_op(op_id));
            match env.get_op(op_id).to_owned() {
                OpEnum::HwInstance(hw_instantce) => {
                    let HwInstance { id, outputs, inputs, target_op_id, name, .. } =
                        hw_instantce.to_owned();
                    match env.get_region(region).get_use(env) {
                        Some(op) if op == self.sig.op_id => {},
                        _ => {
                            panic!("HwInstance has to be placed in the top-level region of the module;\n However, {:?} appears in Region {:?}", env.print_op(OpId(id)), region);
                        },
                    }

                    let target_op_id = match target_op_id {
                        Some(OpIdAttr(id)) => id,
                        None => panic!("HwInstance {:?} has no target_id", id),
                    };

                    let target_mod = ItprtMod::from_cmt(env, target_op_id.into());

                    let inst =
                        ItprtInst::new(name, target_mod, inputs.to_owned(), outputs);

                    self.instances.push(inst);

                    let data_types: Vec<_> = env
                        .get_entities(&inputs)
                        .into_iter()
                        .map(|x| x.get_dtype().unwrap())
                        .collect();
                    let HwInstance { inputs, .. } = hw_instantce.reduce_def_use(reducer);

                    self.add_wires(inputs, data_types, guard.to_owned());
                },
                OpEnum::HwInput(hw_input) => {
                    let HwInput { inputs, .. } = hw_input.reduce_def_use(reducer);
                    if self.inputs.is_empty() {
                        for input in inputs {
                            let wire_id = ItprtWireId(input.id());
                            self.inputs.push(wire_id);
                        }
                    } else {
                        panic!("multiple HwInput in region {:?}", region);
                    }
                },
                OpEnum::HwOutput(hw_output) => {
                    let HwOutput { outputs, .. } = hw_output.reduce_def_use(reducer);
                    if self.outputs.is_empty() {
                        for output in outputs.to_owned() {
                            let wire_id = ItprtWireId(output.id());
                            self.outputs.push(wire_id);
                        }

                        let data_types = env
                            .get_entities(&outputs)
                            .into_iter()
                            .map(|x| x.get_dtype().unwrap())
                            .collect();

                        self.add_wires(outputs, data_types, guard.to_owned())
                    } else {
                        panic!("multiple HwOutput in region {:?}", region);
                    }
                },

                OpEnum::SeqCompReg(seq_comp_reg) => {
                    let SeqCompReg { input, reset, reset_val, .. } =
                        seq_comp_reg.to_owned();
                    assert!(matches!(reset, None), "don't support reset yet");
                    assert!(matches!(reset_val, None), "don't support reset yet");

                    let data_type = env.get_entity(input.unwrap()).get_dtype().unwrap();

                    let SeqCompReg { id, output, input, clk, .. } =
                        seq_comp_reg.reduce_def_use(reducer);

                    self.registers.push(ItprtReg::new(OpId(id), input, output, clk));

                    self.add_wire(input.unwrap(), data_type, guard.to_owned());
                },
                op @ (OpEnum::Assign(_)
                | OpEnum::HwBitCast(_)
                | OpEnum::HwConstant(_)
                | OpEnum::HwAggregateConstant(_)
                | OpEnum::HwArrayConcat(_)
                | OpEnum::HwArrayCreate(_)
                | OpEnum::HwArrayGet(_)
                | OpEnum::HwArraySlice(_)
                | OpEnum::HwStructCreate(_)
                | OpEnum::HwStructExplode(_)
                | OpEnum::HwStructExtract(_)
                | OpEnum::HwStructInject(_)
                | OpEnum::CombVariadic(_)
                | OpEnum::CombBinary(_)
                | OpEnum::CombICmp(_)
                | OpEnum::CombMux2(_)
                | OpEnum::CombUnary(_)) => {
                    let mut data_types = Vec::new();
                    for (_, use_ids) in op.get_uses() {
                        data_types.append(
                            use_ids
                                .iter()
                                .filter_map(|x| match x {
                                    Some(entity_id) => Some(
                                        env.get_entity(*entity_id).get_dtype().unwrap(),
                                    ),
                                    None => None,
                                })
                                .collect::<Vec<_>>()
                                .as_mut(),
                        )
                    }

                    let op = op.reduce_def_use(reducer);

                    self.ops.push(ItprtOp::new(self.ops.len(), op.to_owned()));
                    let mut uses = Vec::new();

                    for (_, use_id) in op.get_uses() {
                        uses.append(
                            use_id
                                .iter()
                                .filter_map(|x| x.to_owned())
                                .collect::<Vec<_>>()
                                .as_mut(),
                        );
                    }
                    self.add_wires(uses, data_types, guard.to_owned())
                },

                OpEnum::Select(select) => {
                    let Select { values, default, .. } = select.to_owned();

                    let data_types = values
                        .iter()
                        .map(|value| env.get_entity(*value).get_dtype().unwrap())
                        .collect::<Vec<_>>();

                    let default_data_type =
                        default.map(|x| env.get_entity(x).get_dtype().unwrap());

                    let select = select.reduce_def_use(reducer);
                    let Select { id, default, conds, onehot, values, .. } =
                        select.to_owned();

                    let cond_check = ItprtCondCheck::new(
                        conds.to_owned(),
                        Some(default.is_some().into()),
                        Some(match onehot {
                            Some(BoolAttr(onehot)) => onehot.to_owned().into(),
                            None => panic!("Select {:?} has no onehot attribute", id),
                        }),
                    );

                    self.ops.push(ItprtOp::new(self.ops.len(), cond_check.into()));

                    self.ops.push(ItprtOp::new(self.ops.len(), select.into()));

                    self.dependency.add(self.ops.len() - 2, self.ops.len() - 1);

                    let onehot = match onehot {
                        Some(BoolAttr(onehot)) => onehot.to_owned(),
                        None => panic!("Select {:?} has no onehot attribute", id),
                    };

                    if onehot {
                        for (cond, (value, data_type)) in
                            conds.iter().zip(values.iter().zip(data_types.iter()))
                        {
                            self.add_wire(
                                value.to_owned(),
                                data_type.to_owned(),
                                guard
                                    .to_owned()
                                    .and(ItprtGuard::Wire(ItprtWireId(cond.id()))),
                            );
                        }

                        if let Some(default) = default {
                            self.add_wire(
                                default,
                                default_data_type.unwrap(),
                                guard.to_owned().and(ItprtGuard::Not(Box::new(
                                    ItprtGuard::Or(
                                        conds
                                            .iter()
                                            .map(|cond| {
                                                ItprtGuard::Wire(ItprtWireId(cond.id()))
                                            })
                                            .collect(),
                                    ),
                                ))),
                            );
                        }
                    } else {
                        unimplemented!("non-onehot select")
                    }
                },

                OpEnum::Cases(case) => {
                    let case = case.reduce_def_use(reducer);

                    let Cases { id, conds, onehot, dflt, bodies, .. } = case.to_owned();

                    let cond_check = ItprtCondCheck::new(
                        conds.to_owned(),
                        Some(dflt.is_some().into()),
                        Some(match onehot {
                            Some(BoolAttr(onehot)) => onehot.to_owned().into(),
                            None => panic!("Cases {:?} has no onehot attribute", id),
                        }),
                    );

                    self.ops.push(ItprtOp::new(self.ops.len(), cond_check.into()));

                    self.ops.push(ItprtOp::new(self.ops.len(), case.to_owned().into()));

                    self.dependency.add(self.ops.len() - 2, self.ops.len() - 1);

                    let onehot = match onehot {
                        Some(BoolAttr(onehot)) => onehot.to_owned(),
                        None => panic!("Cases {:?} has no onehot attribute", id),
                    };

                    if onehot {
                        for (cond, body) in conds.iter().zip(bodies.iter()) {
                            self.explore(
                                env,
                                body.to_owned(),
                                reducer,
                                guard
                                    .to_owned()
                                    .and(ItprtGuard::Wire(ItprtWireId(cond.id()))),
                            );
                        }

                        if let Some(dflt) = dflt {
                            self.explore(
                                env,
                                dflt,
                                reducer,
                                guard.to_owned().and(ItprtGuard::Not(Box::new(
                                    ItprtGuard::Or(
                                        conds
                                            .iter()
                                            .map(|cond| {
                                                ItprtGuard::Wire(ItprtWireId(cond.id()))
                                            })
                                            .collect(),
                                    ),
                                ))),
                            );
                        }
                    } else {
                        unimplemented!("non-onehot cases")
                    }
                },
                _ => {
                    panic!("unsupported operation: {:?}", env.print_op(op_id));
                },
            }
        }
    }
}
