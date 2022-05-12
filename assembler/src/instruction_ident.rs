/// Setting the group size
pub const GROUP_SIZE: u32 = 4;
/// Setting the type info size
pub const TYPE_SIZE: u32 = 4;
/// Setting the instruction code size
pub const INSTRUCTION_CODE_SIZE: u32 = 16;

const MAX_GROUP: u8 = 2u8.pow(GROUP_SIZE);

const MAX_TYPE: u8 = 2u8.pow(TYPE_SIZE);

/// Identifies the instructino in a binary format, in an abstract way
///
/// | **Section Name**        | Instruction Code           | Type Info      | Group              |
/// |-------------------------|----------------------------|----------------|--------------------|
/// | **Section Size** (bits) | 16                         | 4              | 4                  |
/// | **Section Description** | Identifies the instruction | Hints the type | Declares the group |
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstructionIdent {
    pub instruction_code: u16,
    pub type_info: u8,
    pub group: u8,
}

impl InstructionIdent {
    pub const fn new(instruction_code: u16, type_info: u8, group: u8) -> Self {
        assert!(type_info <= MAX_TYPE);
        assert!(group <= MAX_GROUP);
        Self {
            instruction_code,
            type_info,
            group,
        }
    }

    pub const fn encode(&self) -> [u8; 3] {
        let instruction_code_bytes = self.instruction_code.to_le_bytes();
        let (shifted_type_info, overflew) = self.type_info.overflowing_shl(GROUP_SIZE);
        assert!(!overflew);
        let compound_type_info_group = shifted_type_info | self.group;
        let compound_bytes = compound_type_info_group.to_le_bytes();
        [
            instruction_code_bytes[0],
            instruction_code_bytes[1],
            compound_bytes[0],
        ]
    }

    pub const fn decode(inp: [u8; 3]) -> Self {
        const GROUP_INFO_MASK: u8 = 0b00001111;

        let instruction_code = u16::from_le_bytes([inp[0], inp[1]]);
        let compound_type_info_group = u8::from_le_bytes([inp[2]]);
        let group = GROUP_INFO_MASK & compound_type_info_group;
        let type_info = compound_type_info_group >> GROUP_SIZE;
        Self {
            instruction_code,
            group,
            type_info,
        }
    }
}

const fn encode_instruction(instruction_code: u16, type_info: u8, group: u8) -> [u8; 3] {
    InstructionIdent::new(instruction_code, type_info, group).encode()
}

/// Group identifier of the arithmetic base instructions
pub const ARITHMETIC_BASE_GROUP: u8 = 1;
/// Group identifier of the arithmetic shift instructions
pub const ARITHMETIC_SHIFT_GROUP: u8 = 2;
/// Group identifier of the arithmetic bitwise logic instructions
pub const ARITHMETIC_BIT_LOG_GROUP: u8 = 3;
/// Group identifier of the arithmetic mult/div instructions
pub const ARITHMETIC_MULT_DIV_EASY_GROUP: u8 = 4;
/// Group identifier of the jump instructions
pub const JUMP_GROUP: u8 = 5;
/// Group identifier of the function instructions
pub const FUNCTION_GROUP: u8 = 6;
/// Group identifier of the memory move instructions
pub const MEMORY_GROUP: u8 = 7;
/// Group identiier of the stack instructions
pub const STACK_GROUP: u8 = 8;
/// Group identifier of the misc instructions
pub const MISC_GROUP: u8 = 9;

/// Type idenfifier for untyped instructions;
pub const UNTYPED_TYPE: u8 = 0;
/// Type identifier for signed instructions
pub const SIGNED_TYPE_INFO: u8 = 1;
/// Type identifier for unsigned instructions
pub const UNSIGNED_TYPE_INFO: u8 = 2;
/// Type identifier for word instructions
pub const WORD_TYPE_INFO: u8 = 3;
/// Type identifier for byte instructions
pub const BYTE_TYPE_INFO: u8 = 4;
/// Type identifier for logic shift instructions
pub const LOGIC_TYPE_INFO: u8 = 5;
/// Type identifier for arithmetic shift instructions
pub const ARITH_TYPE_INFO: u8 = 6;

// Arithmetic base instructions

/// Instruction code for the `addt` instructions
pub const ADD_T_INST_CODE: u16 = 0;
/// Instruction code for the `addi` instructions
pub const ADD_I_INST_CODE: u16 = 1;
/// Instruction code for the `subt` instructions
pub const SUB_T_INST_CODE: u16 = 2;
/// Instruction code for the `subi` instructions
pub const SUB_I_INST_CODE: u16 = 3;

