use anyhow::Result;

use crate::{Expression, Token};

#[test]
fn tokinize_string_with_space() -> Result<()> {
    let line = r#""this is a test string""#.to_string();

    let tokens = Expression::tokenize_line(line)?;

    let expected_tokens = vec![Token::String("this is a test string".to_string())];

    assert_eq!(tokens, expected_tokens);

    Ok(())
}

#[test]
fn tokinize_assignment() -> Result<()> {
    let line = r#"$test = "testing""#.to_string();

    let tokens = Expression::tokenize_line(line)?;

    let expected_tokens = vec![
        Token::Variable("test".to_string()),
        Token::Assignment,
        Token::String("testing".to_string()),
    ];

    assert_eq!(tokens, expected_tokens);

    Ok(())
}

#[test]
fn tokinize_if_expression() -> Result<()> {
    let line = r#"if "test" == "testing" $test = "hi""#.to_string();

    let tokens = Expression::tokenize_line(line)?;

    let expected_tokens = vec![
        Token::If,
        Token::String("test".to_string()),
        Token::Condition(true),
        Token::String("testing".to_string()),
        Token::Variable("test".to_string()),
        Token::Assignment,
        Token::String("hi".to_string()),
    ];

    assert_eq!(tokens, expected_tokens);

    Ok(())
}

#[test]
fn tokinize_if_not_expression() -> Result<()> {
    let line = r#"if "test" != "testing" $test = "hi""#.to_string();

    let tokens = Expression::tokenize_line(line)?;

    let expected_tokens = vec![
        Token::If,
        Token::String("test".to_string()),
        Token::Condition(false),
        Token::String("testing".to_string()),
        Token::Variable("test".to_string()),
        Token::Assignment,
        Token::String("hi".to_string()),
    ];

    assert_eq!(tokens, expected_tokens);

    Ok(())
}

#[test]
fn tokinize_comment() -> Result<()> {
    let line = "// This is a test comment".to_string();

    let tokens = Expression::tokenize_line(line)?;

    let expected_tokens = vec![Token::Comment];

    assert_eq!(tokens, expected_tokens);

    Ok(())
}
