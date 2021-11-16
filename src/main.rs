// 语法分析

mod read;

use read::*;

/// LL1 语法分析器
/// ```
/// let result_ll = run_ll1(str);
/// ```
/// 返回运行循环次数，或者错误
fn run_ll1(input: &str) -> Result<u32, String> {
    return Ok(0);
}

/// LR1 语法分析器
/// 
/// ```
/// let result_lr = run_lr1(str);
/// ```
/// 返回运行循环次数，或者错误
fn run_lr1(input: &str) -> Result<u32, String> {
    return Ok(0);
}



fn main() {
    let args: Vec<String> = std::env::args().collect();
    let argv: usize = args.len();
    let contents : String;
    let grammar : Grammar;

    match argv {
        1 => {
            grammar = readgrammar();
            contents = readcontent();
        },
        2 => {
            let filename = &args[1];
            contents = readfile(filename);
            grammar = readgrammar();
        },
        3 => {
            let filename = &args[1];
            let grammar_filename = &args[2];
            contents = readfile(filename);
            grammar = readgrammarfile(grammar_filename);
        }
        _ => {
            println!("Usage {}", args[0]);
            println!("Usage {} <filename>", args[0]);
            println!("Usage {} <filename> <grammarfile>", args[0]);
            return;
        }
    }
    println!("{:?}", grammar);
    println!("{}", contents);
    let result_ll = run_ll1(&contents);
    if let Err(error) = result_ll {
        println!("{}", error);
    }
    let result_lr = run_lr1(&contents);
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
    fn test_readgrammarfile() {
        let grammar = readgrammarfile("testgrammar.txt");
        assert_eq!(grammar.nonterminals, vec!["S".to_string(), "A".to_string(), "B".to_string()]);
        assert_eq!(grammar.terminals, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(grammar.start, "S".to_string());
        assert_eq!(grammar.rules, vec![
            ("S".to_string(), vec!["A".to_string(), "B".to_string()]),
            ("A".to_string(), vec!["a".to_string(), "B".to_string()]),
            ("B".to_string(), vec!["b".to_string(), "A".to_string()]),
        ]);
    }
}