// Arithmetic shift instructions

/// Instruction code for the `shlt` instructions
pub const SHL_T_INST_CODE: u16 = 0;
/// Instruction code for the `shli` instructions
pub const SHL_I_INST_CODE: u16 = 1;
/// Instruction code for the `shrt` instructions
pub const SHR_T_INST_CODE: u16 = 2;
/// Instruction code for the `shri` instructions
pub const SHR_I_INST_CODE: u16 = 3;

// Arithmetic bitwise logic instructions

/// Instruction code for the `andt` instruction
pub const AND_T_INST_CODE: u16 = 0;
/// Instruction code for the `andi` instruction
pub const AND_I_INST_CODE: u16 = 1;
/// Instruction code for the `ort` instruction
pub const OR_T_INST_CODE: u16 = 2;
/// Instruction code for the `ori` instruction
pub const OR_I_INST_CODE: u16 = 3;
/// Instruction code for the `xort` instruction
pub const XOR_T_INST_CODE: u16 = 4;
/// Instruction code for the `xori` instruction
pub const XOR_I_INST_CODE: u16 = 5;
/// Instruction code for the `nott` instruction
pub const NOT_T_INST_CODE: u16 = 6;
/// Instruction code for the `noti` instruction
pub const NOT_I_INST_CODE: u16 = 7;

// Arithmetic mult div easy instructions

/// Instruction code for the `mult` instructions
pub const MUL_T_INST_CODE: u16 = 1;
/// Instruction code for the `muli` instructions
pub const MUL_I_INST_CODE: u16 = 2;
/// Instruction code for the `div` instructions
pub const DIV_T_INST_CODE: u16 = 3;

// Jump instructions

/// Instruction code for the `jmp` instruction
pub const JMP_INST_CODE: u16 = 0;
/// Instruction code for the `jmpeq` instruction
pub const JMP_EQ_INST_CODE: u16 = 1;
/// Instruction code for the `jmpne` instruction
pub const JMP_NE_INST_CODE: u16 = 2;
/// Instruction code for the `jmpgt` instructions
pub const JMP_GT_INST_CODE: u16 = 3;
/// Instruction code for the `jmpge` instructions
pub const JMP_GE_INST_CODE: u16 = 4;
/// Instruction code for the `jmplt` instructions
pub const JMP_LT_INST_CODE: u16 = 5;
/// Instruction code for the `jmple` instructions
pub const JMP_LE_INST_CODE: u16 = 6;

// Function instructions

/// Instruction code for the `cal` instruction
pub const CALL_INST_CODE: u16 = 0;
/// Instruction code for the `ret` instruction
pub const RET_INST_CODE: u16 = 1;

// Memory move instructions

/// Instruction code for the `mov` instruction
pub const MOV_INST_CODE: u16 = 0;
/// Instruction code for the `load` instructions
pub const LOAD_INST_CODE: u16 = 1;
/// Instruction code for the `loado` instructions
pub const LOAD_O_INST_CODE: u16 = 2;
/// Instruction code for the `store` instructions
pub const STORE_INST_CODE: u16 = 3;
/// Instruction code for the `storeo` instructions
pub const STORE_O_INST_CODE: u16 = 4;

// Stack instructions

/// Instruction code for the `push` instruction
pub const PUSH_INST_CODE: u16 = 0;
/// Instruction code for the `pop` instruction
pub const POP_INST_CODE: u16 = 1;

// Misc instructions

/// Instruction code for the `halt` instruction
pub const HALT_INST_CODE: u16 = 0;
/// Instruction code for the `exit` instruction
pub const EXIT_INST_CODE: u16 = 1;
/// Instruction code for the `print` instruction
pub const PRINT_INST_CODE: u16 = 2;
/// Instruction code for the `read` instruction
pub const READ_INST_CODE: u16 = 3;
/// Instruction code for the `dump` instruction
pub const DUMP_INST_CODE: u16 = 4;
/// Instruction code for the `nop` instruction
pub const NOP_INST_CODE: u16 = 5;

pub mod inst {
    use super::*;

