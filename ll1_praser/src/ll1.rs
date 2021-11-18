use grammar_struct_lib::grammar_struct::*;
use std::collections::HashSet;
use std::collections::HashMap;

use prettytable::{Table};
// use read::*;

/// 生成 LL1 文法分析表
/// 
/// @param grammar 文法结构
/// 
/// @return LL1 文法分析表
/// ```
/// let ll1_table = ll1_table(grammar);
/// ```
pub fn generate_ll1_table(grammar: &Grammar) -> Result<HashMap<String, HashMap<String, String>>, String> {
    let mut ll1_table = HashMap::new();
    let follow_set = &grammar.follow;
    for nonterminal in &grammar.nonterminals {
        ll1_table.insert(nonterminal.clone(), HashMap::new());
    }
    for production in &grammar.rules {
        let nonterminal = &production.left;
        for right in &production.right {
            let right_first_set = grammar.get_production_first_set(right);
            for right_first in &right_first_set {
                if right_first != &"ε" {
                    if ll1_table.get(nonterminal).unwrap().get(right_first).is_none() || ll1_table.get(nonterminal).unwrap().get(right_first).unwrap() == &"synch" {
                        ll1_table.get_mut(nonterminal).unwrap().insert(right_first.clone(), right.clone());
                    } else {
                        return Err(format!("{} - {} 存在冲突", nonterminal, right_first));
                    }
                }
            }
            if right_first_set.contains("ε") {
                for follow in follow_set.get(nonterminal).unwrap() {
                    if ll1_table.get(nonterminal).unwrap().get(follow).is_none() || ll1_table.get(nonterminal).unwrap().get(follow).unwrap() == &"synch" {
                        ll1_table.get_mut(nonterminal).unwrap().insert(follow.clone(), "ε".to_string());
                    } else {
                        return Err(format!("{} - {} 存在冲突", nonterminal, follow));
                    }
                }
            } else {
                for follow in follow_set.get(nonterminal).unwrap() {
                    if ll1_table.get(nonterminal).unwrap().get(follow).is_none() {
                        ll1_table.get_mut(nonterminal).unwrap().insert(follow.clone(), "synch".to_string());
                    }
                }
            }
        }
    }   

    return Ok(ll1_table);
}


/// LL1 语法分析器
/// ```
/// let result_ll = run_ll1(str, grammar);
/// ```
/// 返回Table，或者错误
pub fn run_ll1(contents: &str, grammar: &Grammar, ll1_table: &HashMap<String, HashMap<String, String>>) -> Result<Table, String> {
    let mut stack = vec!["$".to_string(), grammar.start.to_string()];
    let mut contents = contents.trim().to_string() + "$";
    let mut print_table = Table::new();
    print_table.set_titles(row![bFy => "步骤", "栈", "输入串", "动作"]);
    let mut step = 0;
    while contents.len() > 0 {
        let top = stack.pop().unwrap();
        // println!("{} {}", step, contents);
        if grammar.is_terminal(&top) {
            let next_symbol = grammar.get_terminal(&contents);
            if let Err(_err) = next_symbol {
                return Err(format!("读入文法中存在非终结符的内容，错误的符号为 {}", contents));
            }
            if top == next_symbol.unwrap() {
                contents = contents.replacen(top.as_str(), "", 1).trim().to_string();
                // println!("{} {} {} {}", step, stack.join(" "), contents, "匹配");
                // let stach_str = ;
                if top == "$" {
                    print_table.add_row(row![step.to_string(), stack.join(" "), contents, "ACC接受"]);
                } else {
                    print_table.add_row(row![step.to_string(), stack.join(" "), contents, "匹配".to_string() + top.as_str()]);
                }
            } else {
                return Err(format!("读入文法匹配失败，错误的符号为 {}", contents));
            }
        } else {
            let next_symbol = grammar.get_terminal(&contents);
            if let Err(_err) = next_symbol {
                return Err(format!("读入文法中存在非终结符的内容，错误的符号为 {}", contents));
            }
            let next_symbol = next_symbol.unwrap();
            let next_production = ll1_table.get(&top).unwrap().get(&next_symbol);
            if let Some(production) = next_production {
                let mut production_vec = Vec::new();
                let mut production_str = production.to_string();
                if !grammar.is_empty(&production_str) {
                    while production_str.len() > 0 {
                        if grammar.is_terminal(&production_str) {
                            production_vec.push(grammar.get_terminal(&production_str).unwrap());
                            production_str = production_str.replacen(grammar.get_terminal(&production_str).unwrap().as_str(), "", 1);
                        } else {
                            production_vec.push(grammar.get_noterminal(&production_str).unwrap());
                            production_str = production_str.replacen(grammar.get_noterminal(&production_str).unwrap().as_str(), "", 1);
                        }
                    }
                    production_vec.reverse();
                    stack.append(&mut production_vec);
                }
                print_table.add_row(row![step.to_string(), stack.join(" "), contents, top + "->" + production.as_str()]);
            } else {
                return Err(format!("读入文法匹配失败，错误的符号为 {}", contents));
            }
            // contents = contents.replace(next_symbol.as_str(), "");
        }
        step += 1;
    }
    return Ok(print_table);
}

