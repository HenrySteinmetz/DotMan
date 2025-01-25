use crate::TemplateEngine;

#[test]
fn replace_single_line() {
    let test_file = format!("Line1\nLine2\nLine3");

    let mut engine = TemplateEngine::default();
    engine.template_results = vec![(Some("Replaced".to_string()), 1)];

    let output = engine.new_file_contents(test_file.clone());

    let expected_output = format!("Line1\nReplaced\nLine3\n");

    assert_eq!(output, expected_output);
}

#[test]
fn replace_multiple_lines() {
    let test_file = format!("Line1\nLine2\nLine3\nLine4\nLine5\n");

    let mut engine = TemplateEngine::default();
    engine.template_results = vec![
        (Some("Replaced line 3".to_string()), 2),
        (Some("Replaced line 5".to_string()), 4),
    ];

    let output = engine.new_file_contents(test_file.clone());

    let expected_output = format!("Line1\nLine2\nReplaced line 3\nLine4\nReplaced line 5\n");

    assert_eq!(output, expected_output);
}