    /// Instruction ident for the `addts` instruction
    pub const ADD_T_S_INST: [u8; 3] =
        encode_instruction(ADD_T_INST_CODE, SIGNED_TYPE_INFO, ARITHMETIC_BASE_GROUP);
    /// Instruction ident for the `addis` instruction
    pub const ADD_I_S_INST: [u8; 3] =
        encode_instruction(ADD_I_INST_CODE, SIGNED_TYPE_INFO, ARITHMETIC_BASE_GROUP);
    /// Instruction ident for the `addtu` instruction
    pub const ADD_T_U_INST: [u8; 3] =
        encode_instruction(ADD_T_INST_CODE, UNSIGNED_TYPE_INFO, ARITHMETIC_BASE_GROUP);
    /// Instruction ident for the `addiu` instruction
    pub const ADD_I_U_INST: [u8; 3] =
        encode_instruction(ADD_I_INST_CODE, UNSIGNED_TYPE_INFO, ARITHMETIC_BASE_GROUP);
    /// Instruction ident for the `subts` instruction
    pub const SUB_T_S_INST: [u8; 3] =
        encode_instruction(SUB_T_INST_CODE, SIGNED_TYPE_INFO, ARITHMETIC_BASE_GROUP);
    /// Instruction ident for the `subis` instruction
    pub const SUB_I_S_INST: [u8; 3] =
        encode_instruction(SUB_I_INST_CODE, SIGNED_TYPE_INFO, ARITHMETIC_BASE_GROUP);
    /// Instruction ident for the `subtu` instruction
    pub const SUB_T_U_INST: [u8; 3] =
        encode_instruction(SUB_T_INST_CODE, UNSIGNED_TYPE_INFO, ARITHMETIC_BASE_GROUP);
    /// Instruction ident for the `subiu` instruction
    pub const SUB_I_U_INST: [u8; 3] =
        encode_instruction(SUB_I_INST_CODE, UNSIGNED_TYPE_INFO, ARITHMETIC_BASE_GROUP);

    /// Instruction ident for the `lshlt` instruction
    pub const LSHL_T_S_INST: [u8; 3] =
        encode_instruction(SHL_T_INST_CODE, LOGIC_TYPE_INFO, ARITHMETIC_SHIFT_GROUP);
    /// Instruction ident for the `lshli` instruction
    pub const LSHL_I_S_INST: [u8; 3] =
        encode_instruction(SHL_I_INST_CODE, LOGIC_TYPE_INFO, ARITHMETIC_SHIFT_GROUP);
    /// Instruction ident for the `lshrt` instruction
    pub const LSHR_T_S_INST: [u8; 3] =
        encode_instruction(SHR_T_INST_CODE, LOGIC_TYPE_INFO, ARITHMETIC_SHIFT_GROUP);
    /// Instruction ident for the `lshri` instruction
    pub const LSHR_I_S_INST: [u8; 3] =
        encode_instruction(SHR_I_INST_CODE, LOGIC_TYPE_INFO, ARITHMETIC_SHIFT_GROUP);
    /// Instruction ident for the `ashrt` instruction
    pub const ASHR_T_S_INST: [u8; 3] =
        encode_instruction(SHR_T_INST_CODE, ARITH_TYPE_INFO, ARITHMETIC_SHIFT_GROUP);
    /// Instruction ident for the `ashri` instruction
    pub const ASHR_I_S_INST: [u8; 3] =
        encode_instruction(SHR_I_INST_CODE, ARITH_TYPE_INFO, ARITHMETIC_SHIFT_GROUP);

    /// Instruction ident for the `andt` instruction
    pub const AND_T_S_INST: [u8; 3] =
        encode_instruction(AND_T_INST_CODE, LOGIC_TYPE_INFO, ARITHMETIC_BIT_LOG_GROUP);
    /// Instruction ident for the `andi` instruction
    pub const AND_I_S_INST: [u8; 3] =
        encode_instruction(AND_I_INST_CODE, LOGIC_TYPE_INFO, ARITHMETIC_BIT_LOG_GROUP);
    /// Instruction ident for the `ort` instruction
    pub const OR_T_S_INST: [u8; 3] =
        encode_instruction(OR_T_INST_CODE, LOGIC_TYPE_INFO, ARITHMETIC_BIT_LOG_GROUP);
    /// Instruction ident for the `ori` instruction
    pub const OR_I_S_INST: [u8; 3] =
        encode_instruction(OR_I_INST_CODE, LOGIC_TYPE_INFO, ARITHMETIC_BIT_LOG_GROUP);
    /// Instruction ident for the `xort` instruction
    pub const XOR_T_S_INST: [u8; 3] =
        encode_instruction(XOR_T_INST_CODE, LOGIC_TYPE_INFO, ARITHMETIC_BIT_LOG_GROUP);
    /// Instruction ident for the `xori` instruction
    pub const XOR_I_S_INST: [u8; 3] =
        encode_instruction(XOR_I_INST_CODE, LOGIC_TYPE_INFO, ARITHMETIC_BIT_LOG_GROUP);
    /// Instruction ident for the `nott` instruction
    pub const NOT_T_S_INST: [u8; 3] =
        encode_instruction(NOT_T_INST_CODE, LOGIC_TYPE_INFO, ARITHMETIC_BIT_LOG_GROUP);
    /// Instruction ident for the `noti` instruction
    pub const NOT_I_S_INST: [u8; 3] =
        encode_instruction(NOT_I_INST_CODE, LOGIC_TYPE_INFO, ARITHMETIC_BIT_LOG_GROUP);

