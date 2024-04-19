use crate::ast::{Condition, DuckDBAST, JoinNode};

/// Parses a spatial join expressed as the JSON AST of a DuckDB query and
/// converts the join condition to an S-expression.
///
/// This function uses the partial AST definition in [`crate::ast`] to
/// deserialize the JSON AST into a Rust struct.
pub fn parse_duckdb_ast_to_s_exp(contents: &str) -> String {
    let ast: Result<DuckDBAST, serde_json::Error> = serde_json::from_str(&contents);

    match ast {
        Ok(ast) => parse_join_node(&ast.statements[0].node.from_table),
        Err(e) => panic!("Error parsing JSON: {}", e),
    }
}

/// Parses a JOIN AST node.
fn parse_join_node(from_table: &JoinNode) -> String {
    return parse_condition(&from_table.condition);
}

/// Parses a Condition, expressed as either a FUNCTION AST node or a
/// CONJUNCTION_AND AST node.
fn parse_condition(condition: &Condition) -> String {
    match condition {
        Condition::FUNCTION {
            function_name,
            children,
        } => {
            return parse_function_node(function_name, children);
        }
        Condition::CONJUNCTION_AND { children } => {
            return parse_conjunction_and_node(children);
        }
    }
}

/// Parses a FUNCTION AST node.
fn parse_function_node(function_name: &str, children: &Vec<crate::ast::ColumnRef>) -> String {
    let left = children[0].column_names.join(".");
    let right = children[1].column_names.join(".");

    return format!("({} {} {})", function_name, left, right);
}

/// Parses a CONJUNCTION_AND AST node.
fn parse_conjunction_and_node(children: &Vec<Condition>) -> String {
    let cs = children
        .iter()
        .map(parse_condition)
        .collect::<Vec<String>>();

    return format!("(AND {})", cs.join(" "));
}
