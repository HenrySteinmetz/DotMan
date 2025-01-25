use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Variable(String),
    Literal(String),
}

#[derive(Debug, Clone, PartialEq)]
enum Expression {
    VariableAssignment(String, Value),
    VariableValue(String),
    StringLiteral(String),
    IfStatement(Condition, Box<Expression>),
    Comment,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Variable(String),
    String(String),
    If,
    Assignment,
    Condition(bool),
    Comment,
}

impl Expression {
    fn from_string(string: String) -> Self {
        let tokens = Self::tokenize_line(string);

        Expression::from_tokens(tokens)
    }

    fn from_tokens(tokens: Vec<Token>) -> Self {
        let mut token_iter = tokens.into_iter();

        match token_iter.next() {
            Some(token) => match token {
                Token::Comment => return Self::Comment,
                Token::String(str) => return Self::StringLiteral(str),
                Token::If => {
                    if let Some(next_token) = token_iter.next() {
                        match next_token {
                            Token::Variable(var) => {
                                if let Some(next_token) = token_iter.next() {
                                    match next_token {
                                        Token::Condition(bool) => {
                                            if let Some(next_token) = token_iter.next() {
                                                match next_token {
                                                    Token::String(str) if bool => {
                                                        return Self::IfStatement(
                                                            Condition::IsEqual(
                                                                Value::Variable(var),
                                                                Value::Literal(str),
                                                            ),
                                                            Box::new(Self::from_tokens(
                                                                token_iter.collect(),
                                                            )),
                                                        )
                                                    }
                                                    Token::String(str) => {
                                                        return Self::IfStatement(
                                                            Condition::IsNotEqual(
                                                                Value::Variable(var),
                                                                Value::Literal(str),
                                                            ),
                                                            Box::new(Self::from_tokens(
                                                                token_iter.collect(),
                                                            )),
                                                        )
                                                    }
                                                    Token::Variable(var2) if bool => {
                                                        return Self::IfStatement(
                                                            Condition::IsEqual(
                                                                Value::Variable(var),
                                                                Value::Variable(var2),
                                                            ),
                                                            Box::new(Self::from_tokens(
                                                                token_iter.collect(),
                                                            )),
                                                        )
                                                    }
                                                    Token::Variable(var2) => {
                                                        return Self::IfStatement(
                                                            Condition::IsNotEqual(
                                                                Value::Variable(var),
                                                                Value::Variable(var2),
                                                            ),
                                                            Box::new(Self::from_tokens(
                                                                token_iter.collect(),
                                                            )),
                                                        )
                                                    }
                                                    _ => panic!("Expected variable or literal!"),
                                                }
                                            } else {
                                                panic!("Missing value!");
                                            }
                                        }
                                        _ => panic!("Expected comparison!"),
                                    }
                                } else {
                                    panic!("Missing comparison operator and value!");
                                }
                            }
                            Token::String(str) => {
                                if let Some(next_token) = token_iter.next() {
                                    match next_token {
                                        Token::Condition(bool) => {
                                            if let Some(next_token) = token_iter.next() {
                                                match next_token {
                                                    Token::String(str2) if bool => {
                                                        return Self::IfStatement(
                                                            Condition::IsEqual(
                                                                Value::Literal(str),
                                                                Value::Literal(str2),
                                                            ),
                                                            Box::new(Self::from_tokens(
                                                                token_iter.collect(),
                                                            )),
                                                        )
                                                    }
                                                    Token::String(str2) => {
                                                        return Self::IfStatement(
                                                            Condition::IsNotEqual(
                                                                Value::Literal(str),
                                                                Value::Literal(str2),
                                                            ),
                                                            Box::new(Self::from_tokens(
                                                                token_iter.collect(),
                                                            )),
                                                        )
                                                    }
                                                    Token::Variable(var2) if bool => {
                                                        return Self::IfStatement(
                                                            Condition::IsEqual(
                                                                Value::Literal(str),
                                                                Value::Variable(var2),
                                                            ),
                                                            Box::new(Self::from_tokens(
                                                                token_iter.collect(),
                                                            )),
                                                        )
                                                    }
                                                    Token::Variable(var2) => {
                                                        return Self::IfStatement(
                                                            Condition::IsNotEqual(
                                                                Value::Literal(str),
                                                                Value::Variable(var2),
                                                            ),
                                                            Box::new(Self::from_tokens(
                                                                token_iter.collect(),
                                                            )),
                                                        )
                                                    }
                                                    _ => panic!("Expected variable or literal!"),
                                                }
                                            } else {
                                                panic!("Missing value!");
                                            }
                                        }
                                        _ => panic!("Expected comparison!"),
                                    }
                                } else {
                                    panic!("Missing comparison operator and value!");
                                }
                            }
                            _ => panic!("Expected variable or literal!"),
                        }
                    } else {
                        panic!("Missing values and comparison operator!")
                    }
                }
                Token::Variable(var) => match token_iter.next() {
                    Some(next_token) => match next_token {
                        Token::Assignment => {
                            if let Some(value) = token_iter.next() {
                                match value {
                                    Token::Variable(var2) => {
                                        return Self::VariableAssignment(var, Value::Variable(var2))
                                    }
                                    Token::String(str) => {
                                        return Self::VariableAssignment(var, Value::Literal(str))
                                    }
                                    _ => panic!("Expected variable or string literal!"),
                                }
                            } else {
                                panic!("Missing variable or string literal!");
                            }
                        }
                        _ => panic!("Expected assignment operator!"),
                    },
                    None => return Self::VariableValue(var),
                },
                Token::Assignment => {
                    panic!("Assignment operator can not be the first token of a line!")
                }
                Token::Condition(_) => {
                    panic!("A condition operatas mutable more than once at a timeor can not be the first token of a line!")
                }
            },
            None => panic!("No tokens provided!"),
        }
    }

