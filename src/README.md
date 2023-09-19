# Hello Rust

> 欢迎来的 Rust 的世界！！！

## Init & Run

要运行 Rust 程序，你需要遵循以下步骤：

1. **安装 Rust**:
   如果你还没有安装 Rust，你可以从 [Rust 官方网站](https://www.rust-lang.org/) 下载并安装 Rust。安装 Rust 会同时安装 `cargo`，这是 Rust 的官方包管理器和构建工具。

   在 Unix-like 系统上，你可以使用以下命令安装 Rust：
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh
   ```

2. **创建一个新的 Rust 项目**:
   使用 `cargo` 创建一个新的 Rust 项目：
   ```bash
   cargo new project_name
   cd project_name
   ```

3. **编写代码**:
   使用你喜欢的文本编辑器或 IDE 打开 `src/main.rs` 文件，并编写或粘贴你的 Rust 代码。

4. **构建并运行**:
   在项目目录中，使用以下命令构建并运行你的 Rust 程序：
   ```bash
   cargo run
   ```

   这会编译你的代码并运行生成的可执行文件。如果你只想编译但不运行，你可以使用：
   ```bash
   cargo build
   ```

5. **运行已编译的程序**:
   如果你已经使用 `cargo build` 编译了你的程序，你可以直接运行生成的可执行文件。它通常位于 `target/debug/` 目录下，并具有与你的项目名称相同的名称。例如：
   ```bash
   ./target/debug/project_name
   ```

## cargo-script

cargo-script 是一个 Cargo 的子命令，允许你直接运行 Rust "脚本"，而无需创建一个完整的 Cargo 项目。你可以使用以下命令安装它：

```bash
cargo install cargo-script
```

安装后，你可以直接运行 Rust 代码，例如：

```bash
# cargo script -e 'println!("Hello, world!");'
cargo script hello.rs --clear-cache
```
