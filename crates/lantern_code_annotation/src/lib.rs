use std::{cmp::min, collections::HashMap, path::PathBuf};

use colored::*;
use oxc_span::Span;

#[derive(Debug)]
pub struct CodeAnnotation {
    source: String,
    path: PathBuf,
    annotations: HashMap<usize, Vec<Annotation>>,
}

#[derive(Debug)]
struct Annotation {
    message: String,
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
        annotations.push(Annotation { message, span });
    }

    pub fn print(&self) -> String {
        let lines = self.source.lines().collect::<Vec<_>>();
        let mut result = Vec::new();
        let mut offset = 0;

        if self.annotations.len() == 0 {
            return String::new();
        }

        result.push(format!("{}:", self.path.to_str().unwrap()));

        for (line_num, line) in lines.iter().enumerate() {
            let annotations = self.annotations.get(&(line_num + 1));
            if annotations.is_none() {
                offset += line.len() + 1;
                continue;
            }
            for annotation in annotations.unwrap() {
                if line_num > 0 {
                    result.push(format!("{} │ {}", line_num, lines[line_num - 1]));
                }
                let span_start_pos = annotation.span.start as usize - offset;
                let span_end_pos = min(annotation.span.end as usize - offset, line.len());
                let line_num_str = format!("{}", line_num + 1);
                let highlighted_span = format!(
                    "{}{}{}",
                    &line[..span_start_pos],
                    &line[span_start_pos..span_end_pos].yellow(),
                    &line[span_end_pos..]
                );
                result.push(format!("{} │ {}", line_num_str, highlighted_span));
                result.push(format!(
                    "{:indent$}└── {}",
                    "",
                    annotation.message,
                    indent = span_start_pos + 3 + line_num_str.len()
                ));
            }
            offset += line.len() + 1;
        }

        return result.join("\n").to_string();
    }
}
