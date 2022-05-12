use std::{collections::HashMap, rc::Rc, str::FromStr};

use comment::strip_coment;
use instruction::{
    ArithmeticBase, ArithmeticMultDivEasy, ControlFlow, Instruction, InstructionParseError, Memory,
    Misc,
};
use label::{LabelToken, LocAwLabel};
use thiserror::Error;

pub mod comment;
pub mod instruction;
pub mod label;
pub mod label_ref;
pub mod literal;
pub mod register;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PureElement {
    Instruction(Instruction),
    Label(Rc<LocAwLabel>),
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(
        "while parsing instruction `{2}` at line `{1}`, an instruction parse error occured: `{0}`"
    )]
    InstructionParseError(InstructionParseError, usize, String),
}

fn preprocess_input(inp: &str) -> Vec<(usize, &str)> {
    inp.split('\n')
        .enumerate()
        .map(|e| (e.0, strip_coment(e.1)))
        .map(|e| (e.0, e.1.trim()))
        .filter(|e| !std::primitive::str::is_empty(e.1))
        .collect()
}

fn collect_labels(
    lines: &[(usize, &str)],
) -> (HashMap<String, Rc<LocAwLabel>>, HashMap<usize, String>) {
    let mut ret = HashMap::new();
    let mut ret_set = HashMap::new();
    lines.iter().enumerate().for_each(|e| {
        let index = e.0;
        let line = e.1 .1;

        if let Ok(d) = LabelToken::from_str(line) {
            let loc_aw_label = LocAwLabel::new(d.content.clone(), index);
            let rc_aw_label = Rc::new(loc_aw_label);
            ret.insert(d.content.clone(), rc_aw_label);
            ret_set.insert(index, d.content.clone());
        }
    });

    (ret, ret_set)
}

fn parse_instruction(
    inp: &str,
    labels: &HashMap<String, Rc<LocAwLabel>>,
) -> Result<Instruction, InstructionParseError> {
    let arithmetic_base_result = ArithmeticBase::from_str(inp).map(Instruction::ArithmeticBase);
    if arithmetic_base_result.is_ok() {
        return arithmetic_base_result;
    }
    let arithmetic_base_error = arithmetic_base_result.unwrap_err();
    if !arithmetic_base_error.is_unknown_instruction() {
        return Err(arithmetic_base_error);
    }
    let arithmetic_mult_div_result =
        ArithmeticMultDivEasy::from_str(inp).map(Instruction::ArithmeticMultDivEasy);
    if arithmetic_mult_div_result.is_ok() {
        return arithmetic_mult_div_result;
    }
    let arithmetic_mult_div_error = arithmetic_mult_div_result.unwrap_err();
    if !arithmetic_mult_div_error.is_unknown_instruction() {
        return Err(arithmetic_mult_div_error);
    }
    let control_flow_result = ControlFlow::from_str(inp);
    if control_flow_result.is_ok() {
        let mut control_flow_instruction = control_flow_result.unwrap();
        let control_flow_label_opt = control_flow_instruction.get_label();
        if control_flow_label_opt.is_none() {
            return Ok(Instruction::ControlFlow(control_flow_instruction));
        }
        let control_flow_label = control_flow_label_opt.unwrap();
        match labels.get(&control_flow_label.content) {
            None => {
                return Err(InstructionParseError::UnknownLabel(
                    control_flow_label.content.clone(),
                    inp.to_string(),
                ));
            }
            Some(d) => {
                control_flow_instruction.hydrate(d.clone());
                return Ok(Instruction::ControlFlow(control_flow_instruction));
            }
        }
    }

    let control_flow_error = control_flow_result.unwrap_err();
    if !control_flow_error.is_unknown_instruction() {
        return Err(control_flow_error);
    }
    let memory_result = Memory::from_str(inp).map(Instruction::Memory);
    if memory_result.is_ok() {
        return memory_result;
    }
    let memory_error = memory_result.unwrap_err();
    if !memory_error.is_unknown_instruction() {
        return Err(memory_error);
    }
    Misc::from_str(inp).map(Instruction::Misc)
}

pub fn parse(
    input: &str,
) -> Result<(Vec<PureElement>, HashMap<String, Rc<LocAwLabel>>), ParseError> {
    let lines = preprocess_input(input);
    let (labels, labels_locs) = collect_labels(&lines);
    let mut ret = Vec::with_capacity(lines.len());

    for (clean_index, (input_index, input_line)) in lines.iter().enumerate() {
        if !labels_locs.contains_key(&clean_index) {
            match parse_instruction(input_line, &labels) {
                Ok(d) => {
                    ret.push(PureElement::Instruction(d));
                }
                Err(e) => {
                    return Err(ParseError::InstructionParseError(
                        e,
                        *input_index,
                        input_line.to_string(),
                    ));
                }
            }
        } else {
            let associated_name = labels_locs.get(&clean_index).unwrap();
            let associated_label = labels.get(associated_name).cloned().unwrap();
            ret.push(PureElement::Label(associated_label));
        }
    }

    Ok((ret, labels))
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn it_works() {
        const DEMO_FILE: &str = r#"
        main:
            addts $G_0 $Z 10u
            jmp :main
        "#;

        let (stream, labels) = parse(DEMO_FILE).unwrap();
        println!("{:#?}", stream);
    }
}
