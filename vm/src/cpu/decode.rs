use edu_asm_assembler::instruction_ident::inst::*;

use crate::{state::VmState, register_spec::WriteableReadable, immediate::RegisterImmediate};

use super::instruction::*;

#[derive(Clone, Copy)]
pub struct Decoder {
    pub instr: fn(&mut VmState, &[Box<dyn WriteableReadable>]),
    pub parameters: [RegisterImmediate; 8]
}

impl Decoder {
    fn decode(&mut self, state: &mut VmState, instr_bytes: [u8; 3]) {
        let function = match instr_bytes {
            ADD_T_S_INST => add_t_s,
            ADD_I_S_INST => add_i_s,
            ADD_T_U_INST => add_t_u,
           _ => panic!(),
        };
        self.instr = function;
    }
}
