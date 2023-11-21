pub mod ast;

use lrlex::lrlex_mod;
lrlex_mod!("chat.l");

use lrpar::lrpar_mod;
lrpar_mod!("chat.y");

use pyo3::exceptions::PyException;
use pyo3::prelude::*;

/// Return CHAT text from CHAT JSON (assumed to be valid according to
/// JSON Schema generated from Rust AST definition). First deserialize
/// the JSON to `Chat`. For Python interop, take an owned `String` as
/// input and turn a returned error into a `String`.
#[pyfunction]
fn json_to_chat(json: &str) -> PyResult<String> {
    let ast: ast::Chat =
        serde_json::from_str(&json).map_err(|e| PyException::new_err(e.to_string()))?;

    Ok(format!("{}", ast))
}

/// The part of our Python module that is implemented in Rust.
#[pymodule]
fn chat_py_demo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(json_to_chat, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn sample_json() {
        pyo3::prepare_freethreaded_python();

        let json = r#"{
  "tops": [
    {
        "Header": "@Begin"
    },
    {
        "MainTier": "*CHI:word"
    },
    {
        "DependentTier": "@com:comment"
    },
    {
        "Header": "@End"
    }
  ]
}
"#;

        assert_debug_snapshot!(json_to_chat(json));
    }
}
