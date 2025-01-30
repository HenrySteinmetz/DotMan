use std::collections::HashMap;

use anyhow::{anyhow, Result};

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
    fn from_string(string: String) -> Result<Self> {
        let tokens = Self::tokenize_line(string)?;

        Expression::from_tokens(tokens)
    }

    fn from_tokens(tokens: Vec<Token>) -> Result<Self> {
        let mut token_iter = tokens.into_iter();

        let next_token = match token_iter.next() {
            Some(t) => t,
            None => return Err(anyhow!("No tokens provided!")),
        };

        match next_token {
            Token::Comment => Ok(Self::Comment),
            Token::String(str) => Ok(Self::StringLiteral(str)),
            Token::Variable(var) => Self::parse_variable(&mut token_iter, var),
            Token::If => Self::parse_if_expression(&mut token_iter),
            _ => Err(anyhow!("Unexpected token")),
        }
    }

    fn parse_variable(
        token_iter: &mut impl Iterator<Item = Token>,
        variable_name: String,
    ) -> Result<Self> {
        let next_token = token_iter.next();

        match next_token {
            Some(assignment) if assignment == Token::Assignment => {
                let next_token = token_iter.next();

                match next_token {
                    Some(Token::String(str)) => Ok(Self::VariableAssignment(
                        variable_name,
                        Value::Literal(str.to_string()),
                    )),
                    Some(Token::Variable(var2)) => Ok(Self::VariableAssignment(
                        variable_name,
                        Value::Variable(var2.to_string()),
                    )),
                    Some(_) | None => Err(anyhow!(
                        "Expected string literal or variable name after assignment."
                    )),
                }
            }
            Some(_) => Err(anyhow!("Expected assignment operator.")),
            None => Ok(Self::VariableValue(variable_name)),
        }
    }

    fn parse_if_expression(token_iter: &mut impl Iterator<Item = Token>) -> Result<Self> {
        let val1 = match token_iter.next() {
            Some(Token::String(str)) => Value::Literal(str),
            Some(Token::Variable(var)) => Value::Variable(var),
            Some(_) => return Err(anyhow!("Expected variable or string literal.")),
            None => return Err(anyhow!("Missing tokens after `if`.")),
        };

        let cond = match token_iter.next() {
            Some(Token::Condition(cond)) => cond,
            Some(_) | None => return Err(anyhow!("Expected comparison operator.")),
        };

        let val2 = match token_iter.next() {
            Some(Token::String(str)) => Value::Literal(str),
            Some(Token::Variable(var)) => Value::Variable(var),
            Some(_) => return Err(anyhow!("Expected variable or string literal.")),
            None => return Err(anyhow!("Missing token after `if`.")),
        };

        if cond {
            Ok(Self::IfStatement(
                Condition::IsEqual(val1, val2),
                Box::new(Self::from_tokens(token_iter.collect())?),
            ))
        } else {
            Ok(Self::IfStatement(
                Condition::IsNotEqual(val1, val2),
                Box::new(Self::from_tokens(token_iter.collect())?),
            ))
        }
    }

    pub(crate) fn tokenize_line(string: String) -> Result<Vec<Token>> {
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
                #[cfg(test)]
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
                    #[cfg(test)]
                    println!("String end");
                } else {
                    in_string = true;
                    #[cfg(test)]
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
                return Ok(tokens);
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
                            return Err(anyhow!(
                                "Unknown expression encountered.\nDid you mean to do a comparison?"
                            ));
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
                    return Err(anyhow!("Unforseen end of expression.\nDid you forget to enter a value for your assignment?"));
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

        Ok(tokens)
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
    /// Takes in source and template files (true = source && false = template) and returns their parsed and evaluted content
    pub fn parse_files(input: Vec<(String, bool)>) -> Result<Vec<String>> {
        let mut results = Vec::new();

        let mut template_engine = Self::default();

        for (content, source) in input {
            if source {
                template_engine.evaluate_source_file(content.clone())?;
            } else {
                template_engine.evaluate_template_file(content.clone())?;
            }

            results.push(template_engine.new_file_contents(&content));
        }

        Ok(results)
    }

    fn convert_value(&self, value: Value) -> Result<String> {
        match value {
            Value::Literal(lit) => Ok(lit),
            Value::Variable(var) => match self.variables.get(&var) {
                Some(val) => Ok(val.to_string()),
                None => Err(anyhow!(
                    "Could not find value of variable with identifier `{}`",
                    var
                )),
            },
        }
    }

    fn evaluate_expression(&mut self, expression: Expression) -> Result<Option<String>> {
        use Expression::*;

        match expression {
            VariableAssignment(identifier, value) => {
                let _ = self
                    .variables
                    .insert(identifier, self.convert_value(value)?);
                return Ok(None);
            }
            VariableValue(identifier) => match self.variables.get(&identifier) {
                Some(x) => return Ok(Some(x.clone())),
                None => return Err(anyhow!("Unknown identifier `{}`", identifier)),
            },
            StringLiteral(lit) => return Ok(Some(lit)),
            IfStatement(condition, expression) => match condition {
                Condition::IsEqual(val1, val2) => {
                    if self.convert_value(val1)? == self.convert_value(val2)? {
                        return self.evaluate_expression(*expression);
                    } else {
                        return Ok(None);
                    }
                }
                Condition::IsNotEqual(val1, val2) => {
                    if self.convert_value(val1)? != self.convert_value(val2)? {
                        return self.evaluate_expression(*expression);
                    } else {
                        return Ok(None);
                    }
                }
            },
            Comment => Ok(None),
        }
    }

    pub fn evaluate_source_file(&mut self, content: String) -> Result<()> {
        for (index, string) in content.lines().enumerate() {
            let expression = Expression::from_string(string.to_string())?;
            let template_result = (self.evaluate_expression(expression)?, index);
            self.template_results.push(template_result);
        }

        Ok(())
    }

    pub fn evaluate_template_file(&mut self, content: String) -> Result<()> {
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
            let expression = Expression::from_string(template_line.to_string())?;
            let template_result = (self.evaluate_expression(expression.clone())?, line_number);
            #[cfg(test)]
            println!(
                "Line:\n{:#?}\nExpression:\n{:#?}\nResult:\n{:#?}",
                template_line, expression, template_result
            );
            self.template_results.push(template_result);
        }

        Ok(())
    }

    /// This function adds a newline at the end of the file
    /// and is reliant on the fact that the template results are ordered
    pub fn new_file_contents(&self, content: &String) -> String {
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

                result_file += format!("{}\n", line).as_str();
                return result_file;
            }
            #[cfg(test)]
            println!("Result line:\n{}", line);

            result_file += format!("{}\n", line).as_str();
        }

        result_file
    }
}
