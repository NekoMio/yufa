
use std::collections::HashSet;

/// 产生式结构体
#[derive(Debug, Clone, PartialEq)]
pub struct Production {
    pub left: String,
    pub right: HashSet<String>,
}

/// 语法结构体
///
/// @terminals: 终结符集合  
///
/// @nonterminals: 非终结符集合  
///
/// @rules: 产生式集合  
///
/// @start: 开始符
#[derive(Debug, Clone)]
pub struct Grammar {
    pub terminals: Vec<String>,
    pub nonterminals: Vec<String>,
    pub start: String,
    pub rules: Vec<Production>,
}

impl Grammar {
    /// 判断是否是~
    pub fn is_empty(&self, s: &String) -> bool {
        if s == "~" {
            return true;
        }
        return false;
    }

    /// 判断字符串最左端是否为终结符
    pub fn is_terminal(&self, s: &String) -> bool {
        for t in &self.terminals {
            if s.len() >= t.len() && s[0..t.len()] == t.to_string() {
                return true;
            }
        }
        return false;
    }

    /// 返回字符串最左端的终结符
    pub fn get_terminal(&self, s: &String) -> Result<String, String> {
        if self.is_empty(s) {
            return Ok(String::from("~"));
        }
        for t in &self.terminals {
            if s.len() >= t.len() && s[0..t.len()] == t.to_string() {
                return Ok(t.to_string());
            }
        }
        return Err(format!("{} 的最左端不是终结符", s));
    }

    /// 判断字符串最左端是否为非终结符
    pub fn is_noterminal(&self, s: &String) -> bool {
        for t in &self.nonterminals {
            if s.len() >= t.len() && s[0..t.len()] == t.to_string() {
                return true;
            }
        }
        return false;
    }

    /// 返回字符串最左端的非终结符
    pub fn get_noterminal(&self, s: &String) -> Result<String, String> {
        for t in &self.nonterminals {
            // println!("{} {}", t.len(), t);
            if s.len() >= t.len() && s[0..t.len()] == t.to_string() {
                if s[t.len()..].len() > 0 && s[t.len()..].chars().next().unwrap() == '\'' {
                   continue;
                }
                return Ok(t.to_string());
            }
        }
        return Err(format!("{} 的最左端不是非终结符", s));
    }
}