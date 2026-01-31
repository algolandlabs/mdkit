use std::collections::HashMap;

use crate::ast::{ListItem, ListType, Node, TableAlignment, TableCell};

pub struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn parse_document(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        while !self.is_eof() {
            self.skip_empty_lines();
            if self.is_eof() {
                break;
            }

            // Heading parsing
            if self.starts_with("#") {
                nodes.push(self.parse_heading());
            }
            // HR parsing
            else if self.starts_with("---") {
                self.read_line();
                nodes.push(Node::HorizontalRule);
            }
            // Code block parsing
            else if self.starts_with("```") {
                nodes.push(self.parse_code_block());
            }
            // Blockquote parsing
            else if self.starts_with(">") {
                nodes.push(self.parse_blockquote());
            }
            // Math block parsing
            else if self.starts_with("$$") {
                nodes.push(self.parse_block_math());
            }
            // Custom block parsing
            else if self.starts_with(":::") {
                nodes.push(self.parse_custom_block());
            }
            // Table parsing
            else if self.is_table_start() {
                if let Some(table) = self.parse_table() {
                    nodes.push(table);
                }
            }
            // List parsing
            else if self.is_list_start() {
                nodes.push(self.parse_list(0));
                continue;
            }
            // inline elements
            else {
                let inline = self.parse_inline_elements('\n');
                self.consume_if('\n');
                nodes.push(Node::Paragraph(inline));
            }
        }
        nodes
    }

    /// Inline elements parser
    fn parse_inline_elements(&mut self, delimiter: char) -> Vec<Node> {
        let mut nodes = Vec::new();
        let mut text_acc = String::new();

        while !self.is_eof() {
            let ch = self.peek();

            if ch == delimiter && delimiter != '\0' {
                break;
            }
            if ch == '\n' && delimiter != '\n' {
                break;
            }

            if ch == '\\' {
                self.flush_text(&mut text_acc, &mut nodes);
                self.consume(1);
                nodes.push(Node::LineBreak);
            }
            // Inline math
            else if ch == '$' && !self.starts_with("$$") {
                self.flush_text(&mut text_acc, &mut nodes);
                nodes.push(self.parse_inline_math());
            }
            // Bold and Italic ***
            else if self.starts_with("***") {
                self.flush_text(&mut text_acc, &mut nodes);
                self.consume(3);
                let inner = self.parse_inline_elements('*');
                self.consume_repeated('*', 3);
                nodes.push(Node::Bold(vec![Node::Italic(inner)]));
            }
            // Bold, Underline, Strike
            else if self.starts_with("**") {
                self.flush_text(&mut text_acc, &mut nodes);
                self.consume(2);
                let inner = self.parse_inline_elements('*');
                self.consume_repeated('*', 2);
                nodes.push(Node::Bold(inner));
            } else if self.starts_with("__") {
                self.flush_text(&mut text_acc, &mut nodes);
                self.consume(2);
                let inner = self.parse_inline_elements('_');
                self.consume_repeated('_', 2);
                nodes.push(Node::Underline(inner));
            } else if self.starts_with("~~") {
                self.flush_text(&mut text_acc, &mut nodes);
                self.consume(2);
                let inner = self.parse_inline_elements('~');
                self.consume_repeated('~', 2);
                nodes.push(Node::Strikethrough(inner));
            }
            // Single italic *
            else if ch == '*' {
                self.flush_text(&mut text_acc, &mut nodes);
                self.consume(1);
                let inner = self.parse_inline_elements('*');
                self.consume_if('*');
                nodes.push(Node::Italic(inner));
            } else if ch == '`' {
                self.flush_text(&mut text_acc, &mut nodes);
                self.consume(1);

                let mut code_content = String::new();
                while !self.is_eof() && self.peek() != '`' {
                    code_content.push(self.next_char());
                }

                self.consume_if('`');
                nodes.push(Node::InlineCode(code_content));
            } else if self.starts_with("![") {
                // IMAGE: ![alt](url)
                self.flush_text(&mut text_acc, &mut nodes);
                self.consume(2); // "![" ni o'tkazib yuboramiz

                let alt = self.read_until(']');

                if self.peek() == '(' {
                    self.consume(1); // "(" ni o'tkazib yuboramiz
                    let url = self.read_until(')');
                    nodes.push(Node::Image { alt, url });
                } else {
                    // Agar ( bo'lmasa, bu rasm emas, oddiy matn deb hisoblaymiz
                    text_acc.push_str(&format!("![{}", alt));
                }
            } else if self.peek() == '[' {
                // LINK: [text](url)
                self.flush_text(&mut text_acc, &mut nodes);
                self.consume(1); // "[" ni o'tkazib yuboramiz

                let link_text_raw = self.read_until(']');

                if self.peek() == '(' {
                    self.consume(1); // "(" ni o'tkazib yuboramiz
                    let url = self.read_until(')');

                    // Link ichidagi matnni ham parse qilamiz (masalan, [**bold** link](url))
                    let mut sub_parser = Parser::new(&link_text_raw);
                    nodes.push(Node::Link {
                        text: sub_parser.parse_inline_elements('\0'),
                        url,
                    });
                } else {
                    // Agar ( bo'lmasa, bu shunchaki qavs ichidagi matn
                    text_acc.push_str(&format!("[{}", link_text_raw));
                }
            }
            // Normal text
            else {
                text_acc.push(self.next_char());
            }
        }
        self.flush_text(&mut text_acc, &mut nodes);
        nodes
    }
}

