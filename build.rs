use cfgrammar::yacc::YaccKind;
use lrlex::CTLexerBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    lalrpop::process_root().unwrap();

    CTLexerBuilder::new()
        .rust_edition(lrlex::RustEdition::Rust2021)
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                .rust_edition(lrpar::RustEdition::Rust2021)
                .visibility(lrpar::Visibility::Public)
                .grammar_in_src_dir("chat.y")
                .unwrap()
        })
        .visibility(lrlex::Visibility::Public)
        .lexer_in_src_dir("chat.l")
        .unwrap()
        .build()
        .unwrap();
    Ok(())
}
