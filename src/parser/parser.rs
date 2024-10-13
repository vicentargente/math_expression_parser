use std::collections::VecDeque;

mod enums;
mod structs;
mod lexical;
mod syntax;
mod semantic;
mod num_type;

use enums::token::Token;
use lexical::LexicalAnalyzer;



type NumType = i32; // Auxiliar
pub struct Parser {
    operation_string: String
}

impl Parser {
    pub fn new(mut operation_string: String) -> Parser {
        operation_string.make_ascii_lowercase();

        Parser {
            operation_string
        }
    }

    pub fn parse(&mut self) -> Result<NumType, String> {
        let lexical_analyzer = LexicalAnalyzer::new(&self.operation_string);
        let tokens = lexical_analyzer.tokenize()?;

        let syntax_analyzer = syntax::SyntaxAnalyzer::new(tokens);
        let operators = syntax_analyzer.analyze()?;

        let mut semantic_analyzer = semantic::SemanticAnalyzer::new(operators);
        let result = semantic_analyzer.calculate();

        return Ok(result);
    }
}

enum Operator {
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