impl Parser {
    /// Heading parser
    /// # Heading 1
    fn parse_heading(&mut self) -> Node {
        /* Avvalgi javobdagi bilan bir xil */
        let mut level = 0;
        while self.peek() == '#' {
            level += 1;
            self.next_char();
        }
        self.skip_whitespace_inline();
        let content = self.parse_inline_elements('\n');
        self.consume_if('\n');

        let raw_text = self.extract_plain_text(&content);
        let id = self.slugify(&raw_text);

        Node::Heading {
            level: level,
            id: id,
            children: content,
        }
    }

    /// List parser
    /// 1. Item 1
    /// 2. Item 2
    fn parse_list(&mut self, base_indent: usize) -> Node {
        let initial_kind = self.identify_list_type(); // Birinchi qator turini aniqlaymiz
        let mut items: Vec<ListItem> = Vec::new();

        while !self.is_eof() {
            let raw_line = self.peek_line();
            if raw_line.trim().is_empty() {
                break;
            }

            let (indent, trimmed_line) = self.get_line_indentation(&raw_line);

            // MUHIM: Agar joriy qator turi boshlang'ich turdan farq qilsa, listni yopamiz
            if indent == base_indent {
                let current_kind = self.identify_list_type();
                if current_kind != initial_kind {
                    break; // Ordered list tugadi, endi Unordered boshlanishi kerak
                }
            }

            if indent < base_indent || !self.is_list_line(&trimmed_line) {
                break;
            }

            if indent > base_indent {
                if let Some(last_item) = items.last_mut() {
                    last_item.children.push(self.parse_list(indent));
                    continue;
                }
            }

            self.read_line(); // Marker bor qatorni yeymiz

            // Markerlarni (1. yoki -) tozalash
            let clean_content = self.clean_marker(&trimmed_line, initial_kind.clone());
            let (checked, final_text) = self.extract_checkbox(&clean_content);

            let mut p = Parser::new(&final_text);
            items.push(ListItem {
                content: p.parse_inline_elements('\0'),
                checked,
                children: Vec::new(),
            });
        }

        Node::List {
            kind: initial_kind,
            items,
        }
    }

    /// Blockquote parser
    /// > This is blockquote
    fn parse_blockquote(&mut self) -> Node {
        let mut inner_content = String::new();
        while self.starts_with(">") {
            self.consume(1);
            self.skip_whitespace_inline();
            inner_content.push_str(&self.read_line());
            inner_content.push('\n');
            self.skip_whitespace_inline();
        }
        // Ichki Markdownni qayta parse qilish (Nested support)
        let mut sub_parser = Parser::new(&inner_content);
        Node::BlockQuote(sub_parser.parse_document())
    }

    /// Code block parser
    /// ```rust
    /// fn main() {
    ///     println!("Hello from Rust");
    /// }
    /// ```
    fn parse_code_block(&mut self) -> Node {
        self.consume(3);

        let header = self.read_line();
        let parts: Vec<&str> = header.split_whitespace().collect();

        let lang = parts.get(0).unwrap_or(&"").to_string();
        let filename = parts.get(1).map(|s| s.to_string());
        let mut code = String::new();

        while !self.is_eof() && !self.starts_with("```") {
            code.push(self.next_char());
        }
        if self.starts_with("```") {
            self.consume(3);
        }
        Node::CodeBlock {
            lang,
            filename,
            code: code.trim().to_string(),
        }
    }

    /// Table parer
    /// | A | B |
    /// |---|---|
    /// | 1 | 2 |
    fn parse_table(&mut self) -> Option<Node> {
        let header_line = self.read_line();
        let sep_line = self.read_line();

        let alignments = self.parse_alignments(&sep_line);
        let header = self.split_table_row(&header_line, &alignments);

        let mut rows = Vec::new();
        while !self.is_eof() && self.is_table_start() {
            let line = self.read_line();
            rows.push(self.split_table_row(&line, &alignments));
        }

        Some(Node::Table { header, rows })
    }

