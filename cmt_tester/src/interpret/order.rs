use std::collections::HashMap;
use std::vec;

use num_bigint::BigUint;

// use super::structs::*;
use super::*;

#[derive(Debug, Default, Clone)]
pub struct Value {
    pub data_type: DataTypeEnum,
    pub value: BigUint,
}

pub trait DefUseTopo {
    fn get_defs(&self) -> Vec<usize>;
    fn get_uses(&self) -> Vec<usize>;
    fn initialize(&mut self) -> ();
    fn update(&mut self, new_use: usize) -> bool;
    fn ready(&self) -> bool;
}

#[derive(Debug, Default)]
pub struct ListenChannel {
    pub channels: Vec<usize>,
    pub values: Vec<usize>,
}

impl ListenChannel {
    pub fn new(channels: Vec<usize>, values: Vec<usize>) -> Self {
        Self { channels, values }
    }
}

impl DefUseTopo for ListenChannel {
    fn get_defs(&self) -> Vec<usize> { self.values.to_owned() }

    fn get_uses(&self) -> Vec<usize> { Vec::new() }

    fn initialize(&mut self) -> () {}

    fn update(&mut self, _new_use: usize) -> bool { true }

    fn ready(&self) -> bool { true }
}

#[derive(Debug, Default)]
pub struct WriteChannel {
    pub channels: Vec<usize>,
    pub values: Vec<usize>,
    pub status: Vec<bool>,
    pub waited_n: usize,
}

impl WriteChannel {
    pub fn new(channels: Vec<usize>, values: Vec<usize>) -> Self {
        Self { channels, values, ..Self::default() }
    }
}

impl DefUseTopo for WriteChannel {
    fn get_defs(&self) -> Vec<usize> { Vec::new() }

    fn get_uses(&self) -> Vec<usize> { self.values.to_owned() }

    fn initialize(&mut self) -> () {
        self.waited_n = self.values.len();
        self.status = vec![false; self.waited_n];
    }

    fn update(&mut self, new_use: usize) -> bool {
        let idx = self.values.iter().position(|&x| x == new_use);
        match idx {
            None => panic!("new_use {:?} not found in conds", new_use),
            Some(idx) => {
                assert!(idx < self.values.len());
                assert!(self.status[idx] == false);
                self.status[idx] = true;
                self.waited_n -= 1;
                self.waited_n == 0
            },
        }
    }

    fn ready(&self) -> bool { self.waited_n == 0 }
}

#[derive(Debug, Default)]
pub struct LdReg {
    pub regs: Vec<usize>,
}
impl LdReg {
    pub fn new(regs: Vec<usize>) -> Self { Self { regs } }
}
impl DefUseTopo for LdReg {
    fn get_defs(&self) -> Vec<usize> { self.regs.to_owned() }

    fn get_uses(&self) -> Vec<usize> { Vec::new() }

    fn initialize(&mut self) -> () {}

    fn update(&mut self, _new_use: usize) -> bool { true }

    fn ready(&self) -> bool { true }
}

#[derive(Debug, Default)]
pub struct StReg {
    pub values: Vec<usize>,
    pub regs: Vec<usize>,
    pub status: Vec<bool>,
    pub waited_n: usize,
}
impl StReg {
    pub fn new(values: Vec<usize>, regs: Vec<usize>) -> Self {
        Self { values, regs, ..Self::default() }
    }
}
impl DefUseTopo for StReg {
    fn get_defs(&self) -> Vec<usize> { Vec::new() }

    fn get_uses(&self) -> Vec<usize> { self.values.to_owned() }

    fn initialize(&mut self) -> () {
        self.waited_n = self.values.len();
        self.status = vec![false; self.waited_n];
    }

