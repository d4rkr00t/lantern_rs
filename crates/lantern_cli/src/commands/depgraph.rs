use std::path::PathBuf;

use color_eyre::eyre::Result;

use lantern_dependency_graph::LanternFileDependencyMap;
use lantern_formatters::graphviz::file_dependency_map_to_graphviz::file_dependency_map_to_graphviz;

pub fn build(entry_points: &Vec<PathBuf>) -> Result<()> {
    let ln_map = lantern_symbols_map::build_symbols_map(entry_points)?;
    let mut depgraph = LanternFileDependencyMap::new(ln_map);
    depgraph.build_dependency_graph();
    println!("{}", file_dependency_map_to_graphviz(&depgraph));
    return Ok(());
}
