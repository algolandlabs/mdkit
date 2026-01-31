use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ListType {
    Ordered,
    Unordered,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListItem {
    pub content: Vec<Node>,
    pub children: Vec<Node>,
    pub checked: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    pub children: Vec<Node>,
    pub alignment: TableAlignment,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum TableAlignment {
    Left,
    Center,
    Right,
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Node {
    Heading {
        level: usize,
        id: String,
        children: Vec<Node>,
    },
    HorizontalRule,

    Paragraph {
        children: Vec<Node>,
    },
    LineBreak,

    Link {
        text: Vec<Node>,
        url: String,
    },
    Image {
        alt: String,
        url: String,
    },

    Bold {
        children: Vec<Node>,
    },
    Italic {
        children: Vec<Node>,
    },
    Strikethrough {
        children: Vec<Node>,
    },
    Underline {
        children: Vec<Node>,
    },
    Text {
        content: String,
    },

    InlineMath {
        content: String,
    },
    BlockMath {
        content: String,
    },

    InlineCode {
        content: String,
    },
    CodeBlock {
        lang: String,
        filename: Option<String>,
        code: String,
    },

    BlockQuote {
        children: Vec<Node>,
    },
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
