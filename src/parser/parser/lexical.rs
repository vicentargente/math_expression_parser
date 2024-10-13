use super::enums::token::Token;

pub struct LexicalAnalyzer<'a> {
    operation_string: &'a String
}

impl LexicalAnalyzer<'_> {
    pub fn new(operation_string: &String) -> LexicalAnalyzer {
        LexicalAnalyzer {
            operation_string
        }
    }

    pub fn tokenize(&self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        let radix = 10; //TODO: Chose the radix dynamically

        let bytes = self.operation_string.as_bytes();
        let mut current_index = 0;
        while current_index < bytes.len() {
            let current_char = bytes[current_index] as char;

            if current_char.is_whitespace() {
                current_index += 1;
                continue;
            }

            if current_char == 'l' {
                // Temporal
                if current_index + 3 >= bytes.len() {
                    current_index += 1;
                    continue;
                }

                let op = &self.operation_string[current_index..current_index + 3];
                if op == "lsl" || op == "lsr" {
                    tokens.push(Token::Operator(op));
                    current_index += 3;
                    continue;
                }
            }
            else if current_char == 'a' {
                // Temporal
                if current_index + 3 >= bytes.len() {
                    current_index += 1;
                    continue;
                }

                let op = &self.operation_string[current_index..current_index + 3];
                if op == "asl" || op == "asr" || op == "and" {
                    tokens.push(Token::Operator(op));
                    current_index += 3;
                    continue;
                }
            }
            else if current_char == 'o' {
                // Temporal
                if current_index + 2 >= bytes.len() {
                    current_index += 1;
                    continue;
                }

                let op = &self.operation_string[current_index..current_index + 2];
                if op == "or" {
                    tokens.push(Token::Operator(op));
                    current_index += 2;
                    continue;
                }
            }
            else if current_char == 'x' {
                // Temporal
                if current_index + 3 >= bytes.len() {
                    current_index += 1;
                    continue;
                }

                let op = &self.operation_string[current_index..current_index + 3];
                if op == "xor" {
                    tokens.push(Token::Operator(op));
                    current_index += 3;
                    continue;
                }
            }
            else if current_char == 'n' {
                // Temporal
                if current_index + 3 >= bytes.len() {
                    current_index += 1;
                    continue;
                }

                let op = &self.operation_string[current_index..current_index + 3];
                if op == "not" {
                    tokens.push(Token::Operator(op));
                    current_index += 3;
                    continue;
                }
            }

            if current_char.is_digit(radix) {
                let begin_index = current_index;

                current_index += 1;
                while current_index < bytes.len() {
                    let digit_char = bytes[current_index] as char;
                    if !digit_char.is_digit(radix) {
                        break;
                    }

                    current_index += 1;
                }

                tokens.push(Token::Number(&self.operation_string[begin_index..current_index]));
            }
            else if current_char == '+' || current_char == '-' || current_char == '*' || current_char == '/' || current_char == '%' {
                tokens.push(Token::Operator(&self.operation_string[current_index..current_index + 1]));
                current_index += 1;
            }
            else if current_char == '(' {
                tokens.push(Token::ParenthesisOpen);
                current_index += 1;
            }
            else if current_char == ')' {
                tokens.push(Token::ParenthesisClose);
                current_index += 1;
            }
            else {
                return Err(format!(r#"Unexpected character "{}" at position {}"#, current_char, current_index));
            }
        }

        //println!("{:?}", tokens);

        return Ok(tokens);
    }
}