/// 尝试整理为ll1文法
/// ```
/// let grammar = format_ll(grammar).unwarp();
/// ```
/// 返回format后的grammar或者错误
pub fn format_ll(grammar: &Grammar) -> Result<Grammar, String> {
    let mut grammar = grammar.clone();
    let p = grammar.rules.len();
    for i in 0..p {
        for j in 0..i {
            let mut cont = HashSet::new();
            let left = &grammar.rules[j].left;
            for right in &grammar.rules[i].right {
                if right.len() >= left.len() && right[0..left.len()] == *left {
                    for right_j in &grammar.rules[j].right {
                        if right_j.to_string() + &right[1..] != grammar.rules[i].left {
                            cont.insert((right_j.to_string() + &right[1..]).clone());
                        }
                    }
                }
            }
            for right in &grammar.rules[i].right {
                if right.len() < left.len() || right[0..left.len()] != *left {
                    cont.insert(right.clone());
                }
            }
            grammar.rules[i].right = cont.clone();
        }
    }
    // println!("{:#?}", grammar);
    for i in 0..p {
        let left = &grammar.rules[i].left;
        // grammar.rules[i].right;
        let mut flag = false;
        {
            for right in &grammar.rules[i].right {
                if right.len() >= left.len() && right[0..left.len()] == *left {
                    flag = true;
                    break;
                }
            }
        }
        if !flag {
            continue;
        }
        let leftp = left.clone() + "'";
        let mut right_vecp = HashSet::new();
        let mut cont = HashSet::new();
        right_vecp.insert(String::from("ε"));
        for right in &grammar.rules[i].right {
            if right.len() >= left.len() && right[0..left.len()] == *left {
                right_vecp.insert((right[left.len()..].to_string() + &leftp).clone());
            } else {
                cont.insert((right.to_string() + &leftp).clone());
            }
        }
        grammar.rules[i].right = cont.clone();
        grammar.rules.push(Production {
            left: leftp,
            right: right_vecp,
        });
    }
    // 这里应该有个提取左公因子

    for rule in &grammar.rules {
        for right in &rule.right {
            if right.len() >= rule.left.len() && right[0..rule.left.len()] == rule.left {
                return Err(format!("{} 消除左递归失败", rule.left));
            }
        }
        for (i, right) in rule.right.iter().enumerate() {
            for (j, right_j) in rule.right.iter().enumerate() {
                if i == j {
                    continue;
                }
                if right.len() == right_j.len() && right == right_j {
                    return Err(format!("{} 有左公因子", rule.left));
                }
            }
        }
    }
    let mut noterminals = grammar.nonterminals.clone();
    for rule in &grammar.rules {
        if !noterminals.contains(&rule.left) {
            noterminals.push(rule.left.clone());
        }
    }
    grammar.nonterminals = noterminals;
    grammar.first = grammar.get_first_set();
    grammar.follow = grammar.get_follow_set(&grammar.first);
    return Ok(grammar);
}
