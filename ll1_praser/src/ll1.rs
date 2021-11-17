
use grammar_struct_lib::grammar_struct::*;
// use read::*;

/// LL1 语法分析器
/// ```
/// let result_ll = run_ll1(str, grammar);
/// ```
/// 返回运行循环次数，或者错误
pub fn run_ll1(contents: &str, grammar: &Grammar) -> Result<u32, String> {
    return Ok(0);
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
            let mut cont = Vec::new();
            let left = &grammar.rules[j].left;
            for right in &grammar.rules[i].right {
                if right[0..left.len()] == *left {
                    for right_j in &grammar.rules[j].right {
                        cont.push((right_j.to_string() + &right[1..]).clone());
                    }
                }
            }
            for right in &grammar.rules[i].right {
                if right[0..left.len()] != *left {
                    cont.push(right.clone());
                }
            }
            grammar.rules[i].right = cont.clone();
        }
    }
    for i in 0..p {
        let left = &grammar.rules[i].left;
        // grammar.rules[i].right;
        let mut flag = false;
        {
            for right in &grammar.rules[i].right {
                if right[0..left.len()] == *left {
                    flag = true;
                    break;
                }
            }
        }
        if !flag {
            continue;
        }
        let leftp = left.clone() + "'";
        let mut right_vecp = Vec::new();
        let mut cont = Vec::new();
        right_vecp.push(String::from("~"));
        for right in &grammar.rules[i].right {
            if right[0..left.len()] == *left {
                right_vecp.push((right[left.len()..].to_string() + &leftp).clone());
            } else {
                cont.push((right.to_string() + &leftp).clone());
            }
        }
        grammar.rules[i].right = cont.clone();
        grammar.rules.push(Production {
            left: leftp,
            right: right_vecp,
        });
    }
    return Ok(grammar);
}