use std::collections::HashMap;

#[derive(Debug)]
pub enum Token {
    Text(String),
    Variable(String),
}

pub struct TemplateParser {
    template: String,
    tokens: Vec<Token>,
}

impl TemplateParser {
    pub fn new(template: &str) -> TemplateParser {
        TemplateParser {
            template: template.to_string(),
            tokens: vec![],
        }
    }

    pub fn parse(&mut self) {
        let chars = self.template.chars().collect::<Vec<_>>();
        let mut index = 0;

        while index < chars.len() {
            let c = chars[index];

            if c == '{' {
                if let Some(next) = chars.get(index + 1) {
                    match next {
                        '{' => {
                            index += 2;
                            if let Some(variable_end) = self.find_variable_end(&chars[index..]) {
                                let variable = chars[index..index + variable_end]
                                    .into_iter()
                                    .collect();
                                self.tokens.push(Token::Variable(variable));
                                index += variable_end + 2;
                            } else {
                                // Invalid variable, treat it as text
                                self.tokens.push(Token::Text("{{".to_string()));
                                index += 2;
                            }
                        }
                        _ => self.tokens.push(Token::Text(c.to_string())),
                    }
                }
            } else {
                self.tokens.push(Token::Text(c.to_string()));
            }

            index += 1;
        }
    }

    fn find_variable_end(&self, chars: &[char]) -> Option<usize> {
        let mut index = 0;
        let mut open_count = 0;

        while index < chars.len() {
            if chars[index] == '}' {
                if let Some(next) = chars.get(index + 1) {
                    if *next == '}' {
                        if open_count == 0 {
                            return Some(index);
                        } else {
                            open_count -= 1;
                        }
                    }
                }
            } else if chars[index] == '{' {
                if let Some(next) = chars.get(index + 1) {
                    if *next == '{' {
                        open_count += 1;
                    }
                }
            }

            index += 1;
        }

        None
    }

    pub fn render(&self, data: &HashMap<String, String>) -> String {
        let mut output = String::new();

        for token in &self.tokens {
            match token {
                Token::Text(text) => output.push_str(text),
                Token::Variable(variable) => {
                    let value = self.evaluate_variable(variable, data);
                    output.push_str(&value);
                }
            }
        }

        output
    }

    fn evaluate_variable(&self, variable: &str, data: &HashMap<String, String>) -> String {
        let mut parts = variable.split('|');
        let variable_name = parts.next().unwrap();
        let mut value = data.get(variable_name).map(|v| v.to_string()).unwrap_or_default();

        for function in parts {
            value = match function {
                "uppercase" => value.to_uppercase(),
                "lowercase" => value.to_lowercase(),
                _ => value, // Unknown function, keep the original value
            };
        }

        value
    }
}