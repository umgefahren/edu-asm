use edu_asm_parser::instruction::Misc;

use crate::{
    behaviour::{Readable, Writeable},
    register::RegisterSpecifier,
};

use super::{Executable, RegOrLit};

pub(crate) struct Halt {}

impl Executable for Halt {
    fn execute(&self, _: &mut crate::State) {
        println!("\n EXECUTION HALTED INDEFINITLY");
        loop {
            std::thread::sleep(std::time::Duration::from_secs(60));
        }
    }
}

pub(crate) struct Exit<S: Readable> {
    s: S,
}

impl<S: Readable> Executable for Exit<S> {
    fn execute(&self, state: &mut crate::State) {
        let exit_value = self.s.get_signed(state);
        std::process::exit(exit_value as i32);
    }
}

pub(crate) struct Print<S: Readable> {
    s: S,
}

impl<S: Readable> Executable for Print<S> {
    fn execute(&self, state: &mut crate::State) {
        let value = self.s.get_unsigned(state);
        let value_u8 = value as u8;
        let value_char = value_u8 as char;
        print!("{}", value_char);
    }
}

pub(crate) struct Read<S: Writeable> {
    s: S,
}

impl<S: Writeable> Executable for Read<S> {
    fn execute(&self, state: &mut crate::State) {
        use std::io::Read;

        let mut inp = std::io::stdin();
        let mut buf = [0u8; 1];
        inp.read_exact(&mut buf).unwrap();
        let buf_val = buf[0] as u64;
        self.s.set_unsigned(state, buf_val);
    }
}

pub(crate) struct Nop {}

impl Executable for Nop {
    fn execute(&self, _: &mut crate::State) {}
}

pub(super) fn transpile_misc(instr: Misc) -> Box<dyn Executable> {
    match instr {
        Misc::Halt => Box::new(Halt {}),
        Misc::Exit { s } => {
            let s = RegOrLit::from(s);
            Box::new(Exit { s })
        }
        Misc::Print { s } => {
            let s = RegOrLit::from(s);
            Box::new(Print { s })
        }
        Misc::Read { s } => {
            let s = RegisterSpecifier::from(s);
            Box::new(Read { s })
        }
        Misc::Nop => Box::new(Nop {}),
        _ => unimplemented!("{:?} is unimplemented", instr),
    }
}
