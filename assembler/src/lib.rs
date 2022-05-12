pub mod immediate;
pub mod instruction;
pub mod instruction_ident;
pub mod mode;
pub mod register;

use std::{collections::HashMap, rc::Rc};

use edu_asm_parser::{instruction::Instruction, label::LocAwLabel, PureElement};
use instruction::encode_instruction;

#[inline]
pub fn update_pure_elements(
    map: HashMap<String, Rc<LocAwLabel>>,
    elements: Vec<PureElement>,
) -> Vec<PureElement> {
    elements
        .iter()
        .filter(|e| !matches!(e, PureElement::Label(_)))
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

pub fn assemble(elements: Vec<PureElement>) -> Vec<u8> {
    let mut byte_counter = 0;
    let mut label_maps: HashMap<String, Rc<LocAwLabel>> = HashMap::new();
    for element in elements.iter() {
        match element {
            PureElement::Label(l) => {
                let label = Rc::new(LocAwLabel {
                    name: l.name.clone(),
                    loc: byte_counter,
                });
                label_maps.insert(l.name.clone(), label);
            }
            PureElement::Instruction(i) => {
                let instruction_bytes = encode_instruction(i.clone());
                byte_counter += instruction_bytes.len();
            }
        }
    }
    let elements = update_pure_elements(label_maps, elements);
    let mut ret = Vec::with_capacity(byte_counter);
    elements
        .iter()
        .map(|e| match e {
            PureElement::Instruction(i) => encode_instruction(i.clone()),
            _ => vec![],
        })
        .for_each(|mut e| ret.append(&mut e));
    ret
}

#[cfg(test)]
mod tests {
    use edu_asm_parser::parse;

    use crate::assemble;

    #[test]
    fn it_works() {
        let program = include_str!("../../examples/basic.edu");
        let (parsed, _) = parse(program).unwrap();
        println!("{:?}", parsed);
        let assembled = assemble(parsed);
        println!("{:?}", assembled);
    }
}