    fn update(&mut self, new_use: usize) -> bool {
        let idx = self.values.iter().position(|&x| x == new_use);
        match idx {
            None => panic!("new_use {:?} not found in conds", new_use),
            Some(idx) => {
                assert!(idx < self.values.len());
                assert!(self.status[idx] == false);
                self.status[idx] = true;
                self.waited_n -= 1;
                self.waited_n == 0
            },
        }
    }

    fn ready(&self) -> bool { self.waited_n == 0 }
}

#[derive(Debug, Default)]
pub struct Comb {
    pub uses: Vec<usize>,
    pub defs: Vec<usize>,
    pub op: OpEnum,
    pub status: Vec<bool>,
    pub waited_n: usize,
}

impl Comb {
    pub fn new(uses: Vec<usize>, defs: Vec<usize>, op: OpEnum) -> Self {
        Self { uses, defs, op, ..Self::default() }
    }
}

impl DefUseTopo for Comb {
    fn get_defs(&self) -> Vec<usize> { self.defs.to_owned() }

    fn get_uses(&self) -> Vec<usize> { self.uses.to_owned() }

    fn initialize(&mut self) -> () {
        self.waited_n = self.uses.len();
        self.status = vec![false; self.waited_n];
    }

    fn update(&mut self, new_use: usize) -> bool {
        let idx = self.uses.iter().position(|&x| x == new_use);
        match idx {
            None => panic!("new_use {:?} not found in conds", new_use),
            Some(idx) => {
                assert!(idx < self.uses.len());
                assert!(self.status[idx] == false);
                self.status[idx] = true;
                self.waited_n -= 1;
                self.waited_n == 0
            },
        }
    }

    fn ready(&self) -> bool { self.waited_n == 0 }
}

#[derive(Debug, Default)]
pub struct CondCheck {
    pub conds: Vec<usize>,
    pub result: usize,
    pub onehot: bool,
    pub has_default: bool,
    pub status: Vec<bool>,
    pub waited_n: usize,
}

impl CondCheck {
    pub fn new(
        conds: Vec<usize>, result: usize, onehot: bool, has_default: bool,
    ) -> Self {
        Self {
            conds,
            result,
            onehot,
            has_default,
            ..Self::default()
        }
    }
}

impl DefUseTopo for CondCheck {
    fn get_defs(&self) -> Vec<usize> { vec![self.result] }

    fn get_uses(&self) -> Vec<usize> { self.conds.to_owned() }

    fn initialize(&mut self) -> () {
        self.waited_n = self.conds.len();
        self.status = vec![false; self.waited_n];
    }

    fn update(&mut self, new_use: usize) -> bool {
        let idx = self.conds.iter().position(|&x| x == new_use);
        match idx {
            None => panic!("new_use {:?} not found in conds", new_use),
            Some(idx) => {
                assert!(idx < self.conds.len());
                assert!(self.status[idx] == false);
                self.status[idx] = true;
                self.waited_n -= 1;
                self.waited_n == 0
            },
        }
    }

    fn ready(&self) -> bool { self.waited_n == 0 }
}

#[derive(Debug, Default)]
pub struct SelectEvent {
    pub result: usize,
    pub idx: usize,
    pub values: Vec<usize>,
    pub has_idx: bool,
    pub status: Vec<bool>,
    pub waited_n: usize,
}

impl SelectEvent {
    pub fn new(result: usize, idx: usize, values: Vec<usize>) -> Self {
        Self { result, idx, values, ..Self::default() }
    }
}

impl DefUseTopo for SelectEvent {
    fn get_defs(&self) -> Vec<usize> { vec![self.result] }

    fn get_uses(&self) -> Vec<usize> { vec![self.idx].into_iter().chain(self.values.to_owned().into_iter()).collect() }

    fn initialize(&mut self) -> () {
        self.has_idx = false;
        self.waited_n = self.values.len();
        self.status = vec![false; self.waited_n];
    }

