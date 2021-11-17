
/// 产生式结构体
#[derive(Debug, Clone, PartialEq)]
pub struct Production {
    pub left: String,
    pub right: Vec<String>,
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
