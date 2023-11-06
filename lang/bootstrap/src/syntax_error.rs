#[derive(Debug)]
pub enum SyntaxError {
    InvalidToken,
    InvalidEscapeSequenceInStringLiteral,
    InvalidEscapeSequenceInCharLiteral,
    EmptyCharLiteral,
    UnterminatedCharLiteral,
    UnterminatedStringLiteral,
    TooLargeIntegerLiteral,
    NoExpressionAfterUnaryOperator,
    NoExpressionAfterBinaryOperator,
    NoExpressionAfterLParen,
    UnmatchedParen,
    StatementWithoutSemicolon,
}