    fn update(&mut self, new_use: usize) -> bool {
        if new_use == self.idx {
            assert!(self.has_idx == false);
            self.has_idx = true;
        } else {
            let idx = self.values.iter().position(|&x| x == new_use);
            match idx {
                None => panic!("new_use {:?} not found in values and idx", new_use),
                Some(idx) => {
                    assert!(idx < self.values.len());
                    assert!(self.status[idx] == false);
                    self.status[idx] = true;
                    self.waited_n -= 1;
                },
            }
        }

        self.waited_n == 0      
    }

    fn ready(&self) -> bool { self.waited_n == 0 && self.has_idx}
}

#[derive(Debug)]
pub enum Event {
    ListenChannel(ListenChannel),
    WriteChannel(WriteChannel),
    LdReg(LdReg),
    StReg(StReg),
    Comb(Comb),
    CondCheck(CondCheck),
    Select(SelectEvent),
}

#[derive(Debug, Clone)]
pub struct SMSetting {
    pub inputs_from: Vec<usize>,
    pub outputs_to: Vec<usize>,
    pub condition: Condition,
}

#[derive(Debug, Default, Clone)]
pub enum Setting {
    #[default]
    Top,
    SubModule(SMSetting),
}

impl Setting {
    pub fn condition(&self) -> Condition {
        match self {
            Setting::Top => Condition::Must,
            Setting::SubModule(SMSetting { condition, .. }) => condition.to_owned(),
        }
    }
}
#[derive(Debug, Default)]
pub struct Reducer {
    hash_map: HashMap<EntityId, usize>,
    n: usize,
}

enum Reduced {
    New(usize),
    Old(usize),
}

impl Reduced {
    pub fn idx(self) -> usize {
        match self {
            Reduced::New(idx) => idx,
            Reduced::Old(idx) => idx,
        }
    }
}

impl Reducer {
    pub fn reduce(&mut self, entity: EntityId) -> Reduced {
        match self.hash_map.entry(entity) {
            std::collections::hash_map::Entry::Occupied(occupied) => Reduced::Old(*occupied.get()),
            std::collections::hash_map::Entry::Vacant(vacant) => {
                let new_id = self.n;
                self.n += 1;
                vacant.insert(new_id);
                Reduced::New(new_id)
            },
        }
    }

    pub fn reduce_with_target(&mut self, entity: EntityId, target: usize) -> Reduced {
        match self.hash_map.entry(entity) {
            std::collections::hash_map::Entry::Occupied(_) => {
                panic!("entity {:?} has already been reduced", entity)
            },
            std::collections::hash_map::Entry::Vacant(vacant) => Reduced::Old(*vacant.insert(target)),
        }
    }

    pub fn new_value(&mut self) -> usize {
        let new_id = self.n;
        self.n += 1;
        new_id
    }
}

#[derive(Debug, Default)]
pub struct Channel {
    pub data: Value,
}

impl Channel {
    pub fn new(value: Value) -> Self { Self { data: value } }
}

#[derive(Debug, Clone)]
pub enum Condition {
    Must,
    Never,
    Value(usize),
    And(Vec<Condition>),
    Or(Vec<Condition>),
    Not(Box<Condition>),
}

impl Condition {
    pub fn or(self, that: Condition) -> Self { 
        match self {
            Condition::Never => that,
            Condition::Must => Condition::Must,
            Condition::Or(mut vec) => {
                vec.push(that);
                Condition::Or(vec)
            },
            _ => Condition::Or(vec![self, that]),
        }
    }

    pub fn and(self, that: Condition) -> Self { Condition::And(vec![self, that]) }
}

#[derive(Default, Debug)]
pub struct Interpreter {
    pub values: Vec<Value>,
    pub value_conds: Vec<Condition>,
    pub events: Vec<Event>,
    pub event_conds: Vec<Condition>,

    pub reducer: Reducer,

    pub channels: Vec<Channel>,
}

