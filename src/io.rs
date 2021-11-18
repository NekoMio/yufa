use std::fs::File;
use std::io::Read;
use std::io::Write;

use grammar_struct_lib::grammar_struct::*;

use std::collections::HashSet;

use prettytable::{Attr, Cell, Row, Table};

use std::collections::HashMap;

/// 读取文件
/// ```
/// let contents = readfile("test.txt");
/// ```
pub fn readfile(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
}

/// 从命令行读取内容
/// 返回文法串
pub fn readcontent() -> String {
    let mut content = String::new();
    println!("请输入文法串：");
    loop {
        if 0 == std::io::stdin().read_line(&mut content).unwrap() {
            return content;
        }
    }
    // return content;
}

/// 从命令行读取文法
///
/// 返回Grammar结构体
pub fn readgrammar() -> Grammar {
    let mut input = String::new();
    print!("请输入文法非终结符个数：");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("read error");
    let m = input.trim().parse::<u32>().expect("请输入整数");
    input.clear();
    print!("请输入文法非终结符：");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("read error");
    let mut nonterminal = input.trim().split_whitespace();
    let mut nonterminal_vec = Vec::new();
    for _ in 0..m {
        nonterminal_vec.push(nonterminal.next().unwrap().to_string());
    }
    input.clear();
    print!("请输入文法终结符个数：");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("read error");
    // std::io::stderr().write(input.to_string().as_bytes()).unwrap();
    let n = input.trim().parse::<u32>().expect("请输入整数");
    input.clear();
    print!("请输入文法终结符：");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("read error");
    let mut terminal = input.trim().split_whitespace();
    let mut terminal_vec = Vec::new();
    for _ in 0..n {
        terminal_vec.push(terminal.next().unwrap().to_string());
    }
    input.clear();
    print!("请输入文法开始符：");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("read error");
    let start = input.trim().to_string();
    input.clear();
    print!("请输入文法产生式个数：");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("read error");
    let p = input.trim().parse::<u32>().expect("请输入整数");
    input.clear();
    print!("请输入文法产生式：");
    std::io::stdout().flush().unwrap();
    let mut production_vec = Vec::new();
    for _ in 0..p {
        // production_vec.push(production.next().unwrap().to_string());
        // println!("{}", i);
        std::io::stdin().read_line(&mut input).expect("read error");
        let mut production = input.trim().split("->");
        let left = production.next().unwrap().to_string();
        let right = production.next().unwrap().split("|");
        let mut right_vec = HashSet::new();
        for r in right {
            right_vec.insert(r.to_string());
        }
        production_vec.push(Production {
            left,
            right: right_vec,
        });
        input.clear();
    }
    let grammar = Grammar::new(terminal_vec, nonterminal_vec, start, production_vec);
    return grammar;
}

/// 从文件中读取文法
///
/// ```
/// let grammar = readgrammarfile("test.txt");
/// ```
/// @param filename: 文件名
///
/// 返回Grammar结构体
pub fn readgrammarfile(filename: &str) -> Grammar {
    let contents = readfile(filename);
    let mut lines = contents.split("\n");
    let m = lines.next().unwrap().trim().parse::<u32>().unwrap();
    // println!("{}", m);
    let mut nonterminal = Vec::new();
    let mut nonterminal_line = lines.next().unwrap().trim().split_whitespace();
    for _ in 0..m {
        nonterminal.push(nonterminal_line.next().unwrap().trim().to_string());
    }
    let n = lines.next().unwrap().parse::<u32>().unwrap();
    // println!("{}", n);
    let mut terminal = Vec::new();
    let mut terminal_line = lines.next().unwrap().trim().split_whitespace();
    for _ in 0..n {
        terminal.push(terminal_line.next().unwrap().trim().to_string());
    }
    let start = lines.next().unwrap().trim().to_string();
    // println!("{}", start);
    let p = lines.next().unwrap().trim().parse::<u32>().unwrap();
    // println!("{}", p);
    let mut production = Vec::new();
    for _ in 0..p {
        let mut tmp = lines.next().unwrap().split("->");
        let left = tmp.next().unwrap().trim().to_string();
        let right = tmp.next().unwrap().split("|");
        let mut right_vec = HashSet::new();
        for r in right {
            right_vec.insert(r.trim().to_string());
        }
        production.push(Production {
            left,
            right: right_vec,
        });
    }
    let grammar = Grammar::new(terminal, nonterminal, start, production);
    return grammar;
}

