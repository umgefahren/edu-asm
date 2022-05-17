use edu_asm_parser::instruction::RegisterOrLiteral;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RegisterLiteral {
    Register,
    Literal,
}

pub fn register_or_literal_slice(inp: &[RegisterOrLiteral]) -> Vec<RegisterLiteral> {
    inp.iter()
        .map(RegisterLiteral::from_token)
        .collect::<Vec<_>>()
}

impl RegisterLiteral {
    #[inline(always)]
    pub fn encode(&self) -> u8 {
        match self {
            RegisterLiteral::Literal => 1,
            RegisterLiteral::Register => 0,
        }
    }

    #[inline(always)]
    pub fn decode(inp: u8) -> Self {
        match inp {
            0 => Self::Register,
            _ => Self::Literal,
        }
    }

    pub fn from_token(token: &RegisterOrLiteral) -> Self {
        match token {
            RegisterOrLiteral::Register(_) => Self::Register,
            RegisterOrLiteral::Literal(_) => Self::Literal,
        }
    }

    pub fn is_register(&self) -> bool {
        matches!(self, RegisterLiteral::Register)
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, RegisterLiteral::Literal)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OperationMode {
    zero: RegisterLiteral,
    one: RegisterLiteral,
    two: RegisterLiteral,
    three: RegisterLiteral,
    four: RegisterLiteral,
    five: RegisterLiteral,
    six: RegisterLiteral,
    seven: RegisterLiteral,
}

impl OperationMode {
    #[inline(always)]
    pub fn encode(&self) -> u8 {
        let mut ret = 0u8;

        let zero_num = self.zero.encode();
        let one_num = self.one.encode();
        let two_num = self.two.encode();
        let three_num = self.three.encode();
        let four_num = self.four.encode();
        let five_num = self.five.encode();
        let six_num = self.six.encode();
        let seven_num = self.seven.encode();

        ret |= zero_num << 7;
        ret |= one_num << 6;
        ret |= two_num << 5;
        ret |= three_num << 4;
        ret |= four_num << 3;
        ret |= five_num << 2;
        ret |= six_num << 1;
        ret |= seven_num;

        ret
    }

    #[inline(always)]
    pub fn decode(inp: u8) -> Self {
        const ZERO_BIT: u8 = 1u8 << 7;
        const ONE_BIT: u8 = 1u8 << 6;
        const TWO_BIT: u8 = 1u8 << 5;
        const THREE_BIT: u8 = 1u8 << 4;
        const FOUR_BIT: u8 = 1u8 << 3;
        const FIVE_BIT: u8 = 1u8 << 2;
        const SIX_BIT: u8 = 1u8 << 1;
        const SEVEN_BIT: u8 = 1u8;

        let zero_num = inp & ZERO_BIT;
        let one_num = inp & ONE_BIT;
        let two_num = inp & TWO_BIT;
        let three_num = inp & THREE_BIT;
        let four_num = inp & FOUR_BIT;
        let five_num = inp & FIVE_BIT;
        let six_num = inp & SIX_BIT;
        let seven_num = inp & SEVEN_BIT;

        let zero = RegisterLiteral::decode(zero_num);
        let one = RegisterLiteral::decode(one_num);
        let two = RegisterLiteral::decode(two_num);
        let three = RegisterLiteral::decode(three_num);
        let four = RegisterLiteral::decode(four_num);
        let five = RegisterLiteral::decode(five_num);
        let six = RegisterLiteral::decode(six_num);
        let seven = RegisterLiteral::decode(seven_num);

        Self {
            zero,
            one,
            two,
            three,
            four,
            five,
            six,
            seven,
        }
    }

    pub fn new(inp: &[RegisterLiteral]) -> Self {
        assert!(inp.len() <= 8);

        let mut ret = Self {
            zero: RegisterLiteral::Register,
            one: RegisterLiteral::Register,
            two: RegisterLiteral::Register,
            three: RegisterLiteral::Register,
            four: RegisterLiteral::Register,
            five: RegisterLiteral::Register,
            six: RegisterLiteral::Register,
            seven: RegisterLiteral::Register,
        };

        for (i, val) in inp.iter().enumerate() {
            match i {
                0 => ret.zero = *val,
                1 => ret.one = *val,
                2 => ret.two = *val,
                3 => ret.three = *val,
                4 => ret.four = *val,
                5 => ret.five = *val,
                6 => ret.six = *val,
                7 => ret.seven = *val,
                _ => panic!("Invalid index"),
            }
        }

        ret
    }

    pub fn get(&self, idx: usize) -> RegisterLiteral {
        match idx {
            0 => self.zero,
            1 => self.one,
            2 => self.two,
            3 => self.three,
            4 => self.four,
            5 => self.five,
            6 => self.six,
            7 => self.seven,
            _ => panic!("Invalid index"),
        }
    }
}

#[cfg(test)]
mod operation_mode {
    #[test]
    fn encode_decode_literal() {
        use super::*;

        let mode = OperationMode {
            zero: RegisterLiteral::Literal,
            one: RegisterLiteral::Literal,
            two: RegisterLiteral::Literal,
            three: RegisterLiteral::Literal,
            four: RegisterLiteral::Literal,
            five: RegisterLiteral::Literal,
            six: RegisterLiteral::Literal,
            seven: RegisterLiteral::Literal,
        };

        let encoded = mode.encode();
        let decoded = OperationMode::decode(encoded);

        assert_eq!(mode, decoded);
    }

    #[test]
    fn encode_decode_register() {
        use super::*;

        let mode = OperationMode {
            zero: RegisterLiteral::Register,
            one: RegisterLiteral::Register,
            two: RegisterLiteral::Register,
            three: RegisterLiteral::Register,
            four: RegisterLiteral::Register,
            five: RegisterLiteral::Register,
            six: RegisterLiteral::Register,
            seven: RegisterLiteral::Register,
        };

        let encoded = mode.encode();
        let decoded = OperationMode::decode(encoded);

        assert_eq!(mode, decoded);
    }

    #[test]
    fn encode_decode_mixed() {
        use super::*;

        let mode = OperationMode {
            zero: RegisterLiteral::Literal,
            one: RegisterLiteral::Register,
            two: RegisterLiteral::Literal,
            three: RegisterLiteral::Register,
            four: RegisterLiteral::Literal,
            five: RegisterLiteral::Register,
            six: RegisterLiteral::Literal,
            seven: RegisterLiteral::Register,
        };

        let encoded = mode.encode();
        let decoded = OperationMode::decode(encoded);

        assert_eq!(mode, decoded);
    }
}