impl Interpreter {
    pub fn from_hw_module(cmt: &CmtEnv, op_id: OpId) -> Self {
        let mut this = Self::default();
        match cmt.get_op(op_id) {
            OpEnum::HwModule(HwModule { top, .. }) => {
                assert!(
                    top.as_ref().expect("top module not found").0,
                    "must be top module"
                )
            },
            _ => {
                panic!("Expected HwModule")
            },
        }
        this.extend_with_hw_module(cmt, op_id, Setting::Top);
        this
    }

    pub fn extend_with_hw_module(
        &mut self, cmt: &CmtEnv, op_id: OpId, itfc: Setting,
    ) -> () {
        match cmt.get_op(op_id) {
            OpEnum::HwModule(HwModule { body, .. }) => {
                self.extend_with_region(cmt, body.as_ref().unwrap().to_owned(), itfc);
            },
            _ => {
                panic!("Expected HwModule")
            },
        }
    }

    pub fn extend_with_region(
        &mut self, cmt: &CmtEnv, region_id: RegionId, itfc: Setting,
    ) -> () {
        for op_id in cmt.get_region(region_id).op_children.to_owned() {
            cmt.get_op(op_id).to_interpret(self, cmt, &itfc, op_id);
        }
    }

    fn add_entity(&mut self, cmt: &CmtEnv, entity: EntityId, condition: Condition) -> usize {
        let reduced = self.reducer.reduce(entity.to_owned());
        match reduced {
            Reduced::New(idx) => {
                self.new_value(
                    cmt.get_entity(entity).get_dtype().to_owned().unwrap(),
                    condition,
                );
                idx
            },
            Reduced::Old(idx) => {
                self.value_conds[idx] = self.value_conds[idx].to_owned().or(condition);
                idx
            }
        }
    }

    fn add_entities(
        &mut self, cmt: &CmtEnv, entities: Vec<EntityId>, condition: Condition,
    ) -> Vec<usize> {
        let reduced = entities
            .iter()
            .map(|entity| {
                self.add_entity(cmt, entity.to_owned(), condition.to_owned())
            })
            .collect();
        reduced
    }

    fn add_entities_with_target(
        &mut self, cmt: &CmtEnv, entities: Vec<EntityId>, targets: Vec<usize>,
    ) -> () {
        assert!(entities.len() == targets.len());
        for (entity, target) in entities.iter().zip(targets) {
            let reduced = self.reducer.reduce_with_target(entity.to_owned(), target);
            assert!(
                self.values[reduced.idx()].data_type
                    == cmt.get_entity(entity.to_owned()).get_dtype().unwrap()
            );
        }
    }

    fn add_event(&mut self, event: Event) -> () { self.events.push(event); }

    fn add_channels(&mut self, values: Vec<usize>) -> Vec<usize> {
        values
            .into_iter()
            .map(|value_idx| {
                let channel_idx = self.channels.len();
                self.channels.push(Channel::new(self.values[value_idx].to_owned()));
                channel_idx
            })
            .collect()
    }

    fn add_listen_channel(&mut self, values: Vec<usize>) -> ListenChannel {
        let channels = self.add_channels(values.to_owned());
        let listen = ListenChannel::new(channels, values);
        listen
    }

    fn add_write_channel(&mut self, values: Vec<usize>) -> WriteChannel {
        let channels = self.add_channels(values.to_owned());
        let write = WriteChannel::new(channels, values);
        write
    }

    fn add_load_reg(&mut self, regs: Vec<usize>) -> LdReg { LdReg::new(regs) }

    fn add_store_reg(&mut self, values: Vec<usize>, regs: Vec<usize>) -> StReg {
        StReg::new(values, regs)
    }

    fn add_comb(&mut self, uses: Vec<usize>, defs: Vec<usize>, op: OpEnum) -> Comb {
        Comb::new(uses, defs, op)
    }

