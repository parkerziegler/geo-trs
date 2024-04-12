mod ast;
mod parser;
mod reconstruct;

use crate::parser::parse_duckdb_ast_to_s_exp;
use egg::{rewrite as rw, AstSize, Extractor, RecExpr, Rewrite, Runner, SymbolLang};

pub fn rewrite_query(contents: &str) -> String {
    let expr = parse_duckdb_ast_to_s_exp(&contents);

    let rules: &[Rewrite<SymbolLang, ()>] = &[
        rw!("commute-contains-within"; "(st_contains ?a ?b)" => "(st_within ?b ?a)"),
        rw!("commute-covered-by-contains"; "(st_covered_by ?a ?b)" => "(st_covers ?b ?a)"),
        rw!("prefer-st-coveredby"; "(st_within ?a ?b)" => "(st_covered_by ?a ?b)"),
        rw!("prefer-not-intersects"; "(st_disjoint ?a ?b)" => "(NOT (st_intersects ?a ?b))"),
        rw!("and-withins-as-equals"; "(AND (st_within ?a ?b) (st_within ?b ?a))" => "(st_equals ?a ?b)"),
    ];

    let start: RecExpr<SymbolLang> = expr.parse().unwrap();
    let runner = Runner::default().with_expr(&start).run(rules);
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (_, best_expr) = extractor.find_best(runner.roots[0]);

    let output = reconstruct::reconstruct(&contents, best_expr.to_string());

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_rewrite_query() {
        let input_file_path = "queries/chicago_powerplants/chicago_powerplants_input.json";
        let input_file_contents =
            fs::read_to_string(input_file_path).expect("Something went wrong reading the file");

        let output_file_path = "queries/chicago_powerplants/chicago_powerplants_output.json";
        let output_file_contents =
            fs::read_to_string(output_file_path).expect("Something went wrong reading the file");

        let actual = rewrite_query(&input_file_contents)
            .parse::<serde_json::Value>()
            .unwrap();
        let expected = output_file_contents.parse::<serde_json::Value>().unwrap();

        assert_eq!(actual, expected);
    }
}
