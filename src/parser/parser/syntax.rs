use std::collections::VecDeque;

use super::enums::token::Token;
use super::num_type::NumType;

use super::enums::element::{Element, Operator};

pub struct SyntaxAnalyzer<'a> {
    tokens: Vec<Token<'a>>
}

impl<'a> SyntaxAnalyzer<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> SyntaxAnalyzer<'a> {
        SyntaxAnalyzer {
            tokens
        }
    }

    pub fn analyze(&self) -> Result<Vec<Vec<Element<NumType>>>, String>
    {
        let mut expressions_tokens = VecDeque::with_capacity(1);
        expressions_tokens.push_back(&self.tokens[..]);

        let mut interpreted_expressions: Vec<Vec<Element<NumType>>> = Vec::new();
        let mut subexpression_index = 1usize;

        while let Some(expr_tokens) = expressions_tokens.pop_front() {
            let mut token_iter = expr_tokens.iter().enumerate();
            let mut current_token = token_iter.next();

            let mut state = ParserState::Initial(Sign::Positive);
            let mut current_expression = Vec::new();

            while let Some((token_index, token)) = current_token {
                match state {
                    ParserState::Initial(sign) => {
                        match token {
                            Token::Number(val) => {
                                let num = match val.parse::<NumType>() {
                                    Ok(num) => num,
                                    Err(_) => {
                                        return Err(format!("Invalid number: {}", val));
                                    }
                                };

                                state = ParserState::Operand;
                                current_token = token_iter.next();

                                current_expression.push(Element::Number(0 as NumType));
                                current_expression.push(Element::Operator(match sign {
                                    Sign::Positive => Operator::Add,
                                    Sign::Negative => Operator::Sub
                                }));
                                current_expression.push(Element::Number(num));
                            },
                            Token::Operator("+") => {
                                current_token = token_iter.next();
                            },
                            Token::Operator("-") => {
                                current_token = token_iter.next();
                                state = ParserState::Initial(Sign::Negative);
                            },
                            Token::ParenthesisOpen => {
                                state = ParserState::SubExpressionOpen;
                                current_token = token_iter.next();

                                current_expression.push(Element::Number(0 as NumType));
                                current_expression.push(Element::Operator(match sign {
                                    Sign::Positive => Operator::Add,
                                    Sign::Negative => Operator::Sub
                                }));
                                // Push zero
                                // Push operator (from sign)
                            },
                            Token::Operator(val) => {
                                match *val {
                                    "not" => {
                                        state = ParserState::UnaryOperator;
                                        current_token = token_iter.next();

                                        current_expression.push(Element::Number(0 as NumType));
                                        current_expression.push(Element::Operator(match sign {
                                            Sign::Positive => Operator::Add,
                                            Sign::Negative => Operator::Sub
                                        }));
                                        current_expression.push(Element::Operator(Operator::Not));
                                    },
                                    _=> {
                                        return Err(format!("Unexpected operator at position {}", token_index));
                                    }
                                }
                                
                            },
                            _ => {
                                return Err(format!("Unexpected token at position at position {}", token_index));
                            }
                        }
                    },
                    ParserState::Operand => {
                        match token {
                            Token::Number(_) => {
                                return Err(format!("Cannot have two numbers in a row"));
                            },
                            Token::Operator(val) => {
                                let operator = match Operator::from_str(val) {
                                    Some(op) => op,
                                    None => {
                                        return Err(format!("Unexpected operator: {}", val));
                                    }
                                };

                                current_expression.push(Element::Operator(operator));

                                current_token = token_iter.next();

                                state = ParserState::BinaryOperator;
                            },
                            Token::ParenthesisOpen => {
                                current_expression.push(Element::Operator(Operator::Mul));

                                current_token = token_iter.next();

                                state = ParserState::SubExpressionOpen;
                            },
                            Token::ParenthesisClose => {
                                return Err(format!("Unexpected parenthesis close at position {}", token_index));
                            },
                        }
                    },
                    ParserState::UnaryOperator => {
                        match token {
                            Token::Number(val) => {
                                let num = match val.parse::<NumType>() {
                                    Ok(num) => num,
                                    Err(_) => {
                                        return Err(format!("Invalid number: {}", val));
                                    }
                                };

                                current_expression.push(Element::Number(num));

                                current_token = token_iter.next();

                                state = ParserState::Operand;
                            },
                            Token::Operator(_) => {
                                return Err(format!("Cannot have two operators in a row at position {}", token_index));
                            },
                            Token::ParenthesisOpen => {
                                state = ParserState::SubExpressionOpen;
                                current_token = token_iter.next();
                            },
                            Token::ParenthesisClose => {
                                return Err(format!("Unexpected parenthesis close at position {}", token_index));
                            },
                        }
                    },
                    ParserState::BinaryOperator => {
                        match token {
                            Token::Number(val) => {
                                let num = match val.parse::<NumType>() {
                                    Ok(num) => num,
                                    Err(_) => {
                                        return Err(format!("Invalid number: {}", val));
                                    }
                                };

                                current_expression.push(Element::Number(num));

                                current_token = token_iter.next();

                                state = ParserState::Operand;
                            },
                            Token::Operator(_) => {
                                return Err(format!("Cannot have two operators in a row at position {}", token_index));
                            },
                            Token::ParenthesisOpen => {
                                state = ParserState::SubExpressionOpen;
                                current_token = token_iter.next();
                            },
                            Token::ParenthesisClose => {
                                return Err(format!("Unexpected parenthesis close at position {}", token_index));
                            },
                        };
                    },
                    ParserState::SubExpressionOpen => {
                        let subexpr_beginning = token_index;
                        let mut parenthesis_count = 1usize;
                        while let Some((token_index, token)) = current_token {
                            match token {
                                Token::ParenthesisOpen => {
                                    parenthesis_count += 1;
                                },
                                Token::ParenthesisClose => {
                                    if parenthesis_count == 1 {
                                        expressions_tokens.push_back(&expr_tokens[subexpr_beginning..token_index]);
                                        current_expression.push(Element::SubExpression(subexpression_index));
                                        subexpression_index += 1;
                                        state = ParserState::Operand;

                                        current_token = token_iter.next();
                                        break;
                                    } else {
                                        parenthesis_count -= 1;
                                    }
                                },
                                _ => {}
                            }

                            current_token = token_iter.next();
                        }

                        if parenthesis_count != 1 {
                            return Err(format!("Unmatched parenthesis")); // TODO, add position
                        }
                    }
                }
            }

            if state != ParserState::Operand {
                return Err(format!("Unexpected end of expression"));
            }

            interpreted_expressions.push(current_expression);
        }
        
        return Ok(interpreted_expressions);
    }
}

