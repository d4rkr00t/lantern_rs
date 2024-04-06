use std::path::PathBuf;

use oxc_resolver::{ResolveOptions, Resolver};

pub struct LanternResolver {
    resolver: Resolver,
}

impl LanternResolver {
    pub fn new() -> Self {
        let mut options = ResolveOptions::default();
        options.extensions = vec![".js".into(), ".json".into(), ".ts".into()];

        Self {
            resolver: Resolver::new(options),
        }
    }

    pub fn resolve(&self, dir: &PathBuf, specifier: &str) -> Result<PathBuf, ()> {
        let path = self.resolver.resolve(dir, specifier);
        if let Ok(path) = path {
            return Ok(PathBuf::from(path.path()));
        }
        return Err(());
    }
}
