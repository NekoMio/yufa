// 语法分析

use clap::{App, Arg};

#[macro_use]
extern crate prettytable;
mod io;
use io::*;

use ll1_praser::ll1::*;
use lr1_praser::lr1::*;

use grammar_struct_lib::grammar_struct::*;

fn main() {
    let matches = App::new("yufa")
        .version("0.4")
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
        first_set_print(&grammar);
        follow_set_print(&grammar);
        // println!("{:#?}", grammar);
        let ll1_table = generate_ll1_table(&grammar);
        // println!("{:#?}", ll1_table);
        if let Err(error) = ll1_table {
            println!("{}", error);
        } else {
            let ll1_table = ll1_table.unwrap();
            ll1_table_print(&ll1_table, &grammar);
            let result_ll = run_ll1(&contents, &grammar, &ll1_table);
            if let Err(error) = result_ll {
                println!("{}", error);
            } else {
                let result_ll = result_ll.unwrap();
                println!("LL1 分析过程为: ");
                result_ll.printstd();
            }
        }
    }
    {
        println!("\n\n");
        // println!("{:?}", grammar);
        // println!("{}", contents);
        let grammar = grammar.extension();
        let lr1_table = generate_lr1_table(&grammar);
        if let Err(error) = lr1_table {
            println!("{}", error);
        } else {
            let lr1_table = lr1_table.unwrap();
            lr1_table_print(&lr1_table, &grammar);
            let result_lr = run_lr1(&contents, &grammar, &lr1_table);
            if let Err(error) = result_lr {
                println!("{}", error);
            } else {
                let result_lr = result_lr.unwrap();
                println!("LR1 分析过程为: ");
                result_lr.printstd();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[test]
    /// 测试文件读取
    fn test_readfile() {
        let contents = readfile("test/test.txt");
        assert_eq!(contents, "num * (num + num / (num - num))");
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
        let mut ok_rules = BTreeMap::new();
        ok_rules.insert("S".to_string(), rules_rights);
        ok_rules.insert("A".to_string(), rules_righta);
        ok_rules.insert("B".to_string(), rules_rightb);
        assert_eq!(grammar.rules, ok_rules);
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
        rules_rights.insert("a".to_string());
        rules_rights.insert("b".to_string());
        let mut rules_righta = HashSet::new();
        rules_righta.insert("a".to_string());
        rules_righta.insert("B".to_string());
        let mut rules_rightb = HashSet::new();
        rules_rightb.insert("a".to_string());
        rules_rightb.insert("b".to_string());

        let mut ok_rules = BTreeMap::new();
        ok_rules.insert("S".to_string(), rules_rights);
        ok_rules.insert("A".to_string(), rules_righta);
        ok_rules.insert("B".to_string(), rules_rightb);
        assert_eq!(grammar.rules, ok_rules);
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
        first_set_print(&grammar);
    }
    #[test]
    /// 测试FOLLOW集
    fn test_get_follow_set() {
        let grammar = readgrammarfile("test/grammar.txt");
        let grammar = format_ll(&grammar).unwrap();
        let mut follow_right_set = HashSet::new();
        follow_right_set.insert("$".to_string());
        follow_right_set.insert(")".to_string());
        let mut follow_set_ok = HashMap::new();
        follow_set_ok.insert("E".to_string(), follow_right_set.clone());
        follow_set_ok.insert("E'".to_string(), follow_right_set.clone());
        follow_right_set.insert("+".to_string());
        follow_right_set.insert("-".to_string());
        follow_set_ok.insert("T".to_string(), follow_right_set.clone());
        follow_set_ok.insert("T'".to_string(), follow_right_set.clone());
        follow_right_set.insert("*".to_string());
        follow_right_set.insert("/".to_string());
        follow_set_ok.insert("F".to_string(), follow_right_set.clone());
        follow_set_print(&grammar);
        assert_eq!(grammar.follow, follow_set_ok);
        // follow_set_print(&grammar);
    }

    #[test]
    /// 测试LL1分析表
    fn test_generate_ll1_table() {
        let grammar = readgrammarfile("test/grammar.txt");
        let grammar = format_ll(&grammar).unwrap();
        let ll1_table = generate_ll1_table(&grammar).unwrap();
        ll1_table_print(&ll1_table, &grammar);
    }

    #[test]
    /// 测试LL1语法分析
    fn test_run_ll1() {
        let grammar = readgrammarfile("test/grammar.txt");
        let grammar = format_ll(&grammar).unwrap();
        let ll1_table = generate_ll1_table(&grammar).unwrap();
        let input = readfile("test/test.txt");
        let result = run_ll1(&input, &grammar, &ll1_table).unwrap();
        result.printstd();
    }

    #[test]
    /// 测试LR1分析表
    fn test_generate_lr1_table() {
        let grammar = readgrammarfile("test/grammar.txt");
        let lr1_table = generate_lr1_table(&grammar).unwrap();
        lr1_table_print(&lr1_table, &grammar);
    }

    #[test]
    /// 测试LR1语法分析
    fn test_run_lr1() {
        let grammar = readgrammarfile("test/grammar.txt");
        let grammar = grammar.extension();
        let lr1_table = generate_lr1_table(&grammar).unwrap();
        let input = readfile("test/test.txt");
        let result = run_lr1(&input, &grammar, &lr1_table).unwrap();
        result.printstd();
    }
}
