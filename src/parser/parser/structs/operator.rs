use crate::parser::parser::{enums::element::Element, num_type::{NumType, _UnsignedNumType}};
use crate::parser::parser::enums::element::Operator as OperatorType;

pub struct Operator {
    index: usize,
    op_type: OperatorType
}

impl Operator {
    pub fn new(index: usize, op_type: OperatorType) -> Operator {
        Operator { index, op_type }
    }

    pub fn execute(&self, elements: &mut Vec<Element<NumType>>, subexpression_values: &Vec<Option<NumType>>) -> Option<usize> {
        match self.op_type {
            OperatorType::Not => {
                let right_operand = self.get_right_operand(elements, subexpression_values)?;

                elements[self.index + 1] = Element::Number(!right_operand);
                elements.remove(self.index);
                Some(1)
            },
            OperatorType::Lsl => {
                let left_operand = self.get_left_operand(elements, subexpression_values)?;
                let right_operand = self.get_right_operand(elements, subexpression_values)?;

                elements[self.index - 1] = Element::Number(((left_operand as _UnsignedNumType) << (right_operand as _UnsignedNumType)) as NumType);
                elements.remove(self.index);
                elements.remove(self.index);
                Some(2)
            },
            OperatorType::Lsr => {
                let left_operand = self.get_left_operand(elements, subexpression_values)?;
                let right_operand = self.get_right_operand(elements, subexpression_values)?;
                
                elements[self.index - 1] = Element::Number(((left_operand as _UnsignedNumType) >> (right_operand as _UnsignedNumType)) as NumType);
                elements.remove(self.index);
                elements.remove(self.index);
                Some(2)
            },
            OperatorType::Asl => {
                let left_operand = self.get_left_operand(elements, subexpression_values)?;
                let right_operand = self.get_right_operand(elements, subexpression_values)?;

                elements[self.index - 1] = Element::Number(left_operand << right_operand);
                elements.remove(self.index);
                elements.remove(self.index);
                Some(2)
            },
            OperatorType::Asr => {
                let left_operand = self.get_left_operand(elements, subexpression_values)?;
                let right_operand = self.get_right_operand(elements, subexpression_values)?;

                elements[self.index - 1] = Element::Number(left_operand >> right_operand);
                elements.remove(self.index);
                elements.remove(self.index);
                Some(2)
            },
            OperatorType::Mul => {
                let left_operand = self.get_left_operand(elements, subexpression_values)?;
                let right_operand = self.get_right_operand(elements, subexpression_values)?;

                elements[self.index - 1] = Element::Number(left_operand * right_operand);
                elements.remove(self.index);
                elements.remove(self.index);
                Some(2)
            },
            OperatorType::Div => {
                let left_operand = self.get_left_operand(elements, subexpression_values)?;
                let right_operand = self.get_right_operand(elements, subexpression_values)?;

                elements[self.index - 1] = Element::Number(left_operand / right_operand);
                elements.remove(self.index);
                elements.remove(self.index);
                Some(2)
            },
            OperatorType::Mod => {
                let left_operand = self.get_left_operand(elements, subexpression_values)?;
                let right_operand = self.get_right_operand(elements, subexpression_values)?;

                elements[self.index - 1] = Element::Number(left_operand % right_operand);
                elements.remove(self.index);
                elements.remove(self.index);
                Some(2)
            },
            OperatorType::Add => {
                let left_operand = self.get_left_operand(elements, subexpression_values)?;
                let right_operand = self.get_right_operand(elements, subexpression_values)?;

                elements[self.index - 1] = Element::Number(left_operand + right_operand);
                elements.remove(self.index);
                elements.remove(self.index);
                Some(2)
            },
            OperatorType::Sub => {
                let left_operand = self.get_left_operand(elements, subexpression_values)?;
                let right_operand = self.get_right_operand(elements, subexpression_values)?;

                elements[self.index - 1] = Element::Number(left_operand - right_operand);
                elements.remove(self.index);
                elements.remove(self.index);
                Some(2)
            },
            OperatorType::And => {
                let left_operand = self.get_left_operand(elements, subexpression_values)?;
                let right_operand = self.get_right_operand(elements, subexpression_values)?;

                elements[self.index - 1] = Element::Number(left_operand & right_operand);
                elements.remove(self.index);
                elements.remove(self.index);
                Some(2)
            },
            OperatorType::Xor => {
                let left_operand = self.get_left_operand(elements, subexpression_values)?;
                let right_operand = self.get_right_operand(elements, subexpression_values)?;

                elements[self.index - 1] = Element::Number(left_operand ^ right_operand);
                elements.remove(self.index);
                elements.remove(self.index);
                Some(2)
            },
            OperatorType::Or => {
                let left_operand = self.get_left_operand(elements, subexpression_values)?;
                let right_operand = self.get_right_operand(elements, subexpression_values)?;

                elements[self.index - 1] = Element::Number(left_operand | right_operand);
                elements.remove(self.index);
                elements.remove(self.index);
                Some(2)
            },
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn get_left_operand(&self, elements: &mut Vec<Element<NumType>>, subexpression_values: &Vec<Option<NumType>>) -> Option<NumType> {
        match &elements[self.index - 1] {
            Element::Number(num) => Some(*num),
            Element::SubExpression(index) => match subexpression_values[*index] {
                Some(num) => Some(num),
                None => None,
            },
            _ => None,
        }
    }

    fn get_right_operand(&self, elements: &mut Vec<Element<NumType>>, subexpression_values: &Vec<Option<NumType>>) -> Option<NumType> {
        match &elements[self.index + 1] {
            Element::Number(num) => Some(*num),
            Element::SubExpression(index) => match subexpression_values[*index] {
                Some(num) => Some(num),
                None => None,
            },
            _ => None,
        }
    }
}
