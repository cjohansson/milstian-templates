//! # Milstian Templates
//!
//! ![Milstian Logo](https://raw.githubusercontent.com/cjohansson/milstian-rust-internet-framework/master/html/img/logo1-modified.jpg)
//!
//! A template framework in Rust.

extern crate regex;

pub mod tokens;

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum DataType {
    Float(f32),
    HashMap(HashMap<String, DataType>),
    Integer(isize),
    String(String),
    Vector(Box<DataType>),
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    datum: DataType,
    name: String,
}

#[derive(Debug, PartialEq)]
pub enum LexerToken {
    Addition,
    AddOne,
    And,
    As,
    Assign,
    Call(String),
    CloseCurlyBracket,
    CloseParenthesis,
    CloseTag,
    CloseTagWithEcho,
    Comma,
    Division,
    DoubleQuotedString(String),
    EndForEach,
    Else,
    ElseIf,
    EndIf,
    Equals,
    Float(String),
    ForEach,
    GreaterThan,
    GreaterOrEqualThan,
    If,
    Inline(String),
    Integer(String),
    LesserThan,
    LesserOrEqualThan,
    LesserOrGreaterThan,
    Negation,
    Multiplication,
    OpenCurlyBracket,
    OpenParenthesis,
    OpenTag,
    OpenTagWithEcho,
    Or,
    Semicolon,
    SingleQuotedString(String),
    StringConcatenation,
    Subtraction,
    SubtractOne,
    Variable(String),
}

#[derive(Debug, PartialEq)]
pub enum LexerState {
    Code,
    Initial,
}

#[derive(Debug, PartialEq)]
pub struct LexerPosition {
    char_end: usize,
    char_start: usize,
    line_end: usize,
    line_start: usize,
}

#[derive(Debug, PartialEq)]
pub struct LexerElement {
    position: LexerPosition,
    token: LexerToken,
}

#[derive(Debug, PartialEq)]
pub enum LexerTokenMatchPattern {
    Literal(String),
    Regex(String),
}

pub struct LexerTokenMatcher {
    logic: Box<
        Fn(
            &str,       // Buffer
            &usize,     // Character index
            &usize,     // Character start
            &usize,     // Character end
            &mut usize, // Match length
            &usize,     // Line index
            &usize,     // Line start
            &mut usize, // Line end
            &mut Vec<LexerElement>,
            &mut LexerState,
        ),
    >,
    pattern: LexerTokenMatchPattern,
    pub state: LexerState,
}

