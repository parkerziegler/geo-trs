mod ast;
mod codegen;
mod cost;
mod parser;

use crate::ast::DuckDBAST;
use crate::codegen::codegen_ast;
use crate::cost::GeoTRSCostFunction;
use crate::parser::parse_duckdb_ast_to_s_exp;
use egg::{rewrite as rw, Extractor, RecExpr, Rewrite, Runner, SymbolLang};

pub fn rewrite_query(contents: &str) -> String {
    let ast: Result<DuckDBAST, serde_json::Error> = serde_json::from_str(&contents);
    let expr = parse_duckdb_ast_to_s_exp(&contents);

    let rules: &[Rewrite<SymbolLang, ()>] = &[
        rw!("commute-contains-within"; "(st_contains ?a ?b)" => "(st_within ?b ?a)"),
        rw!("commute-covered-by-contains"; "(st_covered_by ?a ?b)" => "(st_covers ?b ?a)"),
        rw!("prefer-st-coveredby"; "(st_within ?a ?b)" => "(st_covered_by ?a ?b)"),
        rw!("prefer-not-intersects"; "(st_disjoint ?a ?b)" => "(NOT (st_intersects ?a ?b))"),
        rw!("and-withins-as-equals"; "(AND (st_within ?a ?b) (st_within ?b ?a))" => "(st_equals ?a ?b)"),
        rw!("and-intersects-as-overlaps"; "(AND (st_intersects ?a ?b) (st_intersects ?b ?a))" => "(st_overlaps ?a ?b)"),
        rw!("prefer-st-dwithin-as-st-distance"; "(st_dwithin ?a ?b ?c)" => "(<= (st_distance ?a ?b) ?c)"),
    ];

    let start: RecExpr<SymbolLang> = expr.parse().unwrap();
    let runner = Runner::default().with_expr(&start).run(rules);
    let extractor = Extractor::new(&runner.egraph, GeoTRSCostFunction);
    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);
    println!("best_cost: {}", best_cost);
    println!("best_expr: {}", best_expr.to_string());

    codegen_ast(ast.unwrap(), best_expr.to_string()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_rewrite_within_query() {
        let input_file_path =
            "queries/chicago_powerplants_within/chicago_powerplants_within_input.json";
        let input_file_contents =
            fs::read_to_string(input_file_path).expect("Something went wrong reading the file");

        let output_file_path =
            "queries/chicago_powerplants_within/chicago_powerplants_within_output.json";
        let output_file_contents =
            fs::read_to_string(output_file_path).expect("Something went wrong reading the file");

        let actual = rewrite_query(&input_file_contents)
            .parse::<serde_json::Value>()
            .unwrap();
        let expected = output_file_contents.parse::<serde_json::Value>().unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_rewrite_and_intersects_query() {
        let input_file_path =
            "queries/chicago_climate_regions_overlaps/chicago_climate_regions_overlaps_input.json";
        let input_file_contents =
            fs::read_to_string(input_file_path).expect("Something went wrong reading the file");

        let output_file_path =
            "queries/chicago_climate_regions_overlaps/chicago_climate_regions_overlaps_output.json";
        let output_file_contents =
            fs::read_to_string(output_file_path).expect("Something went wrong reading the file");

        let actual = rewrite_query(&input_file_contents)
            .parse::<serde_json::Value>()
            .unwrap();
        let expected = output_file_contents.parse::<serde_json::Value>().unwrap();

        assert_eq!(actual, expected);
    }
}
