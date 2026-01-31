use crate::ast::Document;

pub trait Extension: Send + Sync {
    fn name(&self) -> &'static str;

    // markdown text preprocess (masalan, custom syntax)
    fn preprocess(&self, _input: &str) -> Option<String> {
        None
    }

    // AST postprocess (masalan, heading’larni auto-id qilish)
    fn postprocess(&self, _doc: &mut Document) {}
}

#[derive(Default)]
pub struct ExtensionHost {
    exts: Vec<Box<dyn Extension>>,
}

impl ExtensionHost {
    pub fn new() -> Self {
        Self { exts: vec![] }
    }
    pub fn with(mut self, ext: impl Extension + 'static) -> Self {
        self.exts.push(Box::new(ext));
        self
    }

    pub fn run_preprocess(&self, mut s: String) -> String {
        for e in &self.exts {
            if let Some(out) = e.preprocess(&s) {
                s = out;
            }
        }
        s
    }

    pub fn run_postprocess(&self, doc: &mut Document) {
        for e in &self.exts {
            e.postprocess(doc);
        }
    }
}
