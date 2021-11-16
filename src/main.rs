// 语法分析

use clap::{App, Arg};

mod read;
use read::*;

/// LL1 语法分析器
/// ```
/// let result_ll = run_ll1(str, grammar);
/// ```
/// 返回运行循环次数，或者错误
fn run_ll1(contents: &str, grammar: &Grammar) -> Result<u32, String> {
    return Ok(0);
}

/// LR1 语法分析器
///
/// ```
/// let result_lr = run_lr1(str, grammar);
/// ```
/// 返回运行循环次数，或者错误
fn run_lr1(contents: &str, grammar: &Grammar) -> Result<u32, String> {
    return Ok(0);
}


/// 消除左递归
fn format_ll(grammar: &Grammar) -> Result<Grammar, String> {
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

fn main() {
    let matches = App::new("yufa")
        .version("0.1")
        .author("Nekomio <hui@nekomio.com>")
        .about("LL1 & LR1 语法分析器")
        .arg(
            Arg::with_name("grammar")
                .short("g")
                .long("grammar")
                .value_name("FILE")
                .help("语法文件")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("inputfile")
                .short("i")
                .long("inputfile")
                .value_name("FILE")
                .help("输入文件")
                .takes_value(true),
        )
        .get_matches();

    let contents: String;
    let grammar: Grammar;

    if let Some(grammarfile) = matches.value_of("grammar") {
        grammar = readgrammarfile(&grammarfile);
    } else {
        grammar = readgrammar();
        // println!("请指定语法文件");
        // return;
    }
    // println!("{:?}", grammar);

    if let Some(inputfile) = matches.value_of("inputfile") {
        contents = readfile(&inputfile);
    } else {
        contents = readcontent();
        // println!("请指定输入文件");
        // return;
    }
    {
        let grammar = format_to_ll1(&grammar).unwrap();
        println!("{:#?}", grammar);
        let result_ll = run_ll1(&contents, &grammar);
        if let Err(error) = result_ll {
            println!("{}", error);
        }
    }
    // println!("{:?}", grammar);
    println!("{}", contents);
    let result_lr = run_lr1(&contents, &grammar);
    if let Err(error) = result_lr {
        println!("{}", error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// 测试文件读取
    fn test_readfile() {
        let contents = readfile("test.txt");
        assert_eq!(contents, "hello world");
    }
    #[test]
    /// 从文件中读取文法
    fn test_readgrammarfile() {
        let grammar = readgrammarfile("testgrammar.txt");
        assert_eq!(
            grammar.nonterminals,
            vec!["S".to_string(), "A".to_string(), "B".to_string()]
        );
        assert_eq!(grammar.terminals, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(grammar.start, "S".to_string());
        assert_eq!(
            grammar.rules,
            vec![
                ("S".to_string(), vec!["A".to_string(), "B".to_string()]),
                ("A".to_string(), vec!["a".to_string(), "B".to_string()]),
                ("B".to_string(), vec!["b".to_string(), "A".to_string()]),
            ]
        );
    }
}
