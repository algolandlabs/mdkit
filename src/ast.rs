use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ListType {
    Ordered,
    Unordered,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    pub content: Vec<Node>,
    pub children: Vec<Node>,
    pub checked: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableCell {
    pub children: Vec<Node>,
    pub alignment: TableAlignment,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TableAlignment {
    Left,
    Center,
    Right,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Heading {
        level: usize,
        id: String,
        children: Vec<Node>,
    },
    HorizontalRule,

    Paragraph(Vec<Node>),
    LineBreak,

    Link {
        text: Vec<Node>,
        url: String,
    },
    Image {
        alt: String,
        url: String,
    },

    Bold(Vec<Node>),
    Italic(Vec<Node>),
    Strikethrough(Vec<Node>),
    Underline(Vec<Node>),
    Text(String),

    InlineMath(String),
    BlockMath(String),

    InlineCode(String),
    CodeBlock {
        lang: String,
        filename: Option<String>,
        code: String,
    },

    BlockQuote(Vec<Node>),
    List {
        kind: ListType,
        items: Vec<ListItem>,
    },

    Table {
        header: Vec<TableCell>,
        rows: Vec<Vec<TableCell>>,
    },

    CustomBlock {
        name: String,
        attributes: HashMap<String, String>,
        children: Vec<Node>,
    },
}
