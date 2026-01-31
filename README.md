# 🚀 mdkit: High-Performance All-in-One Markdown Parser

**mdkit** is a blazing-fast, universal Markdown parser written in **Rust** and compiled to **WebAssembly**. It’s designed to be the ultimate Swiss-army knife for Markdown in the JavaScript ecosystem—offering AST generation and HTML rendering in a single, lightweight package.

## ✨ Features

* **⚡ Rust Powered**: High-performance parsing that outperforms traditional JS parsers.
* **🌳 AST First**: Get a rich, type-safe Abstract Syntax Tree for custom rendering.
* **🧪 Math & More**: Native support for Inline/Block Math (LaTeX), Custom Blocks (:::), and Github Flavored Markdown (GFM).

---

## 📦 Installation

```bash
bun add @algolandlabs/mdkit

```

---

## 🛠 Usage

### 1. Basic HTML Rendering

The simplest way to convert Markdown to HTML.

```typescript
import { markdown_to_html } from '@algolandlabs/mdkit';

const md = "# Hello World";
const html = markdown_to_html(md);

console.log(html); 
// Output: <h1 id="hello-world">Hello World</h1>
```

### 2. Generating AST (Abstract Syntax Tree)

Perfect for building interactive editors or custom Vue/React renderers.

```typescript
import { markdown_to_ast } from '@algolandlabs/mdkit';

const ast = markdown_to_ast("# Hello");
console.log(ast);
/*
[
  {
    "type": "heading",
    "level": 1,
    "id": "hello",
    "children": [{ "type": "text", "content": "Hello" }]
  }
]
*/

```

---

## 🏗 Supported Syntax

* **Heading**: `#` to `######` (with auto-slug IDs)
* **Lists**: Ordered (`1. `) and Unordered (`- `, `* `) with nesting support.
* **Tables**: Full GFM table support with cell alignment.
* **Math**: `$...$` for inline and `$$...$$` for block (LaTeX compatible).
* **Custom Blocks**: Container syntax using `:::name attributes\ncontent\n:::`.
* **Checkboxes**: `- [ ]` and `- [x]` support.
* **Underline**: `__`
