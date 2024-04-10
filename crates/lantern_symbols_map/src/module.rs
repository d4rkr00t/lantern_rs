use std::path::PathBuf;

#[derive(Debug)]
pub struct LNModule {
    pub file_path: PathBuf,
    pub symbols: Vec<usize>,
    pub is_entry: bool,
}
