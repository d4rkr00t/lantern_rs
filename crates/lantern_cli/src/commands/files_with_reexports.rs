use std::{collections::HashMap, path::PathBuf};

use color_eyre::eyre::Result;

use lantern_code_annotation::CodeAnnotation;
use lantern_files_with_reexports::find_files_with_reexports;

pub fn analyze(entry_points: &Vec<PathBuf>) -> Result<()> {
    let mut ln_map = lantern_symbols_map::build_symbols_map(&entry_points)?;
    let mut annotations: HashMap<usize, CodeAnnotation> = HashMap::new();
    let re_exports = find_files_with_reexports(&ln_map)?;
    let total = re_exports.len();

    for symbol in re_exports {
        let span = symbol.get_span();
        let span_line = ln_map.get_line_number_from_span(symbol.module_id, span);
        if !annotations.contains_key(&symbol.module_id) {
            annotations.insert(
                symbol.module_id,
                CodeAnnotation::new(
                    ln_map.get_module_path(symbol.module_id).clone(),
                    ln_map.get_module_source(symbol.module_id).to_string(),
                ),
            );
        }
        let annotation = annotations.get_mut(&symbol.module_id).unwrap();
        let annotation_message = if let Some(symbol_name) = symbol.get_name() {
            format!("re-export: {}", symbol_name)
        } else {
            "re-export".to_owned()
        };

        annotation.annotate(annotation_message, span_line, span.clone());
    }

    for (_, value) in &annotations {
        println!("{}", value.print());
    }

    println!("Total re-exports found: {}", total);

    return Ok(());
}
