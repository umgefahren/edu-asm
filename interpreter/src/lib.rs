use std::{collections::HashMap, rc::Rc};

use edu_asm_parser::{instruction::Instruction, label::LocAwLabel, PureElement};
use instruction::{transpile_instr, Executable};
use register::RegisterCollection;

pub(crate) mod behaviour;
pub(crate) mod instruction;
pub(crate) mod literal;
pub(crate) mod register;

pub(crate) struct Stack {
    inner: Vec<u64>,
}

impl Stack {
    pub(crate) fn push(&mut self, value: u64) {
        self.inner.push(value)
    }

    pub(crate) fn pop(&mut self) -> Option<u64> {
        self.inner.pop()
    }
}

pub(crate) struct State {
    registers: RegisterCollection,
    stack: Stack,
}

impl State {
    fn new() -> State {
        let stack = Stack { inner: Vec::new() };
        let registers = RegisterCollection::default();

        State { stack, registers }
    }
}

#[inline]
pub fn reduce_label_map(map: HashMap<String, Rc<LocAwLabel>>) -> HashMap<String, Rc<LocAwLabel>> {
    let mut ret = HashMap::with_capacity(map.capacity());

    let mut labels: Vec<LocAwLabel> = map.into_values().map(|e| (*e).clone()).collect();

    labels.sort_by_key(|e| e.loc);

    labels
        .iter()
        .enumerate()
        .map(|e| {
            let index = e.0;
            let decremented = e.1.loc - index;
            LocAwLabel {
                loc: decremented,
                name: e.1.name.clone(),
            }
        })
        .map(Rc::new)
        .for_each(|e| {
            ret.insert(e.name.clone(), e);
        });

    ret
}

#[inline]
pub fn update_pure_elements(
    map: HashMap<String, Rc<LocAwLabel>>,
    elements: Vec<PureElement>,
) -> Vec<PureElement> {
    elements
        .iter()
        .filter(|e| matches!(e, PureElement::Label(_)))
        .map(|e| match e {
            PureElement::Instruction(Instruction::ControlFlow(c)) => {
                let mut locale = c.clone();
                match locale.get_label() {
                    Some(d) => {
                        let loc_label = map.get(&d.content).cloned().unwrap();
                        locale.hydrate(loc_label);
                        PureElement::Instruction(Instruction::ControlFlow(locale))
                    }
                    None => PureElement::Instruction(Instruction::ControlFlow(locale)),
                }
            }
            _ => e.clone(),
        })
        .collect()
}

pub fn execute(elements: Vec<PureElement>, map: HashMap<String, Rc<LocAwLabel>>) {
    let map = reduce_label_map(map);
    let elements = update_pure_elements(map, elements);
    let elements: Vec<Box<dyn Executable>> = elements
        .iter()
        .map(|e| match e {
            PureElement::Instruction(d) => d,
            _ => panic!("FUUUUUUCK"),
        })
        .map(|e| transpile_instr(e.clone()))
        .collect();
    let mut state = State::new();
    loop {
        let index = state.registers.m.ins.inc();
        let element = elements.get(index).unwrap();
        element.execute(&mut state);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
