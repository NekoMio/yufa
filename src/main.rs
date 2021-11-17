// 语法分析

use clap::{App, Arg};

mod read;
use read::*;

use ll1_praser::ll1::*;
use lr1_praser::lr1::*;

use grammar_struct_lib::grammar_struct::*;

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
    }
    // println!("{:?}", grammar);

    if let Some(inputfile) = matches.value_of("inputfile") {
        contents = readfile(&inputfile);
    } else {
        contents = readcontent();
    }
    {
        let grammar = format_ll(&grammar).unwrap();
        println!("{:#?}", grammar);
        let result_ll = run_ll1(&contents, &grammar);
        if let Err(error) = result_ll {
            println!("{}", error);
        }
    }
    {
        // println!("{:?}", grammar);
        // println!("{}", contents);
        let result_lr = run_lr1(&contents, &grammar);
        if let Err(error) = result_lr {
            println!("{}", error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::collections::HashMap;

    #[test]
    /// 测试文件读取
    fn test_readfile() {
        let contents = readfile("test/test.txt");
        assert_eq!(contents, "hello world");
    }
    #[test]
    /// 从文件中读取文法
    fn test_readgrammarfile() {
        let grammar = readgrammarfile("test/testgrammar.txt");
        assert_eq!(
            grammar.nonterminals,
            vec!["S".to_string(), "A".to_string(), "B".to_string()]
        );
        assert_eq!(grammar.terminals, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(grammar.start, "S".to_string());
        let mut rules_rights = HashSet::new();
        rules_rights.insert("A".to_string());
        rules_rights.insert("B".to_string());
        let mut rules_righta = HashSet::new();
        rules_righta.insert("a".to_string());
        rules_righta.insert("B".to_string());
        let mut rules_rightb = HashSet::new();
        rules_rightb.insert("b".to_string());
        rules_rightb.insert("A".to_string());
        assert_eq!(
            grammar.rules,
            vec![
                Production{ left:"S".to_string(), right: rules_rights},
                Production{ left:"A".to_string(), right: rules_righta},
                Production{ left:"B".to_string(), right: rules_rightb},
            ]
        );
    }
    #[test]
    /// 测试文法转化
    fn test_format_ll() {
        let grammar = readgrammarfile("test/testgrammar.txt");
        let grammar = format_ll(&grammar).unwrap();
        assert_eq!(
            grammar.nonterminals,
            vec!["S".to_string(), "A".to_string(), "B".to_string()]
        );
        assert_eq!(grammar.terminals, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(grammar.start, "S".to_string());
        let mut rules_rights = HashSet::new();
        rules_rights.insert("A".to_string());
        rules_rights.insert("B".to_string());
        let mut rules_righta = HashSet::new();
        rules_righta.insert("a".to_string());
        rules_righta.insert("B".to_string());
        let mut rules_rightb = HashSet::new();
        rules_rightb.insert("a".to_string());
        rules_rightb.insert("b".to_string());
        assert_eq!(
            grammar.rules,
            vec![
                Production{ left:"S".to_string(), right: rules_rights},
                Production{ left:"A".to_string(), right: rules_righta},
                Production{ left:"B".to_string(), right: rules_rightb},
            ]
        );
    }

    #[test]
    /// 测试FISRT集
    fn test_get_first_set() {
        let grammar = readgrammarfile("test/testgrammar.txt");
        let grammar = format_ll(&grammar).unwrap();
        // let first_set = get_first_set(&grammar);
        let mut first_right_set = HashSet::new();
        first_right_set.insert("a".to_string());
        first_right_set.insert("b".to_string());
        let mut first_set_ok = HashMap::new();
        first_set_ok.insert("A".to_string(), first_right_set.clone());
        first_set_ok.insert("B".to_string(), first_right_set.clone());
        first_set_ok.insert("S".to_string(), first_right_set.clone());
        assert_eq!(grammar.first, first_set_ok);
    }
    #[test]
    /// 测试FOLLOW集
    fn test_get_follow_set() {
        let grammar = readgrammarfile("test/testgrammar.txt");
        let grammar = format_ll(&grammar).unwrap();
        let mut follow_right_set = HashSet::new();
        follow_right_set.insert("$".to_string());
        let mut follow_set_ok = HashMap::new();
        follow_set_ok.insert("A".to_string(), follow_right_set.clone());
        follow_set_ok.insert("B".to_string(), follow_right_set.clone());
        follow_set_ok.insert("S".to_string(), follow_right_set.clone());
        assert_eq!(grammar.follow, follow_set_ok);
    }
}