    fn add_comb_from_op(&mut self, cmt: &CmtEnv, op: OpEnum, condition: Condition) -> () {
        let mut uses = Vec::new();
        let mut defs = Vec::new();

        for (_, use_ids) in op.get_uses() {
            uses.append(
                use_ids.iter().filter_map(|x| x.to_owned()).collect::<Vec<_>>().as_mut(),
            )
        }
        for (_, def_ids) in op.get_defs() {
            defs.append(
                def_ids.iter().filter_map(|x| x.to_owned()).collect::<Vec<_>>().as_mut(),
            )
        }
        let uses = self.add_entities(cmt, uses, condition);
        let defs = self.add_entities(cmt, defs, Condition::Never);
        let event = Event::Comb(self.add_comb(uses, defs, op.to_owned()));
        self.add_event(event);
    }

    fn new_value(&mut self, data_type: DataTypeEnum, condition: Condition) -> usize {
        let new_value = self.reducer.new_value();
        self.values.push(Value { data_type, value: BigUint::default() });
        self.value_conds.push(condition);
        new_value
    }

    fn add_cond_check(
        &mut self, conds: Vec<usize>, onehot: bool, has_default: bool,
        condition: Condition,
    ) -> usize {
        let checked_value = self.new_value(DataTypeEnum::Void, condition);

        let event =
            Event::CondCheck(CondCheck::new(conds, checked_value, onehot, has_default));
        self.add_event(event);

        checked_value
    }

    
    // itprt.add_select(lhs, cond_checked_value, values, default);
    fn add_select(&mut self, lhs: usize, cond_checked_value: usize, mut values: Vec<usize>, default: Option<usize>) -> () {
        match default {
            Some(default) => values.push(default),
            _ => {},
        };

        let event = Event::Select(
            SelectEvent::new(lhs,  cond_checked_value, values)
        );
        self.add_event(event);
    }

}

trait ToInterpret {
    fn to_interpret(
        &self, itprt: &mut Interpreter, cmt: &CmtEnv, itfc: &Setting, op_id: OpId,
    ) -> ();
}

impl ToInterpret for OpEnum {
    fn to_interpret(
        &self, itprt: &mut Interpreter, cmt: &CmtEnv, itfc: &Setting, op_id: OpId,
    ) -> () {
        match self {
            OpEnum::HwInput(hw_input) => {
                hw_input.to_interpret(itprt, cmt, itfc, op_id);
            },
            OpEnum::HwOutput(hw_output) => {
                hw_output.to_interpret(itprt, cmt, itfc, op_id)
            },
            OpEnum::HwInstance(hw_instance) => {
                hw_instance.to_interpret(itprt, cmt, itfc, op_id)
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
                itprt.add_comb_from_op(cmt, op.to_owned(), itfc.condition());
            },

            OpEnum::Select(select) => {
                select.to_interpret(itprt, cmt, itfc, op_id);
            },

            _ => {
                panic!(
                    "Unsupported operation for interpret:\n\t{:#?}",
                    cmt.print_op(op_id)
                )
            },
        }
    }
}

impl ToInterpret for HwInput {
    fn to_interpret(
        &self, itprt: &mut Interpreter, cmt: &CmtEnv, itfc: &Setting, _op_id: OpId,
    ) -> () {
        let inputs = self.inputs.to_owned();

        match itfc.to_owned() {
            Setting::Top => {
                let inputs = itprt.add_entities(cmt, inputs, Condition::Must);
                let event = Event::ListenChannel(itprt.add_listen_channel(inputs));
                itprt.add_event(event);
            },
            Setting::SubModule(SMSetting { inputs_from, condition, .. }) => {
                assert!(matches!(condition, Condition::Must));
                itprt.add_entities_with_target(cmt, inputs, inputs_from);
            },
        }
    }
}

