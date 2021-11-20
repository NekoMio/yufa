use grammar_struct_lib::grammar_struct::*;
use std::collections::HashMap;
use std::collections::HashSet;

use prettytable::{format, Cell, Row, Table};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SingleProduction {
    pub left: String,
    pub right: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct LR1item {
    pub production: SingleProduction,
    pub dot_index: usize,
    pub symbol: String,
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
                        let next_first = grammar.get_production_first_set(&next_symbol);
                        let mut new_symbol_vec: Vec<String>;
                        if next_symbol.len() == 0 || next_first.contains("ε") {
                            new_symbol_vec = vec![item.symbol.clone()];
                            new_symbol_vec.extend(
                                grammar
                                    .get_production_first_set(&next_symbol)
                                    .iter()
                                    .filter_map(|x| {
                                        if &item.symbol == x {
                                            None
                                        } else {
                                            Some(x.clone())
                                        }
                                    }),
                            );
                        } else {
                            new_symbol_vec = next_first.iter().map(|x| x.to_string()).collect();
                        }
                        for new_symbol in new_symbol_vec {
                            let new_item = LR1item {
                                production: SingleProduction {
                                    left: grammar.get_noterminal(&symbol).unwrap().to_string(),
                                    right: production.clone(),
                                },
                                dot_index: 0,
                                symbol: new_symbol,
                            };
                            closure.insert(new_item);
                        }
                    }
                }
            }
        }
        // println!("{:#?}", closure);
        if prev_closure == closure {
            break;
        }
    }
    // println!("{:#?}", closure);
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
        symbol: "$".to_string(),
    });
    let start_closure = closure(&grammar, start_kernel);
    closure_set.push(start_closure.clone());
    // println!("{} {:#?}", closure_set.len() - 1, start_closure);
    let mut go = HashMap::new();
    let mut index = 0;
    while index < closure_set.len() {
        let this_closure = closure_set[index].clone();
        let edges: HashSet<String> = this_closure
            .iter()
            .filter_map(|item| {
                if grammar.is_terminal(&item.production.right[item.dot_index..].to_string()) {
                    Some(
                        grammar
                            .get_terminal(&item.production.right[item.dot_index..].to_string())
                            .unwrap(),
                    )
                } else if grammar
                    .is_noterminal(&item.production.right[item.dot_index..].to_string())
                {
                    Some(
                        grammar
                            .get_noterminal(&item.production.right[item.dot_index..].to_string())
                            .unwrap(),
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
                                dot_index: item.dot_index + edge.len(),
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
                        // println!("{} {:#?}", closure_set.len(), new_closure);
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
                                dot_index: item.dot_index + edge.len(),
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
                        // println!("{} {:#?}", closure_set.len(), new_closure);
                        closure_set.push(new_closure);
                        go.insert((index, edge.clone()), closure_set.len() - 1);
                    }
                }
            }
        }
        index += 1;
    }
    // println!("============");
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
    let mut lr1_table = HashMap::new();
    let (closure_set, go) = generate_cloure_set(&grammar);
    // println!("{} {:#?}", closure_set.len(), closure_set);
    for i in 0..closure_set.len() {
        lr1_table.insert(i, HashMap::new());
    }
    for (k, v) in go {
        if grammar.is_terminal(&k.1) {
            if lr1_table[&k.0].contains_key(&k.1) {
                return Err(format!("{} {}", k.0, k.1));
            }
            lr1_table.get_mut(&k.0).unwrap().insert(
                k.1,
                (
                    // closure_set[v].iter().next().unwrap().symbol[0].clone(),
                    "S".to_string(),
                    v,
                ),
            );
        } else {
            if lr1_table[&k.0].contains_key(&k.1) {
                return Err(format!("{} {}", k.0, k.1));
            }
            lr1_table
                .get_mut(&k.0)
                .unwrap()
                .insert(k.1, ("".to_string(), v));
        }
    }
    for (index, closure) in closure_set.iter().enumerate() {
        // println!("{}", index);
        for item in closure {
            if item.dot_index == item.production.right.len() {
                let symbol = &item.symbol;
                if item.production.left == grammar.start {
                    if lr1_table[&index].contains_key(symbol) {
                        return Err(format!("{} {}", index, symbol));
                    }
                    lr1_table
                        .get_mut(&index)
                        .unwrap()
                        .insert(symbol.clone(), ("ACC".to_string(), 0));
                } else {
                    if lr1_table[&index].contains_key(symbol) {
                        return Err(format!("{} {}", index, symbol));
                    }
                    lr1_table.get_mut(&index).unwrap().insert(
                        symbol.clone(),
                        (
                            "R".to_string(),
                            grammar
                                .get_rule_id((&item.production.left, &item.production.right))
                                .unwrap(),
                        ),
                    );
                }
            }
        }
    }
    // print!("{:#?}", lr1_table);
    return Ok(lr1_table);
}

