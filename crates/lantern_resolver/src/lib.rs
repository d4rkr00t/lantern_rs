use std::path::PathBuf;

use color_eyre::{eyre::Result, eyre::WrapErr};
use oxc_resolver::{ResolveOptions, Resolver};

#[derive(Debug)]
pub struct LanternResolver {
    resolver: Resolver,
}

impl LanternResolver {
    pub fn new() -> Self {
        let mut options = ResolveOptions::default();
        options.extensions = vec![".js".into(), ".json".into(), ".ts".into(), ".tsx".into()];

        Self {
            resolver: Resolver::new(options),
        }
    }

    pub fn resolve(&self, dir: &PathBuf, specifier: &str) -> Result<PathBuf> {
        let path = self.resolver.resolve(dir, specifier);
        match path {
            Ok(path) => {
                return Ok(PathBuf::from(path.path()));
            }
            Err(err) => {
                return Err(err)
                    .wrap_err_with(|| format!("Couldn't resolve {:?} from {:?}", specifier, dir))
            }
        }
    }
}
