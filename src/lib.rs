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
enum LexerToken {
    And,
    Assign(String, DataType),
    Call(String, Vec<Variable>),
    CloseTag,
    Echo,
    EndForeach,
    ElseIf,
    EndIf,
    Equals,
    ForEach,
    If,
    Inline(String),
    OpenTag,
    OpenTagWithEcho,
    Or,
    String(String),
    Variable(String),
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
struct LexerElement {
    position: LexerPosition,
    token: LexerToken,
}

#[derive(Debug, PartialEq)]
enum LexerTokenMatchPattern {
    Literal(String),
    Regex(String),
}

struct LexerTokenMatcher {
    match_length: usize,
    logic: Box<Fn(&String, u32, &mut Vec<LexerElement>, &mut LexerState)>,
    matches: bool,
    pattern: LexerTokenMatchPattern,
}

impl LexerTokenMatcher {
    fn test(self, buffer: String) -> bool {
        match self.pattern {
            LexerTokenMatchPattern::Literal(pattern) => {
                self.matches = false;
                self.match_length = pattern.len();
                if buffer.len() >= pattern.len() {
                    let end = pattern.len();
                    let starts_with = &buffer[0..end];
                    if starts_with.to_lowercase() == pattern.to_lowercase() {
                        self.match_length = pattern.len();
                    }
                }
                return self.matches;
            }
            LexerTokenMatchPattern::Regex(pattern) => {
                self.matches = false;
                self.match_length = pattern.len();
                let needle = format!("^{}", pattern);
                let re_pattern = Regex::new(&needle).unwrap();
                if let Some(pattern_match) = re_pattern.find(&buffer) {
                    self.match_length = pattern_match.end();
                    self.matches = true;
                }
                return self.matches;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Template {
    data: Option<HashMap<String, DataType>>,
    form: Option<String>,
}

impl Template {
    pub fn new(form: Option<String>, data: Option<HashMap<String, DataType>>) -> Template {
        Template { form, data }
    }

    pub fn process(self) -> Result<String, String> {
        if let Some(form) = self.form {
            match Template::lex(form) {
                Ok(lexer_elements) => match Template::parse(lexer_elements, self.data) {
                    Ok(processed) => Ok(processed),
                    Err(error) => Err(format!("Failed to parse tokens, error: {}", error)),
                },
                Err(error) => Err(format!("Failed to lex form, error: {}", error)),
            }
        } else {
            Err("No form specified for template!".to_string())
        }
    }

    fn string_ends_with_string<'a>(needle: &'a str, buffer: &String) -> Option<(String, &'a str)> {
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

    fn lex(form: String) -> Result<Vec<LexerElement>, String> {
        let mut char_index: usize = 1;
        let mut char_start: usize = 1;
        let mut line_index: usize = 1;
        let mut line_start: usize = 1;
        let mut elements: Vec<LexerElement> = Vec::new();
        let mut state = LexerState::Initial;
        let mut buffer = String::new();

        // New algorithm here
        let mut best_match_logic: Fn(TokenPattern, String, u32, &LexerState);
        let mut best_match_length: u32 = 0;

        while char_index < form.len() {
            
        }


        // Old algorithm here
        for character in form.chars() {
            buffer.push(character);
            match state {
                LexerState::Initial => {
                    if let Some((new_buffer, pattern)) =
                        Template::string_ends_with_string("{% ", &buffer)
                    {
                        elements.push(LexerElement {
                            position: LexerPosition {
                                char_end: char_index - new_buffer.len(),
                                char_start,
                                line_end: line_index,
                                line_start,
                            },
                            token: LexerToken::Inline(new_buffer),
                        });
                        elements.push(LexerElement {
                            position: LexerPosition {
                                char_end: char_index,
                                char_start: char_index - pattern.len(),
                                line_end: line_index,
                                line_start: line_index,
                            },
                            token: LexerToken::OpenTag,
                        });
                        char_start = char_index;
                        line_start = line_index;
                        buffer = String::new();
                        state = LexerState::Code;
                    }
                }
                LexerState::Code => {
                    if let Some((_, pattern)) = Template::string_ends_with_string(" %}", &buffer) {
                        elements.push(LexerElement {
                            position: LexerPosition {
                                char_end: char_index,
                                char_start: char_index - pattern.len(),
                                line_end: line_index,
                                line_start: line_index,
                            },
                            token: LexerToken::CloseTag,
                        });
                        char_start = char_index;
                        line_start = line_index;
                        buffer = String::new();
                        state = LexerState::Initial;
                    } else if let Some((_, pattern)) =
                        Template::string_ends_with_string("echo ", &buffer)
                    {
                        elements.push(LexerElement {
                            position: LexerPosition {
                                char_end: char_index,
                                char_start: char_index - pattern.len(),
                                line_end: line_index,
                                line_start: line_index,
                            },
                            token: LexerToken::Echo,
                        });
                        char_start = char_index;
                        line_start = line_index;
                        buffer = String::new();
                    } else if let Some((_, pattern)) =
                        Template::string_ends_with_regex(r"\$[a-zA-Z][a-zA-Z0-9_]*", &buffer)
                    {
                        elements.push(LexerElement {
                            position: LexerPosition {
                                char_end: char_index,
                                char_start: char_index - pattern.len(),
                                line_end: line_index,
                                line_start: line_index,
                            },
                            token: LexerToken::Variable(pattern.to_string()),
                        });
                        buffer = String::new();
                    }
                }
            }
            char_index = char_index + 1;
            if character == '\n' {
                line_index = line_index + 1;
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
    fn test_string_ends_with_string()
    {
        let buffer = "Blaha Random".to_string();
        assert_eq!(
            Template::string_ends_with_string("andom", &buffer),
            Some(("Blaha R".to_string(), "andom"))
        );
        assert_eq!(
            Template::string_ends_with_string("bandom", &buffer),
            None
        );
    }

    #[test]
    fn test_string_ends_with_regex()
    {
        let buffer = "Blaha Random";
        assert_eq!(
            Template::string_ends_with_regex(r" [a-zA-Z]+", &buffer),
            Some(("Blaha".to_string(), " Random".to_string()))
        );
        assert_eq!(
            Template::string_ends_with_regex(r" [a-z]+", &buffer),
            None
        );

        let buffer = "Blaha Random123";
        assert_eq!(
            Template::string_ends_with_regex(r" [a-zA-Z]+", &buffer),
            None
        );

        let buffer = "Blaha $var";
        assert_eq!(
            Template::string_ends_with_regex(r"\$[a-zA-Z][a-zA-Z0-9_]*", &buffer),
            Some(("Blaha ".to_string(), "$var".to_string()))
        );
    }

    #[test]
    fn test_process() {}

    #[test]
    fn test_parse() {}

    #[test]
    fn test_lex() {
        let lexed_tokens = Template::lex("Random".to_string()).unwrap();
        let mut expected_lexed_tokens: Vec<LexerElement> = Vec::new();
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 7,
                char_start: 1,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Inline("Random".to_string()),
        });
        assert_eq!(lexed_tokens, expected_lexed_tokens);

        let lexed_tokens = Template::lex("Random {% echo $var %}".to_string()).unwrap();
        let mut expected_lexed_tokens: Vec<LexerElement> = Vec::new();
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 8,
                char_start: 1,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::Inline("Random ".to_string()),
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
            token: LexerToken::Variable("var".to_string()),
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