/// LR1 语法分析器
///
/// ```
/// let result_lr = run_lr1(str, grammar);
/// ```
/// 返回运行循环次数，或者错误
pub fn run_lr1(
    contents: &str,
    grammar: &Grammar,
    table: &HashMap<usize, HashMap<String, (String, usize)>>,
) -> Result<Table, String> {
    let mut print_table = Table::new();
    print_table.set_titles(row![bFy => "步骤", "栈", "输入", "动作"]);
    let mut stack = vec![(0, "$".to_string())];
    let mut contents = contents.trim().to_string() + "$";
    let mut step = 0;
    while contents.len() > 0 {
        let (top_status, _) = stack.last().unwrap();
        let next_symbol;
        if grammar.is_terminal(&contents) {
            next_symbol = grammar.get_terminal(&contents).unwrap();
        } else {
            return Err(format!("存在非终结符的内容 {}", contents));
        }
        if table.get(top_status).unwrap().contains_key(&next_symbol) {
            let (action, next_status) = table.get(top_status).unwrap().get(&next_symbol).unwrap();
            if action == "S" {
                stack.push((*next_status, next_symbol.clone()));
                let mut stack_table = Table::new();
                stack_table.add_row(Row::new(
                    stack.iter().map(|x| Cell::new(&x.0.to_string())).collect(),
                ));
                stack_table.add_row(Row::new(
                    stack.iter().map(|x| Cell::new(&x.1.to_string())).collect(),
                ));
                stack_table.set_format(*format::consts::FORMAT_NO_BORDER);
                print_table.add_row(row![
                    step,
                    stack_table,
                    contents,
                    "移进".to_string() + &next_status.to_string()
                ]);
                contents = contents[next_symbol.len()..].trim().to_string();
            } else if action == "R" {
                let rule_id = next_status;
                let rule = grammar.get_id_rule(rule_id).unwrap();
                let mut mut_rule = rule.clone();
                let mut now_stack = Vec::new();
                while mut_rule.1.len() > 0 {
                    if grammar.is_terminal(&mut_rule.1) {
                        now_stack.push(grammar.get_terminal(&mut_rule.1).unwrap());
                        mut_rule.1 = mut_rule.1[grammar.get_terminal(&mut_rule.1).unwrap().len()..]
                            .to_string();
                    } else {
                        now_stack.push(grammar.get_noterminal(&mut_rule.1).unwrap());
                        mut_rule.1 = mut_rule.1
                            [grammar.get_noterminal(&mut_rule.1).unwrap().len()..]
                            .to_string();
                    }
                }
                while now_stack.len() > 0 {
                    if now_stack.last().unwrap() == &stack.last().unwrap().1 {
                        now_stack.pop();
                        stack.pop();
                    } else {
                        print_table.printstd();
                        return Err(format!("{} -> {} 规约失败", rule.0, rule.1));
                    }
                }
                let (top_status, _) = stack.last().unwrap();
                let next_status = table.get(top_status).unwrap().get(&rule.0).unwrap().1;
                stack.push((next_status, rule.0.clone()));
                let mut stack_table = Table::new();
                stack_table.add_row(Row::new(
                    stack.iter().map(|x| Cell::new(&x.0.to_string())).collect(),
                ));
                stack_table.add_row(Row::new(
                    stack.iter().map(|x| Cell::new(&x.1.to_string())).collect(),
                ));
                stack_table.set_format(*format::consts::FORMAT_NO_BORDER);
                print_table.add_row(row![
                    step,
                    stack_table,
                    contents,
                    "规约".to_string() + &rule.0.to_string() + "->" + &rule.1.to_string()
                ]);
            } else if action == "ACC" {
                let mut stack_table = Table::new();
                stack_table.add_row(Row::new(
                    stack.iter().map(|x| Cell::new(&x.0.to_string())).collect(),
                ));
                stack_table.add_row(Row::new(
                    stack.iter().map(|x| Cell::new(&x.1.to_string())).collect(),
                ));
                stack_table.set_format(*format::consts::FORMAT_NO_BORDER);
                print_table.add_row(row![step, stack_table, contents, "接受".to_string(),]);
                contents = contents[next_symbol.len()..].trim().to_string();
                // return Ok(print_table);
            }
        } else {
            print_table.printstd();
            return Err(format!("{} {}", top_status, next_symbol));
        }
        step += 1;
    }
    return Ok(print_table);
}
