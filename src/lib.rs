//! # Milstian Templates
//!
//! ![Milstian Logo](https://raw.githubusercontent.com/cjohansson/milstian-rust-internet-framework/master/html/img/logo1-modified.jpg)
//!
//! A template framework in Rust.

use std::collections::hash_map::DefaultHasher;
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
    CustomFunction(String, Vec<Variable>),
    Echo,
    EndForeach,
    ElseIf,
    EndIf,
    Equals,
    ForEach,
    If,
    Or,
    String(String),
    Variable,
}

#[derive(Debug, PartialEq)]
struct LexerPosition {
    char_end: u32,
    char_start: u32,
    line_end: u32,
    line_start: u32,
}

#[derive(Debug, PartialEq)]
struct LexerElement {
    position: LexerPosition,
    token: LexerToken,
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

    fn lex(form: String) -> Result<Vec<LexerElement>, String> {
        Err("Failed to lex form".to_string())
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
    use Template::*;

    #[test]
    fn test_process() {}

    #[test]
    fn test_parse() {}

    #[test]
    fn test_lex() {
        let lexed_tokens = Template::lex("Random".to_string()).unwrap();
        let expected_lexed_tokens: Vec<LexerElement> = Vec::new();
        expected_lexed_tokens.push(LexerElement {
            position: LexerPosition {
                char_end: 10,
                char_start: 0,
                line_end: 1,
                line_start: 1,
            },
            token: LexerToken::String("Random".to_string()),
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
