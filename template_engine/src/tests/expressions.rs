use crate::{Condition, Expression, Token, Value};

use anyhow::Result;

#[test]
fn literal_expression() -> Result<()> {
    let tokens = vec![Token::String("hi".to_string())];

    let expression = Expression::from_tokens(tokens)?;

    let expected_expression = Expression::StringLiteral("hi".to_string());

    assert_eq!(expression, expected_expression);

    Ok(())
}

#[test]
fn comment_expression() -> Result<()> {
    let tokens = vec![Token::Comment];

    let expression = Expression::from_tokens(tokens)?;

    let expected_expression = Expression::Comment;

    assert_eq!(expression, expected_expression);

    Ok(())
}

#[test]
fn variable_assignment() -> Result<()> {
    let tokens = vec![
        Token::Variable("test".to_string()),
        Token::Assignment,
        Token::String("testing".to_string()),
    ];

    let expression = Expression::from_tokens(tokens)?;

    let expected_expression =
        Expression::VariableAssignment("test".to_string(), Value::Literal("testing".to_string()));

    assert_eq!(expression, expected_expression);

    Ok(())
}

#[test]
fn if_expression() -> Result<()> {
    let tokens = vec![
        Token::If,
        Token::Variable("test".to_string()),
        Token::Condition(true),
        Token::String("testing".to_string()),
        Token::Comment,
    ];

    let expression = Expression::from_tokens(tokens)?;

    let expected_expression = Expression::IfStatement(
        Condition::IsEqual(
            Value::Variable("test".to_string()),
            Value::Literal("testing".to_string()),
        ),
        Box::new(Expression::Comment),
    );

    assert_eq!(expression, expected_expression);

    Ok(())
}
