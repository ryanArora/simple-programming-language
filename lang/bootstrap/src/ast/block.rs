use super::statement::Statement;

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}
