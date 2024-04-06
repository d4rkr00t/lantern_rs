#[macro_export]
macro_rules! load_fixture {
    ($fname:expr) => {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .join($fname);
    };
}
