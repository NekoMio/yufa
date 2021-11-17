## 语法分析器
NekoMio

- [ ] LL1 文法分析
- [ ] LR1 文法分析

### 运行
```bash
cargo run
```

### 帮助菜单
```bash
cargo run -- -h
```
```
yufa 0.1
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
