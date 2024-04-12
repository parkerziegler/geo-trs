use serde::{Deserialize, Serialize};

/// Represents the root node of the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct DuckDBAST {
    pub statements: Vec<Statement>,
}

/// Represents a Statement node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct Statement {
    pub node: Node,
}

/// Represents a Node node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct Node {
    pub from_table: FromTable,
}

/// Represents a FromTable node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct FromTable {
    pub condition: Condition,
    pub left: BaseTable,
    pub right: BaseTable,
}

/// Represents a Condition node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct Condition {
    pub function_name: String,
    pub children: Vec<ColumnRef>,
}

/// Represents a BaseTable node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct BaseTable {
    pub table_name: String,
}

/// Represents a ColumnRef node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct ColumnRef {
    pub column_names: Vec<String>,
}