    pub(crate) fn tokenize_line(string: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut in_string = false;
        let mut escape = false;
        let mut variable = false;
        let mut slash_seen = false;
        let mut skip_space = false;

        let mut char_iter = string.chars();

        while let Some(char) = char_iter.next() {
            if char == ' ' && skip_space {
                println!("Space skipped!");
                skip_space = false;
                continue;
            } else if skip_space {
                skip_space = false;
            }

            if char == '"' && !escape {
                if in_string {
                    current_token.push(char);
                    current_token = current_token.chars().filter(|c| *c != '"').collect();
                    tokens.push(Token::String(current_token.clone()));
                    current_token.clear();
                    in_string = false;
                    println!("String end");
                } else {
                    in_string = true;
                    println!("In string");
                }
            }

            if char == '$' && !escape && !in_string {
                current_token.clear();
                variable = true;
                continue;
            }

            if char == '/' && slash_seen && !escape && !in_string {
                tokens.push(Token::Comment);
                return tokens;
            }

            if char == '/' && !escape && !in_string {
                slash_seen = true;
            }

            if char == '!' && !escape && !in_string {
                if let Some(next_char) = char_iter.next() {
                    match next_char {
                        '=' => {
                            tokens.push(Token::Condition(false));
                            current_token.clear();
                            skip_space = true;
                            continue;
                        }
                        _ => {
                            #[cfg(test)]
                            println!("Char: {}\nNext char: {}", char, next_char);
                            panic!(
                                "Unknown expression encountered.\nDid you mean to do a comparison?"
                            );
                        }
                    }
                } else {
                    panic!("Unforseen end of expression.\nDotMan does not support boolean expressions!");
                }
            }

            if char == '=' && !escape && !in_string {
                if let Some(next_char) = char_iter.next() {
                    #[cfg(test)]
                    println!("Char: {}, Next char: {}", char, next_char);
                    match next_char {
                        '=' => {
                            tokens.push(Token::Condition(true));
                            current_token.clear();
                            skip_space = true;
                            continue;
                        }
                        ' ' => {
                            tokens.push(Token::Assignment);
                            current_token.clear();
                            continue;
                        }
                        _ => {
                            tokens.push(Token::Assignment);
                            current_token.clear();
                            current_token.push(next_char);
                            continue;
                        }
                    }
                } else {
                    panic!("Unforseen end of expression.\nDid you forget to enter a value for your assignment?");
                }
            }

            if escape {
                escape = false;
            }

            if char == '\\' {
                escape = true;
            }

            if current_token == "if" {
                tokens.push(Token::If);
                current_token.clear();
                continue;
            }

            if char == ' ' && variable {
                tokens.push(Token::Variable(current_token.clone()));
                current_token.clear();
                variable = false;
            }

            current_token.push(char);
            #[cfg(test)]
            println!("Current token: {:#?}", current_token);
        }

        tokens
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Condition {
    IsEqual(Value, Value),
    IsNotEqual(Value, Value),
}

#[derive(Debug, Clone, Default)]
pub struct TemplateEngine {
    pub(crate) variables: HashMap<String, String>,
    pub(crate) template_results: Vec<(Option<String>, usize)>,
}

impl TemplateEngine {
    fn convert_value(&self, value: Value) -> String {
        match value {
            Value::Literal(lit) => lit,
            Value::Variable(var) => match self.variables.get(&var) {
                Some(val) => val.to_string(),
                None => panic!("Could not find value of variable with identifier `{}`", var),
            },
        }
    }

    fn evaluate_expression(&mut self, expression: Expression) -> Option<String> {
        use Expression::*;

        match expression {
            VariableAssignment(identifier, value) => {
                let _ = self.variables.insert(identifier, self.convert_value(value));
                return None;
            }
            VariableValue(identifier) => match self.variables.get(&identifier) {
                Some(x) => return Some(x.clone()),
                None => {
                    panic!("Unknown identifier `{}`", identifier)
                }
            },
            StringLiteral(lit) => return Some(lit),
            IfStatement(condition, expression) => match condition {
                Condition::IsEqual(val1, val2) => {
                    if self.convert_value(val1) == self.convert_value(val2) {
                        return self.evaluate_expression(*expression);
                    } else {
                        return None;
                    }
                }
                Condition::IsNotEqual(val1, val2) => {
                    if self.convert_value(val1) != self.convert_value(val2) {
                        return self.evaluate_expression(*expression);
                    } else {
                        return None;
                    }
                }
            },
            Comment => None,
        }
    }

    pub fn evaluate_source_file(&mut self, content: String) {
        for (index, string) in content.lines().enumerate() {
            let expression = Expression::from_string(string.to_string());
            let template_result = (self.evaluate_expression(expression), index);
            self.template_results.push(template_result);
        }
    }

    pub fn evaluate_template_file(&mut self, content: String) {
        let mut template_lines = vec![];

        for (index, line) in content.lines().enumerate() {
            let line = line.trim();

            if line.starts_with("{{") && line.ends_with("}}") {
                let line = line
                    .chars()
                    .filter(|x| *x != '{' || *x != '}')
                    .collect::<String>();

                let line = line.clone();
                template_lines.push((line, index));
            }
        }

        template_lines = template_lines
            .into_iter()
            .map(|mut x| {
                x.0 = x.0.trim().to_string();
                x
            })
            .collect::<Vec<(String, usize)>>();

        #[cfg(test)]
        println!("Template lines:\n{:#?}\n", template_lines);

        for (template_line, line_number) in template_lines.into_iter() {
            let expression = Expression::from_string(template_line.to_string());
            let template_result = (self.evaluate_expression(expression.clone()), line_number);
            #[cfg(test)]
            println!(
                "Line:\n{:#?}\nExpression:\n{:#?}\nResult:\n{:#?}",
                template_line, expression, template_result
            );
            self.template_results.push(template_result);
        }
    }

    /// This function adds a newline at the end of the file
    /// and is reliant on the fact that the template results are ordered
    pub fn new_file_contents(&mut self, content: String) -> String {
        #[cfg(test)]
        println!("Results:\n{:#?}", self.template_results);

        let mut results_index = 0;

        let content_lines = content.lines().enumerate();
        let mut result_file = String::new();

        for (index, mut line) in content_lines {
            #[cfg(test)]
            println!("Original line:\n{}", line);

            if let Some(result) = self.template_results.get(results_index) {
                let result_line_number = result.1;

                println!("Result:\n{:#?}", result);

                if result_line_number == index {
                    line = match &self.template_results[results_index].0 {
                        Some(ref str) => {
                            results_index += 1;
                            str
                        }
                        None => {
                            results_index += 1;
                            ""
                        }
                    };
                }
            } else {
                #[cfg(test)]
                println!("No remaining template results!");
            }
            #[cfg(test)]
            println!("Result line:\n{}", line);

            result_file += format!("{}\n", line).as_str();
        }

        result_file
    }
}
