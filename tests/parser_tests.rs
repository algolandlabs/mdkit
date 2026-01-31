#[cfg(test)]
mod parser_tests {
    use mdkit::parser::Parser;
    use mdkit::renderer::render;

    #[test]
    fn test_bold_italic_mixed() {
        let input = "***bold and italic*** and ~~strike~~";
        let mut parser = Parser::new(input);
        let nodes = parser.parse_document();
        let html = render(&nodes);

        assert!(html.contains("<strong><em>bold and italic</em></strong>"));
        assert!(html.contains("<del>strike</del>"));
    }

    #[test]
    fn test_heading_with_id() {
        let input = "## Project Setup";
        let mut parser = Parser::new(input);
        let nodes = parser.parse_document();
        let html = render(&nodes);

        println!("{:?}", html);
        assert!(html.contains("<h2 id=\"project-setup\">Project Setup</h2>"));
    }

    #[test]
    fn test_code_block_with_filename() {
        let input = "```rust main.rs\nfn main() {}\n```";
        let mut parser = Parser::new(input);
        let nodes = parser.parse_document();
        let html = render(&nodes);

        assert!(html.contains("main.rs"));
        assert!(html.contains("language-rust"));
    }

    #[test]
    fn test_table_parsing() {
        let input = "| Header 1 | Header 2 |\n|---|---|\n| Cell 1 | Cell 2 |";
        let mut parser = Parser::new(input);
        let nodes = parser.parse_document();
        let html = render(&nodes);

        assert!(html.contains("<table>"));
        assert!(html.contains("<th>Header 1</th>"));
        assert!(html.contains("<td>Cell 1</td>"));
    }

    #[test]
    fn test_custom_tabs_block() {
        let input = ":::tabs\n:::tab title=\"Rust\"\nCode\n:::\n:::";
        let mut parser = Parser::new(input);
        let nodes = parser.parse_document();
        let html = render(&nodes);

        assert!(html.contains("data-title='Rust'"));
    }

    #[test]
    fn test_lists_with_checkbox() {
        let input = "- [x] Task Done\n- [ ] Task Pending";
        let mut parser = Parser::new(input);
        let nodes = parser.parse_document();
        let html = render(&nodes);

        assert!(html.contains("checked"));
        assert!(html.contains("<input type='checkbox'"));
    }
}
