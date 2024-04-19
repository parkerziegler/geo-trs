use serde_json::{json, Value};

use crate::ast::DuckDBAST;

struct BinOp {
    predicate: String,
    left_col_names: Vec<String>,
    right_col_names: Vec<String>,
}

pub fn codegen_ast(ast: DuckDBAST, s_exp: String) -> Value {
    let mut parser = lexpr::Parser::from_str(&s_exp);

    if let Some(v) = parser.next_value().unwrap() {
        match v {
            lexpr::Value::Cons(cons) => {
                let (vs, _) = cons.into_vec();

                if vs.len() == 3 {
                    let predicate = unbox_symbol(&vs[0]);
                    let left = unbox_symbol(&vs[1]);
                    let right = unbox_symbol(&vs[2]);

                    let left_col_names = left
                        .split(".")
                        .map(|s| s.to_owned())
                        .collect::<Vec<String>>();
                    let right_col_names = right
                        .split(".")
                        .map(|s| s.to_owned())
                        .collect::<Vec<String>>();

                    let bin_op = BinOp {
                        predicate,
                        left_col_names,
                        right_col_names,
                    };

                    return codegen_bin_op_ast(ast, bin_op);
                } else {
                    return json!({});
                }
            }
            _ => json!({}),
        }
    } else {
        json!({})
    }
}

fn unbox_symbol(v: &lexpr::Value) -> String {
    match v {
        lexpr::Value::Symbol(s) => String::from(&*s.to_owned()),
        _ => panic!("Expected symbol."),
    }
}

pub fn codegen_bin_op_ast(ast: DuckDBAST, bin_op: BinOp) -> Value {
    json!({
        "error": false,
        "statements": [
            {
                "node": {
                    "type": "SELECT_NODE",
                    "modifiers": [],
                    "cte_map": {
                        "map": []
                    },
                    "select_list": ast.statements[0].node.select_list,
                    "from_table": {
                        "type": "JOIN",
                        "alias": "",
                        "sample": null,
                        "query_location": ast.statements[0].node.from_table.query_location,
                        "left": ast.statements[0].node.from_table.left,
                        "right": ast.statements[0].node.from_table.right,
                        "condition": {
                            "class": "FUNCTION",
                            "type": "FUNCTION",
                            "alias": "",
                            "query_location": 0,
                            "function_name": bin_op.predicate,
                            "schema": "",
                            "children": [
                                {
                                    "class": "COLUMN_REF",
                                    "type": "COLUMN_REF",
                                    "alias": "",
                                    "query_location": 0,
                                    "column_names": bin_op.left_col_names
                                },
                                {
                                    "class": "COLUMN_REF",
                                    "type": "COLUMN_REF",
                                    "alias": "",
                                    "query_location": 0,
                                    "column_names": bin_op.right_col_names
                                }
                            ],
                            "filter": null,
                            "order_bys": {
                            "type": "ORDER_MODIFIER",
                            "orders": []
                            },
                            "distinct": false,
                            "is_operator": false,
                            "export_state": false,
                            "catalog": ""
                        },
                        "join_type": "INNER",
                        "ref_type": "REGULAR",
                        "using_columns": []
                    },
                    "where_clause": null,
                    "group_expressions": [],
                    "group_sets": [],
                    "aggregate_handling": "STANDARD_HANDLING",
                    "having": null,
                    "sample": null,
                    "qualify": null
                }
            }
        ]
    })
}