/// 格式化输出LL(1)文法分析表
///
/// @param table: LL(1)文法分析表
///
/// @param grammar: 文法
///
/// ```text
/// +----+----------+----------+----------+----------+--------+-------+--------+-------+
/// |    | +        | -        | *        | /        | (      | )     | num    | $     |
/// +----+----------+----------+----------+----------+--------+-------+--------+-------+
/// | E  |          |          |          |          | E->TE' | synch | E->TE' | synch |
/// +----+----------+----------+----------+----------+--------+-------+--------+-------+
/// | T  | synch    | synch    |          |          | T->FT' | synch | T->FT' | synch |
/// +----+----------+----------+----------+----------+--------+-------+--------+-------+
/// | F  | synch    | synch    | synch    | synch    | F->(E) | synch | F->num | synch |
/// +----+----------+----------+----------+----------+--------+-------+--------+-------+
/// | E' | E'->+TE' | E'->-TE' |          |          |        | E'->ε |        | E'->ε |
/// +----+----------+----------+----------+----------+--------+-------+--------+-------+
/// | T' | T'->ε    | T'->ε    | T'->*FT' | T'->/FT' |        | T'->ε |        | T'->ε |
/// +----+----------+----------+----------+----------+--------+-------+--------+-------+
/// ```
pub fn ll1_table_print(table: &HashMap<String, HashMap<String, String>>, grammar: &Grammar) {
    let mut table_print = Table::new();
    println!("LL1 文法分析表: ");
    let mut row_vec = Vec::new();
    row_vec.push(Cell::new(""));
    for v in &grammar.terminals {
        row_vec.push(Cell::new(&v.to_string()).with_style(Attr::Bold));
    }
    row_vec.push(Cell::new("$").with_style(Attr::Bold));
    table_print.add_row(Row::new(row_vec));
    for k in &grammar.nonterminals {
        let v = table.get(k).unwrap();
        let mut row_vec = Vec::new();
        row_vec.push(Cell::new(&k.to_string()).with_style(Attr::Bold));
        for terminal in &grammar.terminals {
            if let Some(value) = v.get(terminal) {
                row_vec.push(Cell::new(&value.to_string()));
            } else {
                row_vec.push(Cell::new(""));
            }
        }
        if let Some(value) = v.get("$") {
            row_vec.push(Cell::new(&value.to_string()));
        } else {
            row_vec.push(Cell::new(""));
        }
        table_print.add_row(Row::new(row_vec));
    }
    table_print.printstd();
}

/// 格式化输出FIRST集
///
/// @param grammar: 文法
/// ```text
/// +----------+--------+
/// | 非终结符 | FIRST  |
/// +----------+--------+
/// | E        | num (  |
/// +----------+--------+
/// | T        | num (  |
/// +----------+--------+
/// | F        | num (  |
/// +----------+--------+
/// | E'       | - ε +  |
/// +----------+--------+
/// | T'       | / ε *  |
/// +----------+--------+
/// ```
pub fn first_set_print(grammar: &Grammar) {
    let mut first_print = Table::new();
    println!("FIRST集: ");
    first_print.add_row(row![bFy => "非终结符", "FIRST"]);

    for nonterminal in &grammar.nonterminals {
        let mut first_vec = Vec::new();
        first_vec.push(Cell::new(&nonterminal.to_string()).with_style(Attr::Bold));
        let first = grammar.first.get(nonterminal).unwrap();
        let mut first_str = String::new();
        for f in first {
            first_str.push_str(&f.to_string());
            first_str.push_str(" ");
        }
        first_vec.push(Cell::new(&first_str));
        first_print.add_row(Row::new(first_vec));
    }
    first_print.printstd();
}

pub fn follow_set_print(grammar: &Grammar) {
    let mut follow_print = Table::new();

    println!("FOLLOW集: ");
    follow_print.add_row(row![bFy => "非终结符", "FOLLOW"]);

    for nonterminal in &grammar.nonterminals {
        let mut follow_vec = Vec::new();
        follow_vec.push(Cell::new(&nonterminal.to_string()).with_style(Attr::Bold));
        let follow = grammar.follow.get(nonterminal).unwrap();
        let mut follow_str = String::new();
        for f in follow {
            follow_str.push_str(&f.to_string());
            follow_str.push_str(" ");
        }
        follow_vec.push(Cell::new(&follow_str));
        follow_print.add_row(Row::new(follow_vec));
    }

    follow_print.printstd();
}
