pub mod state;
pub mod register;
pub mod settings;
pub mod register_spec;
pub mod memory;
pub mod cpu;
pub mod immediate;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
