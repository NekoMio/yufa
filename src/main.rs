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
                Production{ left:"S".to_string(), right: vec!["A".to_string(), "B".to_string()]},
                Production{ left:"A".to_string(), right: vec!["a".to_string(), "B".to_string()]},
                Production{ left:"B".to_string(), right: vec!["b".to_string(), "A".to_string()]},
            ]
        );
    }
}
