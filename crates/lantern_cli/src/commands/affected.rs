use std::path::PathBuf;

use color_eyre::eyre::Result;

use lantern_dependency_graph::LanternFileDependencyMap;

pub fn analyze(entry_points: &Vec<PathBuf>, changed: &Vec<PathBuf>) -> Result<()> {
    let ln_map = lantern_symbols_map::build_symbols_map(entry_points)?;
    let mut depgraph = LanternFileDependencyMap::new(ln_map);
    depgraph.build_dependency_graph();

    let mut all_affected = Vec::new();

    for changed_file_path in changed {
        let affected = depgraph.get_affected(&changed_file_path.canonicalize()?, true)?;
        all_affected.extend(affected);
    }

    println!("Affected: {:?}", all_affected);
    return Ok(());
}