impl LexerTokenMatcher {
    pub fn test<'a>(&self, buffer: &'a str) -> usize {
        match &self.pattern {
            LexerTokenMatchPattern::Literal(pattern) => {
                if buffer.len() >= pattern.len() {
                    let end = pattern.len();
                    let starts_with = &buffer[0..end];
                    if starts_with.to_lowercase() == pattern.to_lowercase() {
                        return pattern.len();
                    }
                }
            }
            LexerTokenMatchPattern::Regex(pattern) => {
                let needle = format!("^{}", pattern);
                let re_pattern = Regex::new(&needle).unwrap();
                if let Some(pattern_match) = re_pattern.find(&buffer) {
                    return pattern_match.end();
                }
            }
        }
        0
    }

    pub fn execute<'a>(
        &self,
        buffer: &'a str,
        char_index: &usize,
        char_start: &usize,
        char_end: &usize,
        length: &mut usize,
        line_index: &usize,
        line_start: &usize,
        line_end: &mut usize,
        elements: &mut Vec<LexerElement>,
        state: &mut LexerState,
    ) {
        (*self.logic)(
            buffer, char_index, char_start, char_end, length, line_index, line_start, line_end,
            elements, state,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Template {
    data: Option<HashMap<String, DataType>>,
    form: String,
}

impl Template {
    pub fn new(form: String, data: Option<HashMap<String, DataType>>) -> Template {
        Template { form, data }
    }

    pub fn process(&self) -> Result<String, String> {
        match self.lex() {
            Ok(lexer_elements) => match Template::parse(lexer_elements, &self.data) {
                Ok(processed) => Ok(processed),
                Err(error) => Err(format!("Failed to parse tokens, error: {}", error)),
            },
            Err(error) => Err(format!("Failed to lex form, error: {}", error)),
        }
    }

    fn lex(&self) -> Result<Vec<LexerElement>, &str> {
        let form: &str = &self.form;
        // What char are we at
        let mut char_index: usize = 0;

        // Start of latest match
        let mut char_start: usize = 0;

        // End of latest match
        let mut char_end: usize = 0;

        let mut line_index: usize = 1;
        let line_start: usize = 1;
        let mut line_end: usize = 1;
        let mut elements: Vec<LexerElement> = Vec::new();
        let mut state = LexerState::Initial;

        // New algorithm here
        let mut best_match_index: usize = 0;
        let mut best_match_length: usize;
        let mut index: usize;

        let items = tokens::get_lexer_items();
        while char_index < form.len() {
            println!(
                "char_start: {}, char_end: {}, char_index: {}",
                char_start, char_end, char_index
            );
            // TODO: Should track line numbers here
            best_match_length = 0;
            index = 0;
            for item in &items {
                if item.state == state {
                    let match_length = item.test(&form[char_index..]);
                    if match_length > 0 {
                        if match_length > best_match_length {
                            best_match_length = match_length;
                            best_match_index = index;
                        }
                    }
                }
                index = index + 1;
            }

            if best_match_length > 0 {
                let best_match = items.get(best_match_index).unwrap();
                best_match.execute(
                    &form,
                    &char_index,
                    &char_start,
                    &char_end,
                    &mut best_match_length,
                    &line_start,
                    &line_start,
                    &mut line_end,
                    &mut elements,
                    &mut state,
                );
                char_start = char_index;
                char_index = char_index + best_match_length;
                char_end = char_index;
                line_index = line_end;
            } else {
                char_index = char_index + 1;
            }
        }

        if elements.len() == 0 {
            elements.push(LexerElement {
                position: LexerPosition {
                    char_end: form.len() - 1,
                    char_start,
                    line_end: line_index,
                    line_start,
                },
                token: LexerToken::Inline(form.to_string()),
            });
        }
        return Ok(elements);
    }

    fn parse(
        _lexer_tokens: Vec<LexerElement>,
        _data: &Option<HashMap<String, DataType>>,
    ) -> Result<String, String> {
        Err("Failed to parse tokens".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {}

    #[test]
    fn test_parse() {}

    #[test]
    fn test_lex() {
        let lexed_tokens = Template::new("Random".to_string(), None).lex().unwrap();
        let mut expected_lexed_tokens: Vec<LexerElement> = Vec::new();
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 5,
                char_start: 0,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Inline("Random".to_string()),
        });
        assert_eq!(lexed_tokens, expected_lexed_tokens);

        let lexed_tokens = Template::new(
            "Random {% echo(var) %} More text here {{ \"random string\" }}".to_string(),
            None,
        )
        .lex()
        .unwrap();
        let mut expected_lexed_tokens: Vec<LexerElement> = Vec::new();
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 7,
                char_start: 0,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Inline("Random ".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 10,
                char_start: 7,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::OpenTag,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 14,
                char_start: 10,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Call("echo".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 15,
                char_start: 14,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::OpenParenthesis,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 18,
                char_start: 15,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Variable("var".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 19,
                char_start: 18,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::CloseParenthesis,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 22,
                char_start: 19,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::CloseTag,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 38,
                char_start: 22,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Inline(" More text here ".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 41,
                char_start: 38,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::OpenTagWithEcho,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 55,
                char_start: 41,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::DoubleQuotedString("random string".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 59,
                char_start: 56,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::CloseTagWithEcho,
        });
        assert_eq!(lexed_tokens, expected_lexed_tokens);

        let lexed_tokens = Template::new(
            "{% if a > b { echo(a); } else { echo(b); } %}".to_string(),
            None,
        )
        .lex()
        .unwrap();
        let mut expected_lexed_tokens: Vec<LexerElement> = Vec::new();
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 3,
                char_start: 0,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::OpenTag,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 5,
                char_start: 3,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::If,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 7,
                char_start: 6,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Variable("a".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 9,
                char_start: 8,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::GreaterThan,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 11,
                char_start: 10,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Variable("b".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 13,
                char_start: 12,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::OpenCurlyBracket,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 18,
                char_start: 14,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Call("echo".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 19,
                char_start: 18,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::OpenParenthesis,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 20,
                char_start: 19,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Variable("a".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 21,
                char_start: 20,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::CloseParenthesis,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 22,
                char_start: 21,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Semicolon,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 24,
                char_start: 23,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::CloseCurlyBracket,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 29,
                char_start: 25,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Else,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 31,
                char_start: 30,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::OpenCurlyBracket,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 36,
                char_start: 32,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Call("echo".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 37,
                char_start: 36,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::OpenParenthesis,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 38,
                char_start: 37,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Variable("b".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 39,
                char_start: 38,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::CloseParenthesis,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 40,
                char_start: 39,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Semicolon,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 42,
                char_start: 41,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::CloseCurlyBracket,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 45,
                char_start: 42,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::CloseTag,
        });
        assert_eq!(lexed_tokens, expected_lexed_tokens);

        let lexed_tokens = Template::new(
            "{% a = 1; a++; a--; b = 3; echo(a/3); echo(\"was here\"); (a == b); a = 1.0; foreach (a as b) {} a <= b; b >= a; a || b; a && b; a - b; a + b; a * b; a <> b %}".to_string(),
            None,
        )
        .lex()
        .unwrap();
        let mut expected_lexed_tokens: Vec<LexerElement> = Vec::new();
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 7,
                char_start: 0,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Inline("Random ".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 10,
                char_start: 7,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::OpenTag,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 14,
                char_start: 10,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Call("echo".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 15,
                char_start: 14,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::OpenParenthesis,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 18,
                char_start: 15,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Variable("var".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 19,
                char_start: 18,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::CloseParenthesis,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 22,
                char_start: 19,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::CloseTag,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 38,
                char_start: 22,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Inline(" More text here ".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 41,
                char_start: 38,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::OpenTagWithEcho,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 55,
                char_start: 41,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::DoubleQuotedString("random string".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 59,
                char_start: 56,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::CloseTagWithEcho,
        });
        assert_eq!(lexed_tokens, expected_lexed_tokens);
    }

    #[test]
    fn test_set_datum() {}

    #[test]
    fn test_set_data() {}

    #[test]
    fn test_set_form() {}

    #[test]
    fn test_set_file() {}
}
