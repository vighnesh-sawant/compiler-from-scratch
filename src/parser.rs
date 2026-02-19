use std::iter::Peekable;
use std::vec::IntoIter;

use crate::ast::{BinOp, Expression, FunctionDeclaration, Program, Statement, UnOp};
use crate::lexer::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: String },
    UnexpectedEOF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    LogicalOr,
    LogicalAnd,
    Equality,
    Compartors,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    BitwiseShift,
    Term,   // + -
    Factor, // * / %
}

impl Token {
    fn get_precedence(&self) -> Precedence {
        match self {
            Token::LogicalOr => Precedence::LogicalOr,
            Token::LogicalAnd => Precedence::LogicalAnd,
            Token::Equal => Precedence::Equality,
            Token::NotEqual => Precedence::Equality,
            Token::LessThan => Precedence::Compartors,
            Token::LessThanEqual => Precedence::Compartors,
            Token::GreaterThan => Precedence::Compartors,
            Token::GreaterThanEqual => Precedence::Compartors,
            Token::BitwiseOr => Precedence::BitwiseOr,
            Token::BitwiseXor => Precedence::BitwiseXor,
            Token::BitwiseAnd => Precedence::BitwiseAnd,
            Token::LeftShift | Token::RightShift => Precedence::BitwiseShift,
            Token::Addition | Token::Negation => Precedence::Term,
            Token::Multiplication | Token::Division | Token::Remainder => Precedence::Factor,
            _ => Precedence::None,
        }
    }
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn expect(&mut self, expected_token: Token) -> Result<(), ParseError> {
        match self.tokens.next() {
            Some(token) if token == expected_token => Ok(()),
            Some(token) => Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected_token),
                found: format!("{:?}", token),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let function = self.parse_function()?;

        if self.tokens.peek().is_some() {
            return Err(ParseError::UnexpectedToken {
                expected: "End of File".to_string(),
                found: format!("{:?}", self.tokens.peek().unwrap()),
            });
        }

        Ok(Program { function })
    }

    fn parse_function(&mut self) -> Result<FunctionDeclaration, ParseError> {
        self.expect(Token::IntKeyword)?;

        let name = match self.tokens.next() {
            Some(Token::Identifier(name)) => name,
            Some(t) => {
                return Err(ParseError::UnexpectedToken {
                    expected: "Identifier".to_string(),
                    found: format!("{:?}", t),
                });
            }
            None => return Err(ParseError::UnexpectedEOF),
        };

        self.expect(Token::OpenParen)?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;

        let body = self.parse_statement()?;

        self.expect(Token::CloseBrace)?;

        Ok(FunctionDeclaration { name, body })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect(Token::ReturnKeyword)?;

        let expr = self.parse_expression(&Precedence::None)?;

        self.expect(Token::Semicolon)?;

        Ok(Statement::Return(expr))
    }

fn parse_expression(&mut self, min_prec: &Precedence) -> Result<Expression, ParseError> {
        let mut left = self.parse_factor().unwrap();

        while let Some(Token::Addition)
        | Some(Token::Negation)
        | Some(Token::Division)
        | Some(Token::Multiplication)
        | Some(Token::Remainder)
        | Some(Token::BitwiseOr)
        | Some(Token::BitwiseXor)
        | Some(Token::BitwiseAnd)
        | Some(Token::LogicalOr)
        | Some(Token::LogicalAnd)
        | Some(Token::LeftShift)
        | Some(Token::RightShift)
        | Some(Token::Equal)
        | Some(Token::NotEqual)
        | Some(Token::LessThan)
        | Some(Token::LessThanEqual)
        | Some(Token::GreaterThan)
        | Some(Token::GreaterThanEqual) = self.tokens.peek()
        {
            let prec = self.tokens.peek().unwrap().get_precedence();
            if prec <= *min_prec {
                return Ok(left);
            }

            let op_token = self.tokens.next().unwrap();
            let right = self.parse_expression(&prec).unwrap();

            let bin_op = match op_token {
                Token::Addition => BinOp::Add,
                Token::Negation => BinOp::Subtract,
                Token::Division => BinOp::Divide,
                Token::Multiplication => BinOp::Multiply,
                Token::Remainder => BinOp::Remainder,
                Token::BitwiseOr => BinOp::BitwiseOr,
                Token::BitwiseAnd => BinOp::BitwiseAnd,
                Token::BitwiseXor => BinOp::BitwiseXor,
                Token::LogicalAnd=> BinOp::LogicalAnd,
                Token::LogicalOr=> BinOp::LogicalOr,
                Token::LeftShift => BinOp::LeftShift,
                Token::RightShift => BinOp::RightShift,
                Token::Equal => BinOp::Equal,
                Token::NotEqual => BinOp::NotEqual,
                Token::LessThan => BinOp::LessThan,
                Token::LessThanEqual => BinOp::LessThanEqual,
                Token::GreaterThan => BinOp::GreaterThan,
                Token::GreaterThanEqual => BinOp::GreaterThanEqual,
                _ => unreachable!(),
            };

            left = Expression::BinaryOp(bin_op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        match self.tokens.next() {
            Some(Token::OpenParen) => {
                let result = self.parse_expression(&Precedence::None);
                let _ = self.expect(Token::CloseParen);
                result
            }

            Some(Token::IntegerLiteral(val)) => Ok(Expression::Constant(val)),

            Some(Token::Negation) => Ok(Expression::UnaryOp((
                UnOp::Negation,
                Box::new(self.parse_factor().unwrap()),
            ))),

            Some(Token::BitwiseComplement) => Ok(Expression::UnaryOp((
                UnOp::BitwiseComplement,
                Box::new(self.parse_factor().unwrap()),
            ))),

            Some(Token::LogicalNegation) => Ok(Expression::UnaryOp((
                UnOp::LogicalNegation,
                Box::new(self.parse_factor().unwrap()),
            ))),

            Some(t) => Err(ParseError::UnexpectedToken {
                expected: "Factor parsing error".to_string(),
                found: format!("{:?}", t),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }
}
