
use grammar_struct_lib::grammar_struct::*;
use std::collections::HashSet;
use std::collections::HashMap;
// use read::*;

/// LL1 语法分析器
/// ```
/// let result_ll = run_ll1(str, grammar);
/// ```
/// 返回运行循环次数，或者错误
pub fn run_ll1(contents: &str, grammar: &Grammar) -> Result<u32, String> {
    return Ok(0);
}


/// 求文法的FIRST集
/// ```
/// let first_set = get_first_set(grammar);
/// ```
/// 返回FIRST集
/// HashMap<String, HashSet<String>>
/// 
/// key: 非终结符
/// 
/// value: FIRST集
pub fn get_first_set(grammar: &Grammar) -> HashMap<String, HashSet<String>> {
    let mut first_set: HashMap<String, HashSet<String>> = HashMap::new();
    for rule in &grammar.rules {
        first_set.insert(rule.left.clone(), HashSet::new());
    }
    let mut prev_first_set: HashMap<String, HashSet<String>> = first_set.clone();
    loop {
        for rule in &grammar.rules {
            let mut first_set_of_rule: HashSet<String> = HashSet::new();
            for symbol in rule.right.iter() {
                if grammar.is_terminal(symbol) || grammar.is_empty(symbol) {
                    first_set_of_rule.insert(grammar.get_terminal(symbol).unwrap().clone());
                } else {
                    let mut first_set_of_symbol: HashSet<String> = HashSet::new();
                    let mut mut_symbol = symbol.clone();
                    loop {
                        if mut_symbol.len() == 0 {
                            break;
                        } else if grammar.is_terminal(&mut_symbol) || grammar.is_empty(symbol) {
                            first_set_of_symbol.insert(grammar.get_terminal(&mut_symbol).unwrap().clone());
                            break;
                        } else {
                            // println!("{} {}", mut_symbol.len(), mut_symbol);
                            first_set_of_symbol.extend(first_set.get(&grammar.get_noterminal(&mut_symbol).unwrap()).unwrap().clone());
                            if !first_set.get(&grammar.get_noterminal(&mut_symbol).unwrap()).unwrap().contains("~") {
                                break;
                            }
                            mut_symbol = mut_symbol[grammar.get_noterminal(&mut_symbol).unwrap().len()..].trim().to_string();
                        }
                    }

                    first_set_of_rule.extend(first_set_of_symbol);
                }
            }
            first_set_of_rule.extend(first_set.get(&rule.left).unwrap().clone());
            first_set.insert(rule.left.clone(), first_set_of_rule);
        }
        
        if prev_first_set == first_set {
            break;
        } else {
            prev_first_set = first_set.clone();
        }
    }
    return first_set;
}

/// 消除左递归
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
    println!("{:#?}", grammar);
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
        right_vecp.insert(String::from("~"));
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
    return Ok(grammar);
}