use crate::TemplateEngine;

#[test]
fn add_variables() {
    let mut engine = TemplateEngine::default();

    let source_file = format!(r#"$test = "testing""#);

    engine.evaluate_source_file(source_file);

    assert_eq!(engine.variables.get("test").unwrap(), "testing");
}

#[test]
fn insert_variable() {
    let mut engine = TemplateEngine::default();

    let template_file = "{{ $test = \"Test value\" }}\nTrash line\n{{ $test }}".to_string();

    engine.evaluate_template_file(template_file.clone());

    let result = engine.new_file_contents(template_file.clone());

    let expected_result = "\nTrash line\nTest value\n".to_string();

    assert_eq!(result, expected_result);
}

#[test]
fn conditional_assignment() {
    let mut engine = TemplateEngine::default();

    let source_file = "if \"test\" == \"test\" $test = \"success\" ".to_string();

    engine.evaluate_source_file(source_file);

    assert_eq!(engine.variables.get("test").unwrap(), "success");
}
