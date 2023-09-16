use std::path::PathBuf;

use color_eyre::eyre::Result;

use lantern_symbols_map;

pub fn analyze(project_path: PathBuf) -> Result<()> {
    let ln_map = lantern_symbols_map::build(&project_path)?;
    println!("symbols map: {:#?}", ln_map);
    return Ok(());
}
