use crate::ast::{Condition, DuckDBAST, FromTable};

/// Parses a spatial join expressed as the JSON AST of a DuckDB query and converts
/// the join condition to an S-expression of the form `(predicate left right)`.
///
/// This function uses the partial AST definition in [`crate::ast`] to deserialize
/// the JSON AST into a Rust struct.
pub fn parse_duckdb_ast_to_s_exp(contents: &str) -> String {
    let ast: Result<DuckDBAST, serde_json::Error> = serde_json::from_str(&contents);

    match ast {
        Ok(ast) => parse_from_table(&ast.statements[0].node.from_table),
        Err(e) => panic!("Error parsing JSON: {}", e),
    }
}

/// Parses a `FromTable` AST node.
fn parse_from_table(from_table: &FromTable) -> String {
    return parse_condition(&from_table.condition);
}

/// Parses a `Condition` AST node.
fn parse_condition(condition: &Condition) -> String {
    let predicate = condition.function_name.clone();
    let left = condition.children[0].column_names.join(".");
    let right = condition.children[1].column_names.join(".");

    println!("left: {}", left);
    println!("right: {}", right);

    return format!("({} {} {})", predicate, left, right);
}
