use crate::ast::ListType;
use crate::ast::Node;
use crate::ast::TableAlignment;

fn render_alignment(align: TableAlignment) -> &'static str {
    match align {
        TableAlignment::Left => "style='text-align: left'",
        TableAlignment::Center => "style='text-align: center'",
        TableAlignment::Right => "style='text-align: right'",
        TableAlignment::None => "",
    }
}

pub fn render(nodes: &[Node]) -> String {
    let mut html = String::new();
    for node in nodes {
        match node {
            // Heading rendering
            Node::Heading {
                level,
                id,
                children,
            } => {
                html.push_str(&format!(
                    "<h{} id=\"{}\">{}</h{}>\n",
                    level,
                    id,
                    render(children),
                    level
                ));
            }

            // Paragraph rendering
            Node::Paragraph { children } => {
                html.push_str(&format!("<p>{}</p>\n", render(children)));
            }

            Node::Link { text, url } => {
                // Link: <a href="url">text</a>
                html.push_str(&format!("<a href='{}'>{}</a>", url, render(text)));
            }
            Node::Image { alt, url } => {
                // Image: <img src="url" alt="alt" />
                html.push_str(&format!("<img src='{}' alt='{}' />", url, alt));
            }

            // List rendering
            Node::List { kind, items } => {
                // 1. List turiga qarab teg tanlaymiz
                let tag = match kind {
                    ListType::Ordered => "ol",
                    ListType::Unordered => "ul",
                };

                html.push_str(&format!("<{}>\n", tag));

                for item in items {
                    html.push_str("  <li>");

                    // 2. Checkbox bo'lsa, uni kontentdan oldin chiqaramiz
                    if let Some(checked) = item.checked {
                        let check_attr = if checked { "checked" } else { "" };
                        html.push_str(&format!(
                            "<input type='checkbox' disabled {} style='margin-right: 5px;' />",
                            check_attr
                        ));
                    }

                    // 3. Asosiy matnni render qilamiz (Text, Bold, Link va h.k.)
                    html.push_str(&render(&item.content));

                    // 4. MUHIM: Agar ichki (nested) listlar bo'lsa,
                    // ularni <li> yopilishidan oldin rekursiv render qilamiz
                    if !item.children.is_empty() {
                        html.push('\n');
                        // Bu yerda yana render() chaqiriladi va u yangi <ol>/<ul> ochadi
                        let child_html = render(&item.children);

                        // Ichki list chiroyli ko'rinishi uchun har bir qatorni suramiz (tabulation)
                        for line in child_html.lines() {
                            html.push_str(&format!("    {}\n", line));
                        }
                    }

                    html.push_str("</li>\n");
                }

                html.push_str(&format!("</{}>\n", tag));
            }

            // Blockquote rendering
            Node::BlockQuote { children: content } => {
                html.push_str(&format!("<blockquote>\n{}</blockquote>\n", render(content)));
            }

            // Block math rendering
            Node::BlockMath { content: math } => {
                html.push_str(&format!("<div class='math-block'>\\[ {} \\]</div>\n", math));
            }

            // Code block rendering
            Node::CodeBlock {
                lang,
                filename,
                code,
            } => {
                html.push_str(&format!(
                    "<pre><code class=\"language-{}\">{:?}{}</code></pre>\n",
                    lang, filename, code
                ));
            }

            // Table rendering
            Node::Table { header, rows } => {
                html.push_str("<table>\n<thead>\n<tr>\n");
                for cell in header {
                    let align = render_alignment(cell.alignment);
                    html.push_str(&format!("<th{}>{}</th>", align, render(&cell.children)));
                }
                html.push_str("\n</tr>\n</thead>\n<tbody>\n");

                for row in rows {
                    html.push_str("<tr>\n");
                    for cell in row {
                        let align = render_alignment(cell.alignment);
                        html.push_str(&format!("<td{}>{}</td>", align, render(&cell.children)));
                    }
                    html.push_str("\n</tr>\n");
                }
                html.push_str("</tbody>\n</table>\n");
            }

            // Custom block rendering
            Node::CustomBlock {
                name,
                attributes,
                children,
            } => {
                let mut attr_str = String::new();

                for (key, val) in attributes {
                    // HTML xavfsizligi uchun attribute shakliga keltiramiz
                    attr_str.push_str(&format!(" data-{}='{}'", key, val));
                }

                html.push_str(&format!(
                    "<div class='{}' {}>{}</div>\n",
                    name,
                    attr_str,
                    render(children)
                ));
            }

            // Inline elements rendering
            // HR rendering
            Node::HorizontalRule => {
                html.push_str("<hr />\n");
            }

            // Line break rendering
            Node::LineBreak => {
                html.push_str("<br />\n");
            }

            // Text rendering
            Node::Text { content: t } => html.push_str(t),

            // Bold rendering
            Node::Bold { children } => {
                html.push_str(&format!("<strong>{}</strong>", render(children)));
            }

            // Italic rendering
            Node::Italic { children } => {
                html.push_str(&format!("<em>{}</em>", render(children)));
            }

            // Strikethrough rendering
            Node::Strikethrough { children } => {
                html.push_str(&format!("<del>{}</del>", render(children)));
            }

            // Underline rendering
            Node::Underline { children } => {
                html.push_str(&format!("<u>{}</u>", render(children)));
            }

            // Inline math rendering
            Node::InlineMath { content: math } => {
                html.push_str(&format!(
                    "<span class='math-inline'>\\( {} \\)</span>",
                    math
                ));
            }

            // Code span rendering
            Node::InlineCode { content: code } => {
                html.push_str(&format!("<code>{}</code>", code));
            }
        }
    }

    return html;
}