// Analyze tokens
#[derive(Clone, Copy, PartialEq)]
enum Sign {
    Positive,
    Negative
}

#[derive(PartialEq)]
enum ParserState {
    Initial(Sign),
    Operand,
    UnaryOperator,
    BinaryOperator,
    SubExpressionOpen
}

//tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_tokens_single_number() {
        let tokens = vec![Token::Number("42")];
        
        let analyzer = SyntaxAnalyzer::new(tokens);
        let result = analyzer.analyze();

        assert!(result.is_ok());

        let expressions = result.unwrap();

        assert_eq!(expressions.len(), 1);
        assert_eq!(expressions[0], vec![
            Element::Number(0),
            Element::Operator(Operator::Add),
            Element::Number(42)
        ]);
    }

    #[test]
    fn test_analyze_tokens_simple_addition() {
        let tokens = vec![
            Token::Number("1"),
            Token::Operator("+"),
            Token::Number("2")
        ];

        let analyzer = SyntaxAnalyzer::new(tokens);
        
        let result = analyzer.analyze();
        assert!(result.is_ok());
        let expressions = result.unwrap();
        //println!("{:?}", expressions);
        assert_eq!(expressions.len(), 1);
        assert_eq!(expressions[0], vec![
            Element::Number(0),
            Element::Operator(Operator::Add),
            Element::Number(1),
            Element::Operator(Operator::Add),
            Element::Number(2)
        ]);
    }

    #[test]
    fn test_analyze_tokens_simple_subtraction() {
        let tokens = vec![
            Token::Number("5"),
            Token::Operator("-"),
            Token::Number("3")
        ];
        
        let analyzer = SyntaxAnalyzer::new(tokens);
        
        let result = analyzer.analyze();
        assert!(result.is_ok());
        let expressions = result.unwrap();
        //println!("{:?}", expressions);
        assert_eq!(expressions.len(), 1);
        assert_eq!(expressions[0], vec![
            Element::Number(0),
            Element::Operator(Operator::Add),
            Element::Number(5),
            Element::Operator(Operator::Sub),
            Element::Number(3)
        ]);
    }

    #[test]
    fn test_analyze_tokens_unexpected_token() {
        let tokens = vec![
            Token::Number("5"),
            Token::Operator("*")
        ];

        let analyzer = SyntaxAnalyzer::new(tokens);
        
        let result = analyzer.analyze();
        assert!(result.is_err());
    }

    #[test]
    fn test_analyze_tokens_unmatched_parenthesis() {
        let tokens = vec![
            Token::ParenthesisOpen,
            Token::Number("5"),
            Token::Operator("+"),
            Token::Number("3")
        ];

        let analyzer = SyntaxAnalyzer::new(tokens);
        
        let result = analyzer.analyze();
        assert!(result.is_err());
    }

    #[test]
    fn test_analyze_tokens_multiple_unmatched_parenthesis() {
        let tokens = vec![
            Token::ParenthesisOpen,
            Token::ParenthesisOpen,
            Token::Number("5"),
            Token::Operator("+"),
            Token::Number("3"),
            Token::ParenthesisClose
        ];

        let analyzer = SyntaxAnalyzer::new(tokens);

        let result = analyzer.analyze();
        assert!(result.is_err());
    }

    #[test]
    fn test_analyze_tokens_nested_parenthesis() {
        let tokens = vec![
            Token::ParenthesisOpen,
            Token::Number("5"),
            Token::Operator("+"),
            Token::ParenthesisOpen,
            Token::Number("3"),
            Token::Operator("*"),
            Token::Number("2"),
            Token::ParenthesisClose,
            Token::ParenthesisClose
        ];

        let analyzer = SyntaxAnalyzer::new(tokens);
        
        let result = analyzer.analyze();
        assert!(result.is_ok());
        let expressions = result.unwrap();
        //println!("{:?}", expressions);
        assert_eq!(expressions.len(), 3);
        assert_eq!(expressions[0], vec![
            Element::Number(0),
            Element::Operator(Operator::Add),
            Element::SubExpression(1)
        ]);

        assert_eq!(expressions[1], vec![
            Element::Number(0),
            Element::Operator(Operator::Add),
            Element::Number(5),
            Element::Operator(Operator::Add),
            Element::SubExpression(2)
        ]);

        assert_eq!(expressions[2], vec![
            Element::Number(0),
            Element::Operator(Operator::Add),
            Element::Number(3),
            Element::Operator(Operator::Mul),
            Element::Number(2)
        ]);
    }

    #[test]
    fn test_analyze_tokens_multiple_samelevel_parenthesis() {
        let tokens = vec![
            Token::Number("2"),
            Token::Operator("-"),
            Token::ParenthesisOpen,
            Token::Number("5"),
            Token::Operator("+"),
            Token::ParenthesisOpen,
            Token::Number("3"),
            Token::Operator("*"),
            Token::Number("2"),
            Token::ParenthesisClose,
            Token::ParenthesisClose,
            Token::Operator("/"),
            Token::ParenthesisOpen,
            Token::Number("4"),
            Token::Operator("+"),
            Token::Number("2"),
            Token::ParenthesisClose
        ];

        let analyzer = SyntaxAnalyzer::new(tokens);
        
        let result = analyzer.analyze();
        assert!(result.is_ok());
        let expressions = result.unwrap();
        //println!("{:?}", expressions);
        assert_eq!(expressions.len(), 4);
        assert_eq!(expressions[0], vec![
            Element::Number(0),
            Element::Operator(Operator::Add),
            Element::Number(2),
            Element::Operator(Operator::Sub),
            Element::SubExpression(1),
            Element::Operator(Operator::Div),
            Element::SubExpression(2)
        ]);

        assert_eq!(expressions[1], vec![
            Element::Number(0),
            Element::Operator(Operator::Add),
            Element::Number(5),
            Element::Operator(Operator::Add),
            Element::SubExpression(3)
        ]);

        assert_eq!(expressions[2], vec![
            Element::Number(0),
            Element::Operator(Operator::Add),
            Element::Number(4),
            Element::Operator(Operator::Add),
            Element::Number(2)
        ]);

        assert_eq!(expressions[3], vec![
            Element::Number(0),
            Element::Operator(Operator::Add),
            Element::Number(3),
            Element::Operator(Operator::Mul),
            Element::Number(2)
        ]);
    }
}