    /// Math block parser
    /// $$
    /// c^2 = a^2 + b^
    /// $$
    fn parse_block_math(&mut self) -> Node {
        self.consume(2);
        let mut content = String::new();
        while !self.is_eof() && !self.starts_with("$$") {
            content.push(self.next_char());
        }
        if self.starts_with("$$") {
            self.consume(2);
        }
        Node::BlockMath(content.trim().to_string())
    }

    /// Inline math parser
    /// $$ y = 10x + 5
    fn parse_inline_math(&mut self) -> Node {
        self.consume(1);
        let mut content = String::new();
        while !self.is_eof() && self.peek() != '$' {
            content.push(self.next_char());
        }
        self.consume_if('$');
        Node::InlineMath(content)
    }

    /// Custom block parser
    /// :::tab title=Tab 1
    /// content
    /// :::
    fn parse_custom_block(&mut self) -> Node {
        self.consume(3); // ::: yeymiz

        let header = self.read_line();
        let mut parts = header.split_whitespace();

        // Birinchi so'z - label (masalan: tab)
        let name = parts.next().unwrap_or("").to_string();

        // Qolganlari - attributlar (title="My Title" disabled=false)
        let mut attributes = HashMap::new();
        for part in parts {
            if let Some((key, value)) = part.split_once('=') {
                // Qo'shtirnoqlarni olib tashlaymiz: "title" -> title
                let clean_value = value.trim_matches('"').to_string();
                attributes.insert(key.to_string(), clean_value);
            }
        }

        let mut inner_content = String::new();
        let mut nest_level = 1;

        // To'g'ri yopuvchi ::: ni topish uchun loop
        while !self.is_eof() && nest_level > 0 {
            if self.starts_with(":::") {
                // Ichkarida yana blok ochilsa (masalan :::tab), uni kontentga qo'shamiz
                // Lekin bizning asosiy blokimiz qachon tugashini bilishimiz kerak
                // Buning uchun kelayotgan satrni tekshiramiz
                // let peek_pos = self.pos + 3;
                // Agar ::: dan keyin matn bo'lsa, bu yangi ichki blok (nest_level++)
                // Agar ::: dan keyin darhol yangi qator bo'lsa, bu yopuvchi blok (nest_level--)

                let remaining = &self.input[self.pos..];
                let line = remaining
                    .iter()
                    .take_while(|&&c| c != '\n')
                    .collect::<String>();

                if line.trim() == ":::" {
                    nest_level -= 1;
                    if nest_level > 0 {
                        inner_content.push_str(":::\n");
                        self.consume(3);
                        self.consume_if('\n');
                    } else {
                        self.consume(3); // Asosiy blok yopildi
                        self.consume_if('\n');
                    }
                } else {
                    // Bu ichki blok ochilishi: :::tab
                    nest_level += 1;
                    inner_content.push_str(&line);
                    inner_content.push('\n');
                    self.consume(line.len());
                    self.consume_if('\n');
                }
            } else {
                inner_content.push(self.next_char());
            }
        }

        // Ichki kontentni yangi parser bilan rekursiv parse qilamiz
        let mut sub_parser = Parser::new(&inner_content);
        Node::CustomBlock {
            name,
            attributes,
            children: sub_parser.parse_document(),
        }
    }
}

impl Parser {
    fn peek(&self) -> char {
        *self.input.get(self.pos).unwrap_or(&'\0')
    }