    /// Instruction ident for the `mults_e` instruction
    pub const MUL_T_S_E_INST: [u8; 3] = encode_instruction(
        MUL_T_INST_CODE,
        SIGNED_TYPE_INFO,
        ARITHMETIC_MULT_DIV_EASY_GROUP,
    );
    /// Instruction ident for the `mulis_e` instruction
    pub const MUL_I_S_E_INST: [u8; 3] = encode_instruction(
        MUL_I_INST_CODE,
        SIGNED_TYPE_INFO,
        ARITHMETIC_MULT_DIV_EASY_GROUP,
    );
    /// Instruction ident for the `multu_e` instruction
    pub const MUL_T_U_E_INST: [u8; 3] = encode_instruction(
        MUL_T_INST_CODE,
        UNSIGNED_TYPE_INFO,
        ARITHMETIC_MULT_DIV_EASY_GROUP,
    );
    /// Instruction ident for the `muliu_e` instruction
    pub const MUL_I_U_E_INST: [u8; 3] = encode_instruction(
        MUL_I_INST_CODE,
        UNSIGNED_TYPE_INFO,
        ARITHMETIC_MULT_DIV_EASY_GROUP,
    );
    /// Instruction ident for the `divts_e` instruction
    pub const DIV_T_S_E_INST: [u8; 3] = encode_instruction(
        DIV_T_INST_CODE,
        SIGNED_TYPE_INFO,
        ARITHMETIC_MULT_DIV_EASY_GROUP,
    );
    /// Instruction ident for the `divtu_e` instruction
    pub const DIV_T_U_E_INST: [u8; 3] = encode_instruction(
        DIV_T_INST_CODE,
        UNSIGNED_TYPE_INFO,
        ARITHMETIC_MULT_DIV_EASY_GROUP,
    );

    /// Instruction ident for the `jmp` instruction
    pub const JMP_INST: [u8; 3] = encode_instruction(JMP_INST_CODE, UNTYPED_TYPE, JUMP_GROUP);
    /// Instruction ident for the `jmpeq` instruction
    pub const JMP_EQ_INST: [u8; 3] = encode_instruction(JMP_EQ_INST_CODE, UNTYPED_TYPE, JUMP_GROUP);
    /// Instruction ident for the `jmpne` instruction
    pub const JMP_NE_INST: [u8; 3] = encode_instruction(JMP_NE_INST_CODE, UNTYPED_TYPE, JUMP_GROUP);
    /// Instruction ident for the `jmpgts` instruction
    pub const JMP_GT_S_INST: [u8; 3] =
        encode_instruction(JMP_GT_INST_CODE, SIGNED_TYPE_INFO, JUMP_GROUP);
    /// Instruction ident for the `jmpges` instruction
    pub const JMP_GE_S_INST: [u8; 3] =
        encode_instruction(JMP_GE_INST_CODE, SIGNED_TYPE_INFO, JUMP_GROUP);
    /// Instruction ident for the `jmplts` instruction
    pub const JMP_LT_S_INST: [u8; 3] =
        encode_instruction(JMP_LT_INST_CODE, SIGNED_TYPE_INFO, JUMP_GROUP);
    /// Instruction ident for the `jmples` instruction
    pub const JMP_LE_S_INST: [u8; 3] =
        encode_instruction(JMP_LE_INST_CODE, SIGNED_TYPE_INFO, JUMP_GROUP);
    /// Instruction ident for the `jmpgtu` instruction
    pub const JMP_GT_U_INST: [u8; 3] =
        encode_instruction(JMP_GT_INST_CODE, UNSIGNED_TYPE_INFO, JUMP_GROUP);
    /// Instruction ident for the `jmpgeu` instruction
    pub const JMP_GE_U_INST: [u8; 3] =
        encode_instruction(JMP_GE_INST_CODE, UNSIGNED_TYPE_INFO, JUMP_GROUP);
    /// Instruction ident for the `jmpltu` instruction
    pub const JMP_LT_U_INST: [u8; 3] =
        encode_instruction(JMP_LT_INST_CODE, UNSIGNED_TYPE_INFO, JUMP_GROUP);
    /// Instruction ident for the `jmpleu` instruction
    pub const JMP_LE_U_INST: [u8; 3] =
        encode_instruction(JMP_LE_INST_CODE, UNSIGNED_TYPE_INFO, JUMP_GROUP);