impl ToInterpret for HwOutput {
    fn to_interpret(
        &self, itprt: &mut Interpreter, cmt: &CmtEnv, itfc: &Setting, _op_id: OpId,
    ) -> () {
        let outputs = self.outputs.to_owned();

        match itfc.to_owned() {
            Setting::Top => {
                let outputs = itprt.add_entities(cmt, outputs, Condition::Must);
                let event = Event::WriteChannel(itprt.add_write_channel(outputs));
                itprt.add_event(event);
            },
            Setting::SubModule(SMSetting { outputs_to, condition, .. }) => {
                assert!(matches!(condition, Condition::Must));
                itprt.add_entities_with_target(cmt, outputs, outputs_to);
            },
        }
    }
}

impl ToInterpret for HwInstance {
    fn to_interpret(
        &self, itprt: &mut Interpreter, cmt: &CmtEnv, itfc: &Setting, _op_id: OpId,
    ) -> () {
        let target_op_id = self.target_op_id.to_owned().unwrap();
        let (inputs, outputs) = match itfc {
            Setting::Top => (
                itprt.add_entities(cmt, self.inputs.to_owned(), Condition::Must),
                itprt.add_entities(cmt, self.outputs.to_owned(), Condition::Must),
            ),
            Setting::SubModule(SMSetting { condition, .. }) => {
                assert!(matches!(condition, Condition::Must));
                (
                    itprt.add_entities(cmt, self.inputs.to_owned(), Condition::Must),
                    itprt.add_entities(cmt, self.outputs.to_owned(), Condition::Must),
                )
            },
        };
        let new_itfc = Setting::SubModule(SMSetting {
            inputs_from: inputs,
            outputs_to: outputs,
            condition: Condition::Must,
        });
        itprt.extend_with_hw_module(cmt, target_op_id.into(), new_itfc)
    }
}

impl ToInterpret for SeqCompReg {
    fn to_interpret(
        &self, itprt: &mut Interpreter, cmt: &CmtEnv, itfc: &Setting, _op_id: OpId,
    ) -> () {
        let SeqCompReg { input, output, reset, reset_val, .. } = self;
        assert!(matches!(reset.as_ref(), None), "don't support reset yet");
        assert!(matches!(reset_val.as_ref(), None), "don't support reset yet");
        let (Some(output), Some(input)) = (output.to_owned(), input.to_owned()) else {
            panic!("input and output must be provided for SeqCompReg");
        };

        if let Setting::SubModule(SMSetting { condition, .. }) = itfc {
            assert!(matches!(condition, Condition::Must));
        };

        let output = itprt.add_entities(cmt, vec![output], Condition::Must);
        let e_ld_reg = Event::LdReg(itprt.add_load_reg(output.to_owned()));
        itprt.add_event(e_ld_reg);

        let input = itprt.add_entities(cmt, vec![input], Condition::Must);
        let e_st_reg = Event::StReg(itprt.add_store_reg(input, output));
        itprt.add_event(e_st_reg);
    }
}

impl ToInterpret for Select {
    fn to_interpret(
        &self, itprt: &mut Interpreter, cmt: &CmtEnv, itfc: &Setting, op_id: OpId,
    ) -> () {
        let Select { lhs, conds, values, onehot, default, .. } = self;

        let condition = itfc.condition();
        let onehot = onehot.as_ref().unwrap().0;

        let conds = itprt.add_entities(cmt, conds.to_owned(), condition.to_owned());
        let has_default = default.is_some();
        let cond_checked_value =
            itprt.add_cond_check(conds.to_owned(), onehot, has_default, condition.to_owned());

        let default = match default {
            Some(default) => {
                let default = itprt.add_entities(cmt, vec![default.to_owned()], condition.to_owned());
                Some(default[0])
            },
            None => None,
        };
        
        let values = itprt.add_entities(cmt, values.to_owned(), condition);
        let lhs = itprt.add_entity(cmt, lhs.as_ref().unwrap().to_owned(), Condition::Never); 

        itprt.add_select(lhs, cond_checked_value, values, default);
    }
}
