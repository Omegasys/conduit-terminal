use conduit::config_engine::parser::ConfigParser;

#[test]
fn parser_can_parse_simple_toml() {
    let parser = ConfigParser::new();

    let input = r#"
        scrollback = 10000
        shell = "bash"
    "#;

    let config = parser.parse(input).expect("configuration should parse");

    assert_eq!(config.get("scrollback"), Some("10000"));
    assert_eq!(config.get("shell"), Some("bash"));
}

#[test]
fn parser_rejects_invalid_toml() {
    let parser = ConfigParser::new();

    let input = r#"
        scrollback =
        shell = "bash"
    "#;

    assert!(parser.parse(input).is_err());
}

#[test]
fn parser_handles_nested_values() {
    let parser = ConfigParser::new();

    let input = r#"
        [terminal]
        scrollback = 5000
        cursor_style = "block"
    "#;

    let config = parser.parse(input).expect("configuration should parse");

    assert!(config.contains("terminal"));
}