    /// Instruction ident for the `cal` instruction
    pub const CAL_INST: [u8; 3] = encode_instruction(CALL_INST_CODE, UNTYPED_TYPE, FUNCTION_GROUP);
    /// Instruction ident for the `ret` instruction
    pub const RET_INST: [u8; 3] = encode_instruction(RET_INST_CODE, UNTYPED_TYPE, FUNCTION_GROUP);

    /// Instruction ident for the `mov` instruction
    pub const MOV_INST: [u8; 3] = encode_instruction(MOV_INST_CODE, UNTYPED_TYPE, MEMORY_GROUP);
    /// Instruction ident for the `load` instruction
    pub const LOAD_INST: [u8; 3] = encode_instruction(LOAD_INST_CODE, WORD_TYPE_INFO, MEMORY_GROUP);
    /// Instruction ident for the `loado` instruction
    pub const LOAD_O_INST: [u8; 3] =
        encode_instruction(LOAD_O_INST_CODE, WORD_TYPE_INFO, MEMORY_GROUP);
    /// Instruction ident for the `loadb` instruction
    pub const LOAD_B_INST: [u8; 3] =
        encode_instruction(LOAD_O_INST_CODE, BYTE_TYPE_INFO, MEMORY_GROUP);
    /// Instruction ident for the `loadbo` instruction
    pub const LOAD_B_O_INST: [u8; 3] =
        encode_instruction(LOAD_O_INST_CODE, BYTE_TYPE_INFO, MEMORY_GROUP);
    /// Instruction ident for the `stor` instruction
    pub const STOR_INST: [u8; 3] =
        encode_instruction(STORE_INST_CODE, WORD_TYPE_INFO, MEMORY_GROUP);
    /// Instruction ident for the `storo` instruction
    pub const STOR_O_INST: [u8; 3] =
        encode_instruction(STORE_O_INST_CODE, WORD_TYPE_INFO, MEMORY_GROUP);
    /// Instruction ident for the `storb` instruction
    pub const STOR_B_INST: [u8; 3] =
        encode_instruction(STORE_O_INST_CODE, BYTE_TYPE_INFO, MEMORY_GROUP);
    /// Instruction ident for the `storbo` instruction
    pub const STOR_B_O_INST: [u8; 3] =
        encode_instruction(STORE_O_INST_CODE, BYTE_TYPE_INFO, MEMORY_GROUP);

    /// Instruction ident for the `push` instruction
    pub const PUSH_INST: [u8; 3] = encode_instruction(PUSH_INST_CODE, WORD_TYPE_INFO, MEMORY_GROUP);
    /// Instruction ident for the `pop` instruction
    pub const POP_INST: [u8; 3] = encode_instruction(POP_INST_CODE, WORD_TYPE_INFO, MEMORY_GROUP);

    /// Instruction ident for the `halt` instruction
    pub const HALT_INST: [u8; 3] = encode_instruction(HALT_INST_CODE, UNTYPED_TYPE, MISC_GROUP);
    /// Instruction ident for the `exit` instruction
    pub const EXIT_INST: [u8; 3] = encode_instruction(EXIT_INST_CODE, UNTYPED_TYPE, MISC_GROUP);
    /// Instruction ident for the `print` instruction
    pub const PRINT_INST: [u8; 3] = encode_instruction(PRINT_INST_CODE, UNTYPED_TYPE, MISC_GROUP);
    /// Instruction ident for the `read` instruction
    pub const READ_INST: [u8; 3] = encode_instruction(READ_INST_CODE, UNTYPED_TYPE, MISC_GROUP);
    /// Instruction ident for the `dump` instruction
    pub const DUMP_INST: [u8; 3] = encode_instruction(DUMP_INST_CODE, UNTYPED_TYPE, MISC_GROUP);
    /// Instruction ident for the `nop` instruction
    pub const NOP_INST: [u8; 3] = encode_instruction(NOP_INST_CODE, UNTYPED_TYPE, MISC_GROUP);
}
