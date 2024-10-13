// Tokenize
#[derive(Debug)]
pub enum Token<'a> {
    Number(&'a str),
    Operator(&'a str),
    ParenthesisOpen,
    ParenthesisClose
}