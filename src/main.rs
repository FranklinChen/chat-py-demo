//! Generate JSON Schema for AST.

use chat_py_demo::ast::Chat;
use schemars::schema_for;
use std::io;

fn main() -> io::Result<()> {
    let schema = schema_for!(Chat);
    let stdout = io::stdout().lock();

    serde_json::to_writer_pretty(stdout, &schema)?;
    Ok(())
}
