//! # Milstian Templates
//!
//! ![Milstian Logo](https://raw.githubusercontent.com/cjohansson/milstian-rust-internet-framework/master/html/img/logo1-modified.jpg)
//!
//! A template framework in Rust.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

pub enum DataType {
    Float(f32),
    HashMap(HashMap<String, DataType>),
    Integer(isize),
    String(String),
    Vector(Box<DataType>),
}

struct Variable {
    datum: DataType,
    name: String,
}

enum LexerToken {
    And,
    CustomFunction(String, Vec<Variable>),
    Echo,
    ElseIf,
    EndIf,
    Equals,
    ForEach,
    If,
    Or,
    String(String),
    Variable,
}

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
                Ok(lexer_tokens) => match Template::parse(lexer_tokens, self.data) {
                    Ok(processed) => Ok(processed),
                    Err(error) => Err(format!("Failed to parse tokens, error: {}", error)),
                },
                Err(error) => Err(format!("Failed to lex form, error: {}", error)),
            }
        } else {
            Err("No form specified for template!".to_string())
        }
    }

    fn lex(form: String) -> Result<Vec<LexerToken>, String> {
        Err("Failed to lex form".to_string())
    }

    fn parse(
        lexer_tokens: Vec<LexerToken>,
        data: Option<HashMap<String, DataType>>,
    ) -> Result<String, String> {
        Err("Failed to parse tokens".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let template = Template::new(Some("Random".to_string()), None);
        let processed = template.process();
        assert_eq!(processed.unwrap(), "Random".to_string());
    }

    #[test]
    fn test_parse() {}

    #[test]
    fn test_lex() {}

    #[test]
    fn test_set_datum() {}

    #[test]
    fn test_set_data() {}

    #[test]
    fn test_set_form() {}

    #[test]
    fn test_set_file() {}
}
