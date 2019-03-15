//! # Milstian Templates
//!
//! ![Milstian Logo](https://raw.githubusercontent.com/cjohansson/milstian-rust-internet-framework/master/html/img/logo1-modified.jpg)

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

pub enum DataType {
    Float(f32),
    HashMap(HashMap<String, DataType>),
    Integer(isize),
    String(String),
    Vector(Box<DataType>),
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
            if let Some(data) = self.data {
                // TODO Attach data here
            }

            let processed = form.clone();
            // TODO Process template here
            Ok(processed)
        } else {
            Err("No form specified for template!".to_string())
        }
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
