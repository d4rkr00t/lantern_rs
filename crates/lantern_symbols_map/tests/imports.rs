#[cfg(test)]
mod tests {
    use lantern_symbols_map::build_symbols_map;
    use lantern_testing::load_fixture;

    #[test]
    fn imports() {
        let path_buf = load_fixture!("imports.ts");
        let ep = vec![path_buf];
        build_symbols_map(&ep).unwrap();
    }
}
