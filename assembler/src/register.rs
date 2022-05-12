use edu_asm_parser::register::RegisterToken;

/// `$G_0` register code (0b000 in binary) (0 in decimal)
pub const G_0_CODE: u8 = 0b0;
/// `$G_1` register code (0b001 in binary) (1 in decimal)
pub const G_1_CODE: u8 = 0b1;
/// `$G_2` register code (0b010 in binary) (2 in decimal)
pub const G_2_CODE: u8 = 0b10;
/// `$G_3` register code (0b011 in binary) (3 in decimal)
pub const G_3_CODE: u8 = 0b11;
/// `$G_4` register code (0b100 in binary) (4 in decimal)
pub const G_4_CODE: u8 = 0b100;
/// `$G_5` register code (0b101 in binary) (5 in decimal)
pub const G_5_CODE: u8 = 0b101;
/// `$G_6` register code (0b110 in binary) (6 in decimal)
pub const G_6_CODE: u8 = 0b110;
/// `$G_7` register code (0b111 in binary) (7 in decimal)
pub const G_7_CODE: u8 = 0b111;

/// $S_B register code (0b1000 in binary) (8 in decimal)
pub const S_B_CODE: u8 = 0b1000;
/// $S_E register code (0b1001 in binary) (9 in decimal)
pub const S_E_CODE: u8 = 0b1001;

/// $R register code (0b10000 in binary) (16 in decimal)
pub const R_CODE: u8 = 0b10000;
/// $I register code (0b10001 in binary) (17 in decimal)
pub const I_CODE: u8 = 0b10001;
/// $Z register code (0b10010 in binary) (18 in decimal)
pub const Z_CODE: u8 = 0b10010;
/// $E register code (0b10011 in binary) (19 in decimal)
pub const E_CODE: u8 = 0b10011;

pub fn encode_register_token(token: &RegisterToken) -> u8 {
    match token {
        RegisterToken::GeneralPurpose(0) => G_0_CODE,
        RegisterToken::GeneralPurpose(1) => G_1_CODE,
        RegisterToken::GeneralPurpose(2) => G_2_CODE,
        RegisterToken::GeneralPurpose(3) => G_3_CODE,
        RegisterToken::GeneralPurpose(4) => G_4_CODE,
        RegisterToken::GeneralPurpose(5) => G_5_CODE,
        RegisterToken::GeneralPurpose(6) => G_6_CODE,
        RegisterToken::GeneralPurpose(7) => G_7_CODE,
        RegisterToken::GeneralPurpose(_) => panic!("Invalid general purpose register"),
        RegisterToken::StackBase => S_B_CODE,
        RegisterToken::StackEnd => S_E_CODE,
        RegisterToken::Return => R_CODE,
        RegisterToken::Instruction => I_CODE,
        RegisterToken::Zero => Z_CODE,
        RegisterToken::Error => E_CODE,
    }
}
