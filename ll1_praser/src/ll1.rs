
use grammar_struct_lib::grammar_struct::*;
use std::collections::HashSet;
// use read::*;

/// LL1 语法分析器
/// ```
/// let result_ll = run_ll1(str, grammar);
/// ```
/// 返回运行循环次数，或者错误
pub fn run_ll1(contents: &str, grammar: &Grammar) -> Result<u32, String> {
    return Ok(0);
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