use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;

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
    // pub rules: Vec<Production>,
    pub rules: BTreeMap<String, HashSet<String>>,
    pub first: HashMap<String, HashSet<String>>,
    pub follow: HashMap<String, HashSet<String>>,
}

impl Grammar {
    /// 构造函数
    pub fn new(
        terminals: Vec<String>,
        nonterminals: Vec<String>,
        start: String,
        rules: BTreeMap<String, HashSet<String>>,
    ) -> Grammar {
        let mut ret = Grammar {
            terminals,
            nonterminals,
            start,
            rules,
            first: HashMap::new(),
            follow: HashMap::new(),
        };
        ret.first = ret.get_first_set();
        ret.follow = ret.get_follow_set(&ret.first);
        return ret;
    }

    /// 判断是否是ε
    pub fn is_empty(&self, s: &String) -> bool {
        if s == "ε" {
            return true;
        }
        return false;
    }

    /// 判断字符串最左端是否为终结符
    pub fn is_terminal(&self, s: &String) -> bool {
        if self.is_empty(s) {
            return true;
        }
        for t in &self.terminals {
            // println!("{} {}", t, s);
            if s.len() >= t.len() && s[0..t.len()] == t.to_string() {
                return true;
            }
        }
        if s == "$" {
            return true;
        }
        return false;
    }

