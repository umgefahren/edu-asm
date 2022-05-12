use crate::State;

pub(crate) trait Readable {
    fn get_unsigned(&self, state: &State) -> u64;
    fn get_signed(&self, state: &State) -> i64;
}

pub(crate) trait Writeable {
    fn set_unsigned(&self, state: &mut State, val: u64);
    fn set_signed(&self, state: &mut State, val: i64);
}
