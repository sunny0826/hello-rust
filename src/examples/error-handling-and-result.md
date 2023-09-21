# 示例 5: 错误处理和Result

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file_contents("hello.txt") {
        Ok(data) => println!("File contents: {}", data),
        Err(e) => println!("Error reading file: {}", e),
    }
}
```

**知识点讲解**:

1. **Result 枚举**: `Result<T, E>` 是 Rust 中用于错误处理的另一个核心枚举。它有两个变体：`Ok(T)` 表示成功的结果，`Err(E)` 表示错误。

2. **? 运算符**: 在函数返回 `Result` 类型时，你可以使用 `?` 运算符来简化错误处理。如果 `Result` 是 `Ok`，它会返回内部的值；如果是 `Err`，它会立即从当前函数返回这个 `Err`。

3. **匹配Result**: 在 `main` 函数中，我们使用 `match` 表达式来处理 `read_file_contents` 函数返回的 `Result`。这允许我们根据函数是否成功来采取不同的行动。

4. **标准库的错误处理**: `std::io::Error` 是标准库中用于表示 I/O 错误的类型。在这个示例中，我们使用它作为 `Result` 的错误类型。

错误处理是 Rust 的一个重要特性，它确保了代码的健壮性和可靠性。通过使用 `Result` 和 `?` 运算符，Rust 提供了一种简洁而强大的方式来处理可能的错误。
