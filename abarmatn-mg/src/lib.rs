#![allow(dead_code, unused_imports)]

use crate::parser::Parser;

pub mod parser;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parser::{get_keys, get_blocks};

    use super::*;
    #[test]
    fn test_get_keys() {
        let text = "hello {{name}} {{last_name}}";

        assert_eq!(get_keys(text).0, vec!["name", "last_name"])
    }

    #[test]
    fn test_get_blocks() {
        let text = "hello {{ name }} {{ last_name }}";

        assert_eq!(get_blocks(text), vec!["{{ name }}", "{{ last_name }}"])
    }

    #[test]
    fn test_parse() {
        let text = String::from("hello {{ name }} {{ last_name }}");

        let parsed = Parser::new(text).parse(HashMap::from([("name", "John"), ("last_name", "Doe")]));

        assert_eq!(parsed, "hello John Doe")
    }
}