    /// 返回字符串最左端的终结符
    pub fn get_terminal(&self, s: &String) -> Result<String, String> {
        if self.is_empty(s) {
            return Ok(String::from("ε"));
        }
        for t in &self.terminals {
            if s.len() >= t.len() && s[0..t.len()] == t.to_string() {
                return Ok(t.to_string());
            }
        }
        if s == "$" {
            return Ok(String::from("$"));
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

    /// 求文法的FIRST集
    /// ```
    /// let first_set = get_first_set(grammar);
    /// ```
    /// 返回FIRST集
    /// HashMap<String, HashSet<String>>
    ///
    /// key: 非终结符
    ///
    /// value: FIRST集
    pub fn get_first_set(&self) -> HashMap<String, HashSet<String>> {
        let grammar = self;
        let mut first_set: HashMap<String, HashSet<String>> = HashMap::new();
        for rule in &grammar.rules {
            first_set.insert(rule.0.clone(), HashSet::new());
        }
        let mut prev_first_set: HashMap<String, HashSet<String>> = first_set.clone();
        loop {
            for rule in &grammar.rules {
                let mut first_set_of_rule: HashSet<String> = HashSet::new();
                for symbol in rule.1.iter() {
                    if grammar.is_terminal(symbol) || grammar.is_empty(symbol) {
                        first_set_of_rule.insert(grammar.get_terminal(symbol).unwrap().clone());
                    } else {
                        let mut first_set_of_symbol: HashSet<String> = HashSet::new();
                        let mut mut_symbol = symbol.clone();
                        loop {
                            if mut_symbol.len() == 0 {
                                break;
                            } else if grammar.is_terminal(&mut_symbol) || grammar.is_empty(symbol) {
                                first_set_of_symbol
                                    .insert(grammar.get_terminal(&mut_symbol).unwrap().clone());
                                break;
                            } else {
                                // println!("{} {}", mut_symbol.len(), mut_symbol);
                                first_set_of_symbol.extend(
                                    first_set
                                        .get(&grammar.get_noterminal(&mut_symbol).unwrap())
                                        .unwrap()
                                        .clone(),
                                );
                                if !first_set
                                    .get(&grammar.get_noterminal(&mut_symbol).unwrap())
                                    .unwrap()
                                    .contains("ε")
                                {
                                    break;
                                }
                                mut_symbol = mut_symbol
                                    [grammar.get_noterminal(&mut_symbol).unwrap().len()..]
                                    .trim()
                                    .to_string();
                            }
                        }

                        first_set_of_rule.extend(first_set_of_symbol);
                    }
                }
                first_set_of_rule.extend(first_set.get(rule.0).unwrap().clone());
                first_set.insert(rule.0.clone(), first_set_of_rule);
            }
            if prev_first_set == first_set {
                break;
            } else {
                prev_first_set = first_set.clone();
            }
        }
        return first_set;
    }

    /// 求特定产生式的FIRST集
    /// 
    /// @param rule 产生式
    /// 
    /// @return FIRST集
    /// 
    /// ```
    /// let first_set = grammar.get_production_first_set(rule);
    /// ```
    /// 返回FIRST集
    /// HashSet<String>
    pub fn get_production_first_set(&self, rule: &String) -> HashSet<String> {
        let grammar = self;
        let mut production_first_set: HashSet<String> = HashSet::new();
        let mut mut_rule = rule.clone();
        while mut_rule.len() > 0 {
            if grammar.is_terminal(&mut_rule) || grammar.is_empty(&mut_rule) {
                production_first_set.insert(grammar.get_terminal(&mut_rule).unwrap().clone());
                break;
            } else if grammar.is_noterminal(&mut_rule) {
                production_first_set.extend(
                    grammar
                        .first
                        .get(&grammar.get_noterminal(&mut_rule).unwrap())
                        .unwrap()
                        .clone(),
                );
                if !grammar
                    .first
                    .get(&grammar.get_noterminal(&mut_rule).unwrap())
                    .unwrap()
                    .contains("ε")
                {
                    break;
                }
                mut_rule = mut_rule
                    [grammar.get_noterminal(&mut_rule).unwrap().len()..]
                    .trim()
                    .to_string();
            } else {
                // 应该不会运行到这里
                break;
            }
        }
        return production_first_set;
    }

    /// 判断一个非终结符之后的所有符号的FIRST是否都是空
    ///
    /// @param first_set FIRST集
    ///
    /// @param symbol 当前symbol
    /// ```
    /// all_empty(grammar, symbol, first_set);
    /// ```
    /// 返回bool
    /// true: 都是空
    /// false: 不是
    fn all_empty(&self, rule: &String, first_set: &HashMap<String, HashSet<String>>) -> bool {
        let grammar = self;
        let mut mut_rule = rule.clone();
        loop {
            if mut_rule.len() == 0 {
                return true;
            } else if grammar.is_terminal(&mut_rule) {
                return false;
            } else if grammar.is_noterminal(&mut_rule) {
                if first_set
                    .get(&grammar.get_noterminal(&mut_rule).unwrap())
                    .unwrap()
                    .contains("ε")
                {
                    mut_rule = mut_rule[grammar.get_noterminal(&mut_rule).unwrap().len()..]
                        .trim()
                        .to_string();
                    // continue;
                } else {
                    return false;
                }
            } else {
                // 应该是不会跑到这里的
                return false;
            }
        }
        // return true;
    }

    /// 求文法的FOLLOW集
    /// ```
    /// let follow_set = get_follow_set(grammar);
    /// ```
    /// 返回FOLLOW集
    /// HashMap<String, HashSet<String>>
    pub fn get_follow_set(
        &self,
        first_set: &HashMap<String, HashSet<String>>,
    ) -> HashMap<String, HashSet<String>> {
        let grammar = self;
        let mut follow_set: HashMap<String, HashSet<String>> = HashMap::new();
        for rule in &grammar.rules {
            follow_set.insert(rule.0.clone(), HashSet::new());
        }
        let mut start_follow_set = HashSet::new();
        start_follow_set.insert("$".to_string());
        follow_set.insert(grammar.start.clone(), start_follow_set);
        let mut pre_follow_set: HashMap<String, HashSet<String>> = follow_set.clone();
        loop {
            for rule in &grammar.rules {
                for symbol in rule.1.iter() {
                    let mut mut_symbol: String = symbol.clone();
                    loop {
                        if mut_symbol.len() == 0 || grammar.is_empty(&mut_symbol) {
                            break;
                        } else if grammar.is_terminal(&mut_symbol) {
                            mut_symbol = mut_symbol
                                [grammar.get_terminal(&mut_symbol).unwrap().len()..]
                                .trim()
                                .to_string();
                        } else if grammar.is_noterminal(&mut_symbol) {
                            let next_symbol = mut_symbol
                                [grammar.get_noterminal(&mut_symbol).unwrap().len()..]
                                .trim()
                                .to_string();
                            if next_symbol.len() == 0 {
                                let left_follow_set = follow_set.get(rule.0).unwrap().clone();
                                follow_set
                                    .get_mut(&grammar.get_noterminal(&mut_symbol).unwrap())
                                    .unwrap()
                                    .extend(left_follow_set);
                                break;
                            }
                            if grammar.all_empty(&next_symbol, first_set) {
                                let left_follow_set = follow_set.get(rule.0).unwrap().clone();
                                follow_set
                                    .get_mut(&grammar.get_noterminal(&mut_symbol).unwrap())
                                    .unwrap()
                                    .extend(left_follow_set);
                            }
                            if grammar.is_terminal(&next_symbol) {
                                follow_set
                                    .get_mut(&grammar.get_noterminal(&mut_symbol).unwrap())
                                    .unwrap()
                                    .insert(grammar.get_terminal(&next_symbol).unwrap().clone());
                            }
                            if grammar.is_noterminal(&next_symbol) {
                                // println!(
                                //     "{:#?}",
                                //     first_set
                                //         .get(&grammar.get_noterminal(&next_symbol).unwrap())
                                //         .unwrap()
                                // );
                                let mut mut_first_set = first_set
                                    .get(&grammar.get_noterminal(&next_symbol).unwrap())
                                    .unwrap()
                                    .clone();
                                mut_first_set.retain(|x| x != "ε");
                                follow_set
                                    .get_mut(&grammar.get_noterminal(&mut_symbol).unwrap())
                                    .unwrap()
                                    .extend(mut_first_set.clone());
                                if !first_set
                                    .get(&grammar.get_noterminal(&next_symbol).unwrap())
                                    .unwrap()
                                    .contains("ε")
                                {
                                    break;
                                }
                            }
                            mut_symbol = mut_symbol
                                [grammar.get_noterminal(&mut_symbol).unwrap().len()..]
                                .trim()
                                .to_string();
                        }
                    }
                }
            }

            if pre_follow_set == follow_set {
                break;
            } else {
                pre_follow_set = follow_set.clone();
            }
        }

        return follow_set;
    }

    /// 构建文法的拓广文法
    pub fn extension(&self) -> Grammar {
        let mut grammar = self.clone();
        let mut pre_production = HashSet::new();
        pre_production.insert(grammar.start.to_string());
        grammar.rules.insert(grammar.start.to_string() + "''", pre_production);
        grammar.start = grammar.start.to_string() + "''";
        grammar.nonterminals.push(grammar.start.to_string() + "''");
        grammar.first = grammar.first.clone();
        grammar.follow = grammar.follow.clone();
        return grammar;
    }
}
