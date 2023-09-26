#[cfg(test)]
mod tests {
    use lantern_symbols_map::build;
    use lantern_testing::load_fixture;
    use std::path::PathBuf;

    #[test]
    fn imports() {
        let path_buf = load_fixture!("imports.ts");
        let ep = vec![&path_buf];
        build(&ep).unwrap();
    }
}
