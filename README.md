# minigrep
一个使用 Rust 实现的轻量级命令行文本搜索工具，支持大小写敏感与可选的大小写不敏感匹配。

## 功能说明
- 在指定文件中搜索目标字符串
- 默认区分大小写
- 通过环境变量 `IGNORE_CASE` 启用大小写不敏感匹配

## 使用方式
### 1. 区分大小写（默认）
```bash
cargo run -- <query> <file_path>
```

示例：
```bash
cargo run -- name poem.txt
```

### 2. 不区分大小写
通过设置环境变量 `IGNORE_CASE`：
```bash
IGNORE_CASE=1 cargo run -- <query> <file_path>
```

示例：
```bash
IGNORE_CASE=1 cargo run -- name poem.txt
```

## 运行测试
```bash
cargo test
```