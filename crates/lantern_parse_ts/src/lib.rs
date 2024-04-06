use std::path::Path;

use color_eyre::eyre::Result;

use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_parser::Parser;
use oxc_span::SourceType;

pub fn parse_ts<'a>(
    allocator: &'a Allocator,
    source_text: &'a str,
    path: &'a Path,
) -> Result<&'a mut Program<'a>> {
    let source_type = SourceType::from_path(path).unwrap();
    let ret = Parser::new(&allocator, &source_text, source_type).parse();

    if ret.errors.is_empty() {
        return Ok(allocator.alloc(ret.program));
    }

    return Err(color_eyre::eyre::eyre!(
        "Couldn't parse file {}",
        path.display()
    ));
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use lantern_testing::load_fixture;

    #[test]
    fn loads_simple_ts_file() {
        let path_buf = load_fixture!("simple.ts");
        let path = Path::new(&path_buf);
        let source_text = std::fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("{:?} not found", path.display()));
        let allocator = Allocator::default();
        parse_ts(&allocator, &source_text, &path).unwrap();
    }

    #[test]
    fn loads_simple_tsx_file() {
        let path_buf = load_fixture!("simple.tsx");
        let path = Path::new(&path_buf);
        let source_text = std::fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("{:?} not found", path.display()));
        let allocator = Allocator::default();
        parse_ts(&allocator, &source_text, &path).unwrap();
    }
}
