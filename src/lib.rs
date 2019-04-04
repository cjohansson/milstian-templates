//! # Milstian Templates
//!
//! ![Milstian Logo](https://raw.githubusercontent.com/cjohansson/milstian-rust-internet-framework/master/html/img/logo1-modified.jpg)
//!
//! A template framework in Rust.

extern crate regex;

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
struct Variable {
    datum: DataType,
    name: String,
}

#[derive(Debug, PartialEq)]
enum LexerToken<'a> {
    And,
    Assign(&'a str, DataType),
    Call(&'a str, Vec<Variable>),
    CloseTag,
    Echo,
    EndForeach,
    ElseIf,
    EndIf,
    Equals,
    ForEach,
    If,
    Inline(&'a str),
    OpenTag,
    OpenTagWithEcho,
    Or,
    String(&'a str),
    Variable(&'a str),
}

#[derive(Debug, PartialEq)]
enum LexerState {
    Code,
    Initial,
}

#[derive(Debug, PartialEq)]
struct LexerPosition {
    char_end: usize,
    char_start: usize,
    line_end: usize,
    line_start: usize,
}

#[derive(Debug, PartialEq)]
struct LexerElement<'a> {
    position: LexerPosition,
    token: LexerToken<'a>,
}

#[derive(Debug, PartialEq)]
enum LexerTokenMatchPattern {
    Literal(String),
    Regex(String),
}

struct LexerTokenMatcher {
    logic: Box<
        Fn(
            &str, // Buffer
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
    pub fn test(&self, buffer: &str) -> usize {
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

    pub fn execute(
        &self,
        buffer: &String,
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
        (*self.logic)(buffer, char_index, char_start, char_end, length, line_index, line_start, line_end, elements, state)
    }
}

#[derive(Debug, PartialEq)]
pub struct Template<'a> {
    data: Option<HashMap<String, DataType>>,
    form: &'a str,
}

impl <'a> Template<'a> {
    pub fn new(form: &'a str, data: Option<HashMap<String, DataType>>) -> Template<'a> {
        Template { form, data }
    }

    pub fn process(self) -> Result<String, String> {
        match Template::lex(self.form) {
            Ok(lexer_elements) => match Template::parse(lexer_elements, self.data) {
                Ok(processed) => Ok(processed),
                Err(error) => Err(format!("Failed to parse tokens, error: {}", error)),
            },
            Err(error) => Err(format!("Failed to lex form, error: {}", error)),
        }
    }

    fn string_ends_with_string<'a, 'b>(needle: &'a str, buffer: &'b str) -> Option<(&'b str, &'a str)> {
        if buffer.len() >= needle.len() {
            let start = buffer.len() - needle.len();
            let ends_with = &buffer[start..];
            if ends_with.to_lowercase() == needle.to_lowercase() {
                let new_buffer = buffer[0..start].to_string();
                return Some((new_buffer, needle));
            }
        }
        None
    }

    fn string_ends_with_regex(old_needle: &str, buffer: &str) -> Option<(String, String)> {
        let needle = format!("{}$", old_needle);
        let pattern = Regex::new(&needle).unwrap();
        if pattern.is_match(buffer) {
            let hit = pattern.find(buffer).unwrap();
            let start = hit.start();
            let end = hit.end();
            let new_buffer = buffer[0..start].to_string();
            let matched_string = buffer[start..end].to_string();
            return Some((new_buffer, matched_string));
        }
        None
    }

    fn lex<'a>(form: &'a str) -> Result<Vec<LexerElement<'a>>, &str> {
        let mut char_index: usize = 1;
        let mut char_start: usize = 1;
        let mut char_end: usize = 1;
        let mut line_index: usize = 1;
        let mut line_start: usize = 1;
        let mut line_end: usize = 1;
        let mut elements: Vec<LexerElement> = Vec::new();
        let mut state = LexerState::Initial;
        let mut buffer = String::new();

        // New algorithm here
        let mut best_match_index: usize = 0;
        let mut best_match_length: usize = 0;
        let mut index: usize = 0;

        let mut items: Vec<LexerTokenMatcher> = Vec::new();

        // Setup lexer patterns here
        items.push(LexerTokenMatcher {
            logic: Box::new(
                |buffer: &str,
                 char_index: &usize,
                 char_start: &usize,
                 char_end: &usize,
                 length: &usize,
                 line_index: &usize,
                 line_start: &usize,
                 line_end: &usize,
                 elements: &mut Vec<LexerElement>,
                 state: &mut LexerState| {
                    println!("Was here");
                    let new_buffer = &buffer[*char_start..*char_end];
                    elements.push(LexerElement {
                        position: LexerPosition {
                            char_end: *char_end,
                            char_start: *char_start,
                            line_end: *line_end,
                            line_start: *line_start,
                        },
                        token: LexerToken::Inline(new_buffer),
                    });
                    elements.push(LexerElement {
                        position: LexerPosition {
                            char_end: (*char_index),
                            char_start: char_index - length,
                            line_end: (*line_index),
                            line_start: (*line_index),
                        },
                        token: LexerToken::OpenTag,
                    });
                    (*state) = LexerState::Code;
                },
            ),
            pattern: LexerTokenMatchPattern::Literal("{% ".to_string()),
            state: LexerState::Initial,
        });

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
                best_match.execute(&buffer, &char_index, &char_start, &char_end, &best_match_length, &line_start, &line_start, &line_end, &mut elements, &mut state);
                char_index = char_index + best_match_length;
            } else {
                char_index = char_index + 1;
            }
        }

        if elements.len() == 0 {
            elements.push(LexerElement {
                position: LexerPosition {
                    char_end: char_index,
                    char_start,
                    line_end: line_index,
                    line_start,
                },
                token: LexerToken::Inline(form),
            });
        }
        return Ok(elements);
    }

    fn parse(
        lexer_tokens: Vec<LexerElement>,
        data: Option<HashMap<String, DataType>>,
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
            Template::string_ends_with_string("andom", &buffer),
            Some(("Blaha R", "andom"))
        );
        assert_eq!(Template::string_ends_with_string("bandom", &buffer), None);
    }

    #[test]
    fn test_string_ends_with_regex() {
        let buffer = "Blaha Random";
        assert_eq!(
            Template::string_ends_with_regex(r" [a-zA-Z]+", &buffer),
            Some(("Blaha", " Random"))
        );
        assert_eq!(Template::string_ends_with_regex(r" [a-z]+", &buffer), None);

        let buffer = "Blaha Random123";
        assert_eq!(
            Template::string_ends_with_regex(r" [a-zA-Z]+", &buffer),
            None
        );

        let buffer = "Blaha $var";
        assert_eq!(
            Template::string_ends_with_regex(r"\$[a-zA-Z][a-zA-Z0-9_]*", &buffer),
            Some(("Blaha ", "$var"))
        );
    }

    #[test]
    fn test_process() {}

    #[test]
    fn test_parse() {}

    #[test]
    fn test_lex() {
        let lexed_tokens = Template::lex("Random").unwrap();
        let mut expected_lexed_tokens: Vec<LexerElement> = Vec::new();
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 7,
                char_start: 1,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Inline("Random"),
        });
        assert_eq!(lexed_tokens, expected_lexed_tokens);

        let lexed_tokens = Template::lex("Random {% echo $var %}").unwrap();
        let mut expected_lexed_tokens: Vec<LexerElement> = Vec::new();
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 8,
                char_start: 1,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Inline("Random "),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 11,
                char_start: 8,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::OpenTag,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 15,
                char_start: 11,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Echo,
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 20,
                char_start: 16,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Variable("var"),
        });
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 23,
                char_start: 20,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::CloseTag,
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
