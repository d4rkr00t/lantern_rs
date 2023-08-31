#[cfg(test)]
mod tests {
    use lantern_core::analyze;
    use lantern_testing::load_fixture;
    use std::path::PathBuf;

    #[test]
    fn imports() {
        let path_buf = load_fixture!("imports.ts");
        analyze(&path_buf).unwrap();
    }
}
