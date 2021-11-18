use grammar_struct_lib::grammar_struct::*;
use std::collections::HashMap;
use std::collections::HashSet;

use prettytable::Table;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SingleProduction {
    pub left: String,
    pub right: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct LR1item {
    pub production: SingleProduction,
    pub dot_index: usize,
    pub symbol: Vec<String>,
}

type Closure = HashSet<LR1item>;

fn closure(grammar: &Grammar, kernel: Closure) -> Closure {
    let mut closure = kernel.clone();
    loop {
        let prev_closure = closure.clone();
        for item in &prev_closure {
            if item.dot_index < item.production.right.len() {
                let symbol = item.production.right[item.dot_index..].to_string();
                if grammar.is_noterminal(&symbol) {
                    for production in &grammar.rules[&grammar.get_noterminal(&symbol).unwrap()] {
                        let next_symbol =
                            symbol[grammar.get_noterminal(&symbol).unwrap().len()..].to_string();
                        let mut new_symbol_vec = item.symbol.clone();
                        new_symbol_vec.extend(grammar.get_production_first_set(&next_symbol));
                        let new_item = LR1item {
                            production: SingleProduction {
                                left: grammar.get_noterminal(&symbol).unwrap().to_string(),
                                right: production.clone(),
                            },
                            dot_index: 0,
                            symbol: new_symbol_vec,
                        };
                        closure.insert(new_item);
                    }
                }
            }
        }
        if prev_closure == closure {
            break;
        }
    }
    return closure;
}

fn generate_cloure_set(grammar: &Grammar) -> (Vec<Closure>, HashMap<(usize, String), usize>) {
    let mut closure_set: Vec<Closure> = Vec::new();
    let mut start_kernel: Closure = HashSet::new();
    start_kernel.insert(LR1item {
        production: SingleProduction {
            left: grammar.start.clone(),
            right: grammar.rules[&grammar.start]
                .iter()
                .map(|x| x.clone())
                .collect::<Vec<String>>()
                .join(" "),
        },
        dot_index: 0,
        symbol: vec!["$".to_string()],
    });
    let start_closure = closure(&grammar, start_kernel);

    closure_set.push(start_closure.clone());
    let mut go = HashMap::new();
    let mut index = 0;
    while index < closure_set.len() {
        let this_closure = closure_set[index].clone();
        let edges: HashSet<String> = this_closure
            .iter()
            .filter_map(|item| {
                if grammar.is_terminal(&item.production.right[item.dot_index..].to_string()) {
                    Some(grammar.get_terminal(&item.production.right[item.dot_index..].to_string()).unwrap())
                } else if grammar
                    .is_noterminal(&item.production.right[item.dot_index..].to_string())
                {
                    Some(
                        grammar
                            .get_noterminal(&item.production.right[item.dot_index..].to_string()).unwrap(),
                    )
                } else {
                    None
                }
            })
            .map(|x| x.clone())
            .collect();
        for edge in edges {
            if grammar.is_terminal(&edge) {
                let new_kernel: Closure = this_closure
                    .iter()
                    .filter_map(|item| {
                        if grammar.is_terminal(&item.production.right[item.dot_index..].to_string())
                            && edge
                                == grammar
                                    .get_terminal(
                                        &item.production.right[item.dot_index..].to_string(),
                                    )
                                    .unwrap()
                        {
                            Some(LR1item {
                                production: item.production.clone(),
                                dot_index: item.dot_index + 1,
                                symbol: item.symbol.clone(),
                            })
                        } else {
                            None
                        }
                    })
                    .map(|x| x.clone())
                    .collect();
                let new_closure = closure(&grammar, new_kernel);
                match closure_set.iter().position(|x| x == &new_closure) {
                    Some(pos) => {
                        go.insert((index, edge.clone()), pos);
                    }
                    None => {
                        closure_set.push(new_closure);
                        go.insert((index, edge.clone()), closure_set.len() - 1);
                    }
                }
            } else {
                let new_kernel: Closure = this_closure
                    .iter()
                    .filter_map(|item| {
                        if grammar
                            .is_noterminal(&item.production.right[item.dot_index..].to_string())
                            && edge
                                == grammar
                                    .get_noterminal(
                                        &item.production.right[item.dot_index..].to_string(),
                                    )
                                    .unwrap()
                        {
                            Some(LR1item {
                                production: item.production.clone(),
                                dot_index: item.dot_index + 1,
                                symbol: item.symbol.clone(),
                            })
                        } else {
                            None
                        }
                    })
                    .map(|x| x.clone())
                    .collect();
                let new_closure = closure(&grammar, new_kernel);
                match closure_set.iter().position(|x| x == &new_closure) {
                    Some(pos) => {
                        go.insert((index, edge.clone()), pos);
                    }
                    None => {
                        closure_set.push(new_closure);
                        go.insert((index, edge.clone()), closure_set.len() - 1);
                    }
                }
            }
        }
        index += 1;
    }

    return (closure_set, go);
}

/// 生成 LR1 文法分析表
///
/// @param grammar 文法结构
///
/// @return LR1 文法分析表
/// ```
/// let lr1_table = lr1_table(grammar);
/// ```
pub fn generate_lr1_table(
    grammar: &Grammar,
) -> Result<HashMap<usize, HashMap<String, (String, usize)>>, String> {
    let grammar = grammar.extension();
    let mut lr1_table = HashMap::new();
    let (closure_set, go) = generate_cloure_set(&grammar);
    for i in 0..closure_set.len() - 1 {
        lr1_table.insert(i, HashMap::new());
    }
    for (k, v) in go {
        lr1_table.get_mut(&k.0).unwrap().insert(k.1, (
            // closure_set[v].iter().next().unwrap().symbol[0].clone(),
            "S".to_string(),
            v,
        ));
    }
    for (index, closure) in closure_set.iter().enumerate() {
        for item in closure {
            if item.dot_index == item.production.right.len() {
                for symbol in item.symbol.iter() {
                    // lr1_table.get_mut(&index).unwrap().insert(
                    //     symbol.clone(),
                    //     (
                    //         "R".to_string(),
                    //         grammar.get_production_id(&item.production.left, &item.production.right),
                    //     ),
                    // );
                }
            }
        }
    }
    return Ok(lr1_table);
}

/// LR1 语法分析器
///
/// ```
/// let result_lr = run_lr1(str, grammar);
/// ```
/// 返回运行循环次数，或者错误
pub fn run_lr1(contents: &str, grammar: &Grammar) -> Result<Table, String> {
    let print_table = Table::new();
    return Ok(print_table);
}
