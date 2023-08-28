use std::path::PathBuf;

use color_eyre::eyre::Result;

use swc_common::BytePos;
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::Parser;
use swc_ecma_parser::StringInput;
use swc_ecma_parser::Syntax;
use swc_ecma_parser::TsConfig;

pub fn parse_ts(path: &PathBuf) -> Result<swc_ecma_ast::Program> {
    let tsx = matches!(path.extension(), Some(ext) if ext == "tsx");
    println!("{:?}", path);
    let src = std::fs::read_to_string(path)?;
    let src = src.as_str();
    let failure_message = format!("Failed to parse {:?}", path);

    let program = Parser::new_from(Lexer::new(
        Syntax::Typescript(TsConfig {
            tsx,
            decorators: false,
            dts: false,
            no_early_errors: false,
            disallow_ambiguous_jsx_like: true,
        }),
        EsVersion::Es2022,
        StringInput::new(src, BytePos(0 as u32), BytePos(src.len() as u32)),
        None,
    ))
    .parse_program()
    .expect(&failure_message);

    Ok(program)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lantern_testing::load_fixture;

    #[test]
    fn loads_simple_ts_file() {
        let path_buf = load_fixture!("simple.ts");
        parse_ts(&path_buf).unwrap();
    }

    #[test]
    fn loads_simple_tsx_file() {
        let path_buf = load_fixture!("simple.tsx");
        parse_ts(&path_buf).unwrap();
    }
}
