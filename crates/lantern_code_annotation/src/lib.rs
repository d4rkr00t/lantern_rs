use std::{collections::HashMap, path::PathBuf};

use swc_common::Span;

#[derive(Debug)]
pub struct CodeAnnotation {
    source: String,
    path: PathBuf,
    annotations: HashMap<usize, Vec<Annotation>>,
}

#[derive(Debug)]
struct Annotation {
    message: String,
    line: usize,
    span: Span,
}

impl CodeAnnotation {
    pub fn new(path: PathBuf, source: String) -> Self {
        Self {
            source,
            path,
            annotations: HashMap::new(),
        }
    }

    pub fn annotate(&mut self, message: String, line: usize, span: Span) {
        let annotations = self.annotations.entry(line).or_insert_with(Vec::new);
        annotations.push(Annotation {
            message,
            line,
            span,
        });
    }

    pub fn print(&self) -> String {
        let lines = self.source.lines().collect::<Vec<_>>();
        let mut result = Vec::new();

        if self.annotations.len() == 0 {
            return String::new();
        }

        result.push(format!("{}:", self.path.to_str().unwrap()));

        for (line_num, line) in lines.iter().enumerate() {
            let annotations = self.annotations.get(&(line_num + 1));
            if annotations.is_none() {
                continue;
            }
            for annotation in annotations.unwrap() {
                if line_num > 0 {
                    result.push(format!("{} │ {}", line_num, lines[line_num - 1]));
                }
                result.push(format!("{} │ {}", line_num + 1, line));
                result.push(format!("– {}", annotation.message));
            }
        }

        return result.join("\n").to_string();
    }
}
