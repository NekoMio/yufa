# 语法分析器
[![pipeline status](https://gitlab.com/NekoMio/yufa/badges/master/pipeline.svg)](https://gitlab.com/NekoMio/yufa/-/commits/master)  
NekoMio <hui@nekomio.com>

- [x] LL1 文法分析
- [ ] LR1 文法分析

### 运行
```bash
cargo run
```

## 使用
输入语法结构
从上到下分别是  
非终结符个数  
非终结符列表  
终结符个数  
终结符列表  
产生式行数  
产生式列表（要求同样的非终极符的产生式写在同一行）
```text
3
E T F
7
+ - * / ( ) num
E
3
E->E+T|E-T|T
T->T*F|T/F|F
F->(E)|num
```
以文件输入语法串时可以写成多行  
命令行输入只能写为一行


### 帮助菜单
```bash
cargo run -- -h
```
```
yufa 0.3
Nekomio <hui@nekomio.com>
LL1 & LR1 语法分析器

USAGE:
    yufa [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --grammar <FILE>      语法文件
    -i, --inputfile <FILE>    输入文件
```

### 构建
本平台
```
cargo build --release
```
交叉编译Windows
```
cargo build --release --target=x86_64-pc-windows-gnu
```


