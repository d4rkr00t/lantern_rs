use std::path::PathBuf;

use color_eyre::eyre::Result;

use lantern_dependency_graph::LanternFileDependencyMap;

pub fn build(entry_points: &Vec<PathBuf>) -> Result<()> {
    let ln_map = lantern_symbols_map::build(entry_points)?;
    let mut depgraph = LanternFileDependencyMap::new(ln_map);
    depgraph.build_dependency_graph();
    println!("{}", depgraph.graphviz());
    return Ok(());
}
