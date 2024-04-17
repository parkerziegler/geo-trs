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
    pub modifiers: Vec<String>,
    pub cte_map: CTEMap,
    pub select_list: Vec<SelectListEntry>,
    pub from_table: FromTable,
    pub where_clause: Option<Condition>,
    pub group_expressions: Vec<String>,
    pub group_sets: Vec<String>,
    pub aggregate_handling: String,
    pub having: Option<String>,
    pub sample: Option<String>,
    pub qualify: Option<String>,
}

/// Represents a CTEMap node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct CTEMap {
    pub map: Vec<String>,
}

/// Represents a SelectList node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct SelectListEntry {
    pub class: String,
    pub alias: String,
    pub query_location: i32,
    pub column_names: Vec<String>,
}

/// Represents a FromTable node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct FromTable {
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

/// Represents a Condition node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct Condition {
    pub class: String,
    pub alias: String,
    pub query_location: i32,
    pub function_name: String,
    pub schema: String,
    pub children: Vec<ColumnRef>,
    pub filter: Option<String>,
    pub order_bys: OrderBys,
    pub distinct: bool,
    pub is_operator: bool,
    pub export_state: bool,
    pub catalog: String,
}

/// Represents a BaseTable node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct BaseTable {
    pub alias: String,
    pub sample: Option<String>,
    pub query_location: i32,
    pub schema_name: String,
    pub table_name: String,
    pub column_name_alias: Vec<String>,
    pub catalog_name: String,
}

/// Represents a ColumnRef node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct ColumnRef {
    pub column_names: Vec<String>,
}

/// Represents an OrderBys node in the JSON AST of a DuckDB query.
#[derive(Serialize, Deserialize)]
pub struct OrderBys {
    pub orders: Vec<String>,
}
