use json_patch::{patch, Patch};
use serde_json::{from_value, json, Value};

struct Predicate {
    predicate: String,
    left: String,
    right: String,
}

/// Transforms an S-expression, represented as an &str, into a Predicate struct.
fn destruct_s_exp(s_exp: String) -> Predicate {
    println!("s_exp: {}", s_exp);
    let s = s_exp.trim_matches(|p| p == '(' || p == ')');
    let mut parts = s.split_whitespace();
    let predicate = parts.next().unwrap_or_default();
    let left = parts.next().unwrap_or_default();
    let right = parts.next().unwrap_or_default();

    return Predicate {
        predicate: predicate.to_string(),
        left: left.to_string(),
        right: right.to_string(),
    };
}

/// Reconstructs the JSON AST of a DuckDB query from a given AST and an S-expression
/// representing the best spatial join predicate found by the term rewriting system.
pub fn reconstruct(contents: &str, s_exp: String) -> String {
    let ast: Result<Value, serde_json::Error> = serde_json::from_str(&contents);

    let Predicate {
        predicate,
        left,
        right,
    } = destruct_s_exp(s_exp);

    let left_column_names: Vec<&str> = left.split(".").collect();
    let right_column_names: Vec<&str> = right.split(".").collect();

    let ast_patch: Result<Patch, serde_json::Error> = from_value(json!([
        { "op": "replace", "path": "/statements/0/node/from_table/condition/function_name", "value": predicate },
        { "op": "replace", "path": "/statements/0/node/from_table/condition/children/0/column_names", "value": left_column_names },
        { "op": "replace", "path": "/statements/0/node/from_table/condition/children/1/column_names", "value": right_column_names }
    ]));

    match (ast, ast_patch) {
        (Ok(mut ast), Ok(ast_patch)) => {
            patch(&mut ast, &ast_patch).unwrap();

            return ast.to_string();
        }
        _ => panic!("Error parsing JSON."),
    }
}
