use egg::{rewrite as rw, *};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct DuckDBAST {
    statements: Vec<Statement>,
}

#[derive(Serialize, Deserialize)]
struct Statement {
    node: Node,
}

#[derive(Serialize, Deserialize)]
struct Node {
    from_table: FromTable,
}

#[derive(Serialize, Deserialize)]
struct FromTable {
    condition: Condition,
    left: BaseTable,
    right: BaseTable,
}

#[derive(Serialize, Deserialize)]
struct Condition {
    function_name: String,
    children: Vec<ColumnRef>,
}

#[derive(Serialize, Deserialize)]
struct BaseTable {
    table_name: String,
}

#[derive(Serialize, Deserialize)]
struct ColumnRef {
    column_names: Vec<String>,
}

fn parse_duckdb_ast_to_s_exp(contents: &str) -> String {
    let v: Result<DuckDBAST, serde_json::Error> = serde_json::from_str(&contents);

    match v {
        Ok(v) => parse_from_table(&v.statements[0].node.from_table),
        Err(e) => panic!("Error parsing JSON: {}", e),
    }
}

fn parse_from_table(from_table: &FromTable) -> String {
    return parse_condition(&from_table.condition);
}

fn parse_condition(condition: &Condition) -> String {
    let predicate = condition.function_name.clone();
    let left = condition.children[0].column_names.join(".");
    let right = condition.children[1].column_names.join(".");

    return format!("({} {} {})", predicate, left, right);
}

fn main() {
    let file_path = "queries/chicago_powerplants_ast.json";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let expr = parse_duckdb_ast_to_s_exp(&contents);

    let rules: &[Rewrite<SymbolLang, ()>] = &[
        rw!("commute-contains-within"; "(ST_Contains ?a ?b)" => "(ST_Within ?b ?a)"),
        rw!("commute-covered-by-contains"; "(ST_Covered_By ?a ?b)" => "(ST_Covers ?b ?a)"),
        rw!("prefer-st-coveredby"; "(st_within ?a ?b)" => "(ST_Covered_By ?a ?b)"),
        rw!("prefer-not-intersects"; "(ST_Disjoint ?a ?b)" => "(NOT (ST_Intersects ?a ?b))"),
        rw!("and-withins-as-equals"; "(AND (ST_Within ?a ?b) (ST_Within ?b ?a))" => "(ST_Equals ?a ?b)"),
    ];

    let start: RecExpr<SymbolLang> = expr.parse().unwrap();

    let runner = Runner::default().with_expr(&start).run(rules);

    let extractor = Extractor::new(&runner.egraph, AstSize);

    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);
    println!("expr: {}", start);
    println!("best: {}", best_expr);
    println!("Cost: {}", best_cost);
}