    fn next_char(&mut self) -> char {
        let c = self.peek();
        self.pos += 1;
        c
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..]
            .iter()
            .collect::<String>()
            .starts_with(s)
    }

    fn consume(&mut self, n: usize) {
        self.pos += n;
    }

    fn consume_if(&mut self, c: char) {
        if self.peek() == c {
            self.pos += 1;
        }
    }

    fn consume_repeated(&mut self, c: char, n: usize) {
        for _ in 0..n {
            self.consume_if(c);
        }
    }

    fn skip_empty_lines(&mut self) {
        while !self.is_eof() && self.peek().is_whitespace() {
            self.pos += 1;
        }
    }

    fn peek_line(&self) -> String {
        self.input[self.pos..]
            .iter()
            .take_while(|&&c| c != '\n')
            .collect()
    }

    fn read_line(&mut self) -> String {
        let s = self.peek_line();
        self.consume(s.len());
        self.consume_if('\n');
        s
    }

    fn slugify(&self, text: &str) -> String {
        text.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }

    fn extract_plain_text(&self, nodes: &[Node]) -> String {
        let mut text = String::new();
        for node in nodes {
            match node {
                Node::Text(t) => text.push_str(t),
                Node::Bold(c) | Node::Italic(c) | Node::Underline(c) | Node::Strikethrough(c) => {
                    text.push_str(&self.extract_plain_text(c));
                }
                Node::InlineCode(c) | Node::InlineMath(c) => text.push_str(c),
                _ => {}
            }
        }
        text
    }

    fn skip_whitespace_inline(&mut self) {
        while self.peek() == ' ' || self.peek() == '\t' {
            self.pos += 1;
        }
    }

    fn flush_text(&self, text: &mut String, nodes: &mut Vec<Node>) {
        if !text.is_empty() {
            nodes.push(Node::Text(text.drain(..).collect()));
        }
    }

    fn split_table_row(&self, line: &str, aligns: &[TableAlignment]) -> Vec<TableCell> {
        line.trim()
            .trim_matches('|')
            .split('|')
            .enumerate()
            .map(|(i, s)| {
                let mut p = Parser::new(s.trim());
                TableCell {
                    children: p.parse_inline_elements('\0'),
                    alignment: *aligns.get(i).unwrap_or(&TableAlignment::None),
                }
            })
            .collect()
    }

    fn parse_alignments(&self, sep: &str) -> Vec<TableAlignment> {
        sep.trim()
            .trim_matches('|')
            .split('|')
            .map(|s| {
                let s = s.trim();
                match (s.starts_with(':'), s.ends_with(':')) {
                    (true, true) => TableAlignment::Center,
                    (true, false) => TableAlignment::Left,
                    (false, true) => TableAlignment::Right,
                    _ => TableAlignment::None,
                }
            })
            .collect()
    }

    fn is_list_start(&self) -> bool {
        let line = self.peek_line().trim_start().to_string();
        line.starts_with("- ")
            || line.starts_with("* ")
            || (line.len() > 2
                && line.chars().next().unwrap().is_ascii_digit()
                && line.contains(". "))
    }

    fn is_table_start(&self) -> bool {
        let line = self.peek_line();
        line.contains('|') && line.trim().starts_with('|')
    }

    // Qator ro'yxat ekanini tekshirish (- , * , 1. )
    fn is_list_line(&self, line: &str) -> bool {
        let trimmed = line.trim_start();
        trimmed.starts_with("- ")
            || trimmed.starts_with("* ")
            || (trimmed.len() > 2
                && trimmed.chars().next().unwrap().is_ascii_digit()
                && trimmed.contains(". "))
    }

    // Ro'yxat turini aniqlash
    fn identify_list_type(&self) -> crate::ast::ListType {
        let line = self.peek_line().trim_start().to_string();
        if line.starts_with("- ") || line.starts_with("* ") {
            crate::ast::ListType::Unordered
        } else {
            crate::ast::ListType::Ordered
        }
    }

    // Qator boshidagi bo'shliqlarni sanash va toza qatorni qaytarish
    fn get_line_indentation(&self, line: &str) -> (usize, String) {
        let indent = line.chars().take_while(|c| c.is_whitespace()).count();
        (indent, line.trim().to_string())
    }

    // Checkboxni ajratib olish: [x] Task -> (Some(true), "Task")
    fn extract_checkbox(&self, line: &str) -> (Option<bool>, String) {
        if line.starts_with("[ ] ") {
            (Some(false), line[4..].to_string())
        } else if line.starts_with("[x] ") || line.starts_with("[X] ") {
            (Some(true), line[4..].to_string())
        } else {
            (None, line.to_string())
        }
    }

    // Markerlarni tozalash: "1. Item" -> "Item"
    fn clean_marker(&self, line: &str, kind: ListType) -> String {
        match kind {
            ListType::Unordered => {
                // "- " yoki "* " ni kesib tashlaymiz (2 ta belgi)
                if line.starts_with("- ") || line.starts_with("* ") {
                    line[2..].to_string()
                } else {
                    line.to_string()
                }
            }
            ListType::Ordered => {
                // "1. " kabi raqamli markerlarni kesamiz
                if let Some(dot_pos) = line.find(". ") {
                    line[dot_pos + 2..].to_string()
                } else {
                    line.to_string()
                }
            }
        }
    }

    fn read_until(&mut self, stop_char: char) -> String {
        let mut result = String::new();
        while !self.is_eof() && self.peek() != stop_char {
            result.push(self.next_char());
        }
        self.consume_if(stop_char); // Yopuvchi qavsni ( ] yoki ) ) yeymiz
        result
    }
}
