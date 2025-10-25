# RCLI 项目文档

## 项目概述

RCLI 是一个用 Rust 编写的命令行工具，提供以下主要功能：

1. **CSV 文件格式转换** - 将 CSV 文件转换为 JSON、YAML 或 TOML 格式
2. **随机密码生成** - 生成具有可配置强度和字符类型的随机密码

### 技术栈

- **Rust** - 主要编程语言 (Edition 2021)
- **Clap** - 命令行参数解析库，支持 derive 特性
- **Serde** - 序列化/反序列化框架
- **CSV** - CSV 文件处理库
- **rand** - 随机数生成库
- **zxcvbn** - 密码强度评估库

## 构建和运行

### 构建项目

```bash
# 开发构建
cargo build

# 发布构建（优化版本）
cargo build --release
```

### 运行项目

```bash
# 使用 cargo run
cargo run -- [子命令] [参数]

# 使用构建后的二进制文件
./target/debug/rcli [子命令] [参数]  # 开发版本
./target/release/rcli [子命令] [参数]  # 发布版本
```

### 测试

```bash
# 运行测试
cargo test

# 或者使用 nextest（如果配置了）
cargo nextest run
```

## 开发约定

### 代码质量检查

项目配置了完整的 pre-commit 钩子来确保代码质量：

```bash
# 代码格式化
cargo fmt

# 代码检查
cargo clippy --all-targets --all-features --tests --benches -- -D warnings

# 编译检查
cargo check --all

# 依赖检查
cargo deny check -d

# 拼写检查
typos
```

### Pre-commit 钩子

项目使用 `.pre-commit-config.yaml` 配置了以下检查：
- 基础文件检查（字节顺序标记、冲突、符号链接等）
- Python 代码格式化（black）
- Rust 代码格式化（cargo fmt）
- Rust 代码检查（cargo clippy, cargo check）
- 依赖安全检查（cargo deny）
- 拼写检查（typos）

## 项目结构

```
rcli/
├── src/
│   ├── main.rs          # 主程序入口
│   ├── lib.rs           # 库入口，导出公共 API
│   ├── opts.rs          # 命令行参数定义和解析
│   └── process/         # 核心处理逻辑模块
│       ├── mod.rs       # 模块入口，导出处理函数
│       ├── csv_convert.rs  # CSV 转换功能实现
│       └── gen_pass.rs  # 密码生成功能实现
├── assets/              # 示例数据文件
│   ├── juventus.csv     # 示例 CSV 数据
│   └── README.md        # 资源文件说明
├── Cargo.toml           # 项目配置和依赖
├── Cargo.lock           # 依赖锁定文件
├── .pre-commit-config.yaml  # Pre-commit 钩子配置
├── .gitignore           # Git 忽略文件配置
└── README.md            # 项目说明文档
```

### 模块说明

#### `src/main.rs`
主程序入口，解析命令行参数并根据子命令调用相应的处理函数。

#### `src/opts.rs`
定义命令行参数结构和解析逻辑：
- `Opts` - 顶级命令结构
- `SubCommand` - 子命令枚举（Csv, GenPass）
- `CsvOpts` - CSV 处理相关参数
- `GenPassOpts` - 密码生成相关参数
- `OutputFormat` - 输出格式枚举（Json, Yaml, Toml）

#### `src/process/csv_convert.rs`
CSV 转换功能实现：
- 读取 CSV 文件
- 转换为指定格式（JSON/YAML/TOML）
- 输出到指定文件

#### `src/process/gen_pass.rs`
密码生成功能实现：
- 根据指定长度和字符类型生成随机密码
- 使用 zxcvbn 库评估密码强度
- 支持大写字母、小写字母、数字和符号字符

## 使用示例

### CSV 格式转换

```bash
# 将 CSV 转换为 JSON（默认格式）
cargo run -- csv -i assets/juventus.csv

# 将 CSV 转换为 YAML 格式
cargo run -- csv -i assets/juventus.csv -f yaml -o output.yaml

# 将 CSV 转换为 TOML 格式
cargo run -- csv -i assets/juventus.csv -f toml -o output.toml

# 指定自定义分隔符
cargo run -- csv -i assets/juventus.csv -d ',' -f json

# 禁用表头处理
cargo run -- csv -i assets/juventus.csv --header false -f json
```

### 密码生成

```bash
# 生成默认长度（16）的密码
cargo run -- genpass

# 生成指定长度的密码
cargo run -- genpass -l 20

# 生成只包含数字和符号的密码
cargo run -- genpass -l 12 --no-uppercase --no-lowercase

# 生成只包含字母的密码
cargo run -- genpass -l 16 --no-number --no-symbol

# 生成包含所有字符类型的密码（默认）
cargo run -- genpass -l 16 --uppercase --lowercase --number --symbol
```

### 输出示例

#### CSV 转换输出
```json
[
  {
    "Name": "Wojciech Szczesny",
    "Position": "GK",
    "DOB": "18/04/1990",
    "Nationality": "Poland",
    "Kit Number": 25
  },
  // ... 更多记录
]
```

#### 密码生成输出
```
Generated password: Kj8#mN2$pQ9@xR5&
Password strength: 4
```

## 开发指南

### 添加新功能

1. 在 `src/opts.rs` 中添加新的子命令和参数结构
2. 在 `src/process/` 目录下创建新的处理模块
3. 在 `src/process/mod.rs` 中导出新模块
4. 在 `src/main.rs` 中添加对新命令的处理逻辑

### 代码风格

- 使用 `cargo fmt` 进行代码格式化
- 遵循 Rust 官方命名约定
- 使用 `cargo clippy` 检查代码质量
- 为公共 API 添加文档注释

### 依赖管理

- 使用 `cargo deny` 检查依赖安全性
- 在 `Cargo.toml` 中明确指定版本
- 定期更新依赖以获取安全修复

## 许可证

本项目采用 MIT 许可证。详情请参阅 LICENSE 文件。
