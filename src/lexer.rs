use std::fs;
use std::io;
use std::iter::Peekable;
use std::path::Path;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBrace,           // {
    CloseBrace,          // }
    OpenParen,           // (
    CloseParen,          // )
    Semicolon,           // ;
    IntKeyword,          // int
    ReturnKeyword,       // return
    Identifier(String),  // [a-zA-Z]\w*
    IntegerLiteral(i32), // [0-9]+
    Negation,            // -
    BitwiseComplement,   // ~
    LogicalNegation,     // !
    Addition,            // +
    Multiplication,      // *
    Division,            // /
    Decrement,
    Remainder,
}

#[derive(Debug)]
pub enum LexError {
    IoError(io::Error),
    UnknownToken(char, usize), // char, position
}

impl From<io::Error> for LexError {
    fn from(e: io::Error) -> Self {
        LexError::IoError(e)
    }
}

pub fn lex<P: AsRef<Path>>(file_path: P) -> Result<Vec<Token>, LexError> {
    let content = fs::read_to_string(file_path)?;

    let mut tokens = Vec::new();
    let mut chars = content.chars().peekable();
    let mut pos = 0;

    while let Some(&c) = chars.peek() {
        match c {
            c if c.is_whitespace() => {
                chars.next();
                pos += 1;
            }

            '{' => {
                tokens.push(Token::OpenBrace);
                chars.next();
                pos += 1;
            }
            '}' => {
                tokens.push(Token::CloseBrace);
                chars.next();
                pos += 1;
            }
            '(' => {
                tokens.push(Token::OpenParen);
                chars.next();
                pos += 1;
            }
            ')' => {
                tokens.push(Token::CloseParen);
                chars.next();
                pos += 1;
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
                pos += 1;
            }
            '-' => {
                chars.next();
                pos += 1;

                match chars.peek() {
                    Some(&'-') => {
                        chars.next();
                        pos += 1;
                        tokens.push(Token::Decrement);
                    }
                    _ => {
                        tokens.push(Token::Negation);
                    }
                }
            }
            '~' => {
                tokens.push(Token::BitwiseComplement);
                chars.next();
                pos += 1;
            }
            '!' => {
                tokens.push(Token::LogicalNegation);
                chars.next();
                pos += 1;
            }
            '+' => {
                tokens.push(Token::Addition);
                chars.next();
                pos += 1;
            }
            '*' => {
                tokens.push(Token::Multiplication);
                chars.next();
                pos += 1;
            }
            '/' => {
                tokens.push(Token::Division);
                chars.next();
                pos += 1;
            }
            '%' => {
                tokens.push(Token::Remainder);
                chars.next();
                pos += 1;
            }
            c if c.is_ascii_alphabetic() => {
                let text = consume_while(&mut chars, |ch| ch.is_ascii_alphanumeric() || ch == '_');
                pos += text.len();

                let token = match text.as_str() {
                    "int" => Token::IntKeyword,
                    "return" => Token::ReturnKeyword,
                    _ => Token::Identifier(text),
                };
                tokens.push(token);
            }

            c if c.is_ascii_digit() => {
                let num_str = consume_while(&mut chars, |ch| ch.is_ascii_digit());
                pos += num_str.len();

                let value: i32 = num_str.parse().unwrap_or(0);
                tokens.push(Token::IntegerLiteral(value));
            }

            _ => {
                return Err(LexError::UnknownToken(c, pos));
            }
        }
    }

    Ok(tokens)
}

fn consume_while<F>(chars: &mut Peekable<Chars>, predicate: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut result = String::new();
    while let Some(&c) = chars.peek() {
        if predicate(c) {
            result.push(c);
            chars.next();
        } else {
            break;
        }
    }
    result
}
