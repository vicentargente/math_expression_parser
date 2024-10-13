#[derive(Debug, PartialEq)]
pub enum Element<NumType> {
    Number(NumType),
    SubExpression(usize),
    Operator(Operator)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Not,
    Lsl,
    Lsr,
    Asl,
    Asr,
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    And,
    Xor,
    Or
}

impl Operator {
    pub fn from_str(op: &str) -> Option<Operator> {
        match op {
            "not" => Some(Operator::Not),
            "lsl" => Some(Operator::Lsl),
            "lsr" => Some(Operator::Lsr),
            "asl" => Some(Operator::Asl),
            "asr" => Some(Operator::Asr),
            "*" => Some(Operator::Mul),
            "/" => Some(Operator::Div),
            "%" => Some(Operator::Mod),
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Sub),
            "and" => Some(Operator::And),
            "xor" => Some(Operator::Xor),
            "or" => Some(Operator::Or),
            _ => None
        }
    }
}