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
    AdditionOne,
    And,
    Assign(String, DataType),
    Call(String, Vec<Variable>),
    CloseTag,
    CloseTagWithEcho,
    Division,
    DoubleQuotedString(String),
    Echo,
    EndForeach,
    ElseIf,
    EndIf,
    Equals,
    ForEach,
    If,
    Inline(String),
    Multiplication,
    OpenTag,
    OpenTagWithEcho,
    Or,
    SingleQuotedString(String),
    StringConcatenation,
    Subtraction,
    SubtractionOne,
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
            &str,   // Buffer
            &usize, // Character index
            &usize, // Character start
            &usize, // Character end
            &usize, // Match length
            &usize, // Line index
            &usize, // Line start
            &usize, // Line end
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
        length: &usize,
        line_index: &usize,
        line_start: &usize,
        line_end: &usize,
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

    fn string_ends_with_string<'b, 'c>(
        buffer: &'b str,
        needle: &'c str,
    ) -> Option<(&'b str, &'c str)> {
        if buffer.len() >= needle.len() {
            let start = buffer.len() - needle.len();
            let ends_with = &buffer[start..];
            if ends_with.to_lowercase() == needle.to_lowercase() {
                let new_buffer = &buffer[0..start];
                return Some((new_buffer, needle));
            }
        }
        None
    }

    fn string_ends_with_regex<'b, 'c>(
        buffer: &'b str,
        old_needle: &'c str,
    ) -> Option<(&'b str, &'b str)> {
        let needle = format!("{}$", old_needle);
        let pattern = Regex::new(&needle).unwrap();
        if pattern.is_match(buffer) {
            let hit = pattern.find(buffer).unwrap();
            let start = hit.start();
            let end = hit.end();
            let new_buffer = &buffer[0..start];
            let matched_string = &buffer[start..end];
            return Some((new_buffer, matched_string));
        }
        None
    }

    fn lex(&self) -> Result<Vec<LexerElement>, &str> {
        let form: &str = &self.form;
        let mut char_index: usize = 0;
        let mut char_start: usize = 0;
        let mut char_end: usize = 1;
        let mut line_index: usize = 1;
        let mut line_start: usize = 1;
        let mut line_end: usize = 1;
        let mut elements: Vec<LexerElement> = Vec::new();
        let mut state = LexerState::Initial;

        // New algorithm here
        let mut best_match_index: usize = 0;
        let mut best_match_length: usize;
        let mut index: usize;

        let items = tokens::get_lexer_items();
        while char_index < form.len() {
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
                    &best_match_length,
                    &line_start,
                    &line_start,
                    &line_end,
                    &mut elements,
                    &mut state,
                );
                char_index = char_index + best_match_length;
                char_start = char_index;
                char_end = char_index;
            } else {
                char_end = char_index;
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
        lexer_tokens: Vec<LexerElement>,
        data: &Option<HashMap<String, DataType>>,
    ) -> Result<String, String> {
        Err("Failed to parse tokens".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_ends_with_string() {
        let buffer = "Blaha Random";
        assert_eq!(
            Template::string_ends_with_string(&buffer, "andom"),
            Some(("Blaha R", "andom"))
        );
        assert_eq!(Template::string_ends_with_string(&buffer, "bandom"), None);
    }

    #[test]
    fn test_string_ends_with_regex() {
        let buffer = "Blaha Random";
        assert_eq!(
            Template::string_ends_with_regex(&buffer, r" [a-zA-Z]+"),
            Some(("Blaha", " Random"))
        );
        assert_eq!(Template::string_ends_with_regex(&buffer, r" [a-z]+"), None);

        let buffer = "Blaha Random123";
        assert_eq!(
            Template::string_ends_with_regex(&buffer, r" [a-zA-Z]+"),
            None
        );

        let buffer = "Blaha $var";
        assert_eq!(
            Template::string_ends_with_regex(&buffer, r"\$[a-zA-Z][a-zA-Z0-9_]*"),
            Some(("Blaha ", "$var"))
        );
    }

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

        let lexed_tokens = Template::new("Random {% echo $var %} More text here {{ $var }}".to_string(), None).lex().unwrap();
        let mut expected_lexed_tokens: Vec<LexerElement> = Vec::new();
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 6,
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
            token: LexerToken::Echo,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 19,
                char_start: 15,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Variable("var".to_string()),
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
                char_end: 37,
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
                char_end: 45,
                char_start: 41,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Variable("var".to_string()),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 48,
                char_start: 45,
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
