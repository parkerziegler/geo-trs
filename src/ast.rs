use serde::{Deserialize, Serialize};

/// The root node of the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct DuckDBAST {
    pub error: bool,
    pub statements: Vec<Statement>,
}

// A statement object of the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct Statement {
    pub node: SelectNode,
}

/// The SELECT_NODE node of the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct SelectNode {
    #[serde(rename = "type")]
    pub type_: String,
    pub select_list: Vec<ColumnRef>,
    pub from_table: JoinNode,
}

/// The JOIN node of the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct JoinNode {
    #[serde(rename = "type")]
    pub type_: String,
    pub alias: String,
    pub sample: Option<String>,
    pub query_location: i32,
    pub left: BaseTable,
    pub right: BaseTable,
    pub condition: Condition,
    pub join_type: String,
    pub ref_type: String,
    pub using_columns: Vec<String>,
}

/// The BASE_TABLE node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct BaseTable {
    #[serde(rename = "type")]
    pub type_: String,
    pub alias: String,
    pub sample: Option<String>,
    pub query_location: i32,
    pub schema_name: String,
    pub table_name: String,
    pub column_name_alias: Vec<String>,
    pub catalog_name: String,
}

/// The CONDITION node in the JSON AST of a DuckDB query. For our current
/// benchmarks, this may be either a FUNCTION or a CONJUNCTION_AND.
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Condition {
    FUNCTION {
        function_name: String,
        children: Vec<ColumnRef>,
    },
    CONJUNCTION_AND {
        children: Vec<Condition>,
    },
}

/// The COLUMN_REF node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct ColumnRef {
    class: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub alias: String,
    pub query_location: i32,
    pub column_names: Vec<String>,
}
