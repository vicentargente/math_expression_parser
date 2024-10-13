use super::{enums::element::Element, structs::operator::Operator, NumType};
use super::enums::element::Operator as Op;

pub struct SemanticAnalyzer {
    elements: Vec<Vec<Element<NumType>>>
}

impl SemanticAnalyzer {
    pub fn new(elements: Vec<Vec<Element<NumType>>>) -> SemanticAnalyzer {
        SemanticAnalyzer {
            elements
        }
    }

    pub fn calculate(&mut self) -> NumType {
        let mut sub_expr_values = vec![None; self.elements.len()];

        let mut expression_iter = self.elements.iter_mut().enumerate().rev();
        while let Some((expression_index, mut expression)) = expression_iter.next() {
            let mut operator_iter = expression.iter().enumerate();

            let mut first_priority_operators: Vec<Operator> = Vec::new();
            let mut second_priority_operators: Vec<Operator> = Vec::new();
            let mut third_priority_operators: Vec<Operator> = Vec::new();
            let mut fourth_priority_operators: Vec<Operator> = Vec::new();
            let mut fifth_priority_operators: Vec<Operator> = Vec::new();
            let mut sixth_priority_operators: Vec<Operator> = Vec::new();
            let mut seventh_priority_operators: Vec<Operator> = Vec::new();

            while let Some((element_index, element)) = operator_iter.next() {
                match element {
                    Element::Operator(operator) => {
                        match operator {
                            Op::Not => {
                                first_priority_operators.push(Operator::new(element_index, *operator));
                            },
                            Op::Lsl | Op::Lsr | Op::Asl | Op::Asr => {
                                second_priority_operators.push(Operator::new(element_index, *operator));
                            },
                            Op::Mul | Op::Div | Op::Mod => {
                                third_priority_operators.push(Operator::new(element_index, *operator));
                            },
                            Op::Add | Op::Sub => {
                                fourth_priority_operators.push(Operator::new(element_index, *operator));
                            },
                            Op::And => {
                                fifth_priority_operators.push(Operator::new(element_index, *operator));
                            },
                            Op::Xor => {
                                sixth_priority_operators.push(Operator::new(element_index, *operator));
                            },
                            Op::Or => {
                                seventh_priority_operators.push(Operator::new(element_index, *operator));
                            },
                        }
                    },
                    _ => {}
                }
            }

            let ordered_priority_operators = [
                &mut first_priority_operators,
                &mut second_priority_operators,
                &mut third_priority_operators,
                &mut fourth_priority_operators,
                &mut fifth_priority_operators,
                &mut sixth_priority_operators,
                &mut seventh_priority_operators
            ];

            for i in 0..ordered_priority_operators.len() {
                for j in 0..ordered_priority_operators[i].len() {
                    let deleted_elements = {
                        let operator = &ordered_priority_operators[i][j];
                        operator.execute(&mut expression, &sub_expr_values)
                    };
            
                    if let Some(deleted_elements) = deleted_elements {
                        let current_operator_index = ordered_priority_operators[i][j].get_index();
            
                        for k in 0..ordered_priority_operators.len() {
                            for l in 0..ordered_priority_operators[k].len() {
                                if ordered_priority_operators[k][l].get_index() > current_operator_index {
                                    let new_index = ordered_priority_operators[k][l].get_index() - deleted_elements;
                                    ordered_priority_operators[k][l].set_index(
                                        new_index
                                    );
                                }
                            }
                        }
                    }
                }
            }

            if let Element::Number(val) = expression[0] {
                sub_expr_values[expression_index] = Some(val);
            }
            
        };

        return sub_expr_values[0].unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_analyzer() {
        let elements = vec![
            vec![
                Element::Number(1),
                Element::Operator(Op::Add),
                Element::Number(2),
                Element::Operator(Op::Mul),
                Element::Number(3),
                Element::Operator(Op::Div),
                Element::Number(2),
                Element::Operator(Op::Sub),
                Element::SubExpression(1)
            ],
            vec![
                Element::Number(1),
                Element::Operator(Op::Add),
                Element::Number(2)
            ]
        ];

        let mut semantic_analyzer = SemanticAnalyzer::new(elements);
        let result = semantic_analyzer.calculate();

        assert_eq!(result, 1);
    }
}