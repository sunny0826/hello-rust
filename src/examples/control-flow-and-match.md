# 示例 2: 控制流和模式匹配

```rust
fn main() {
    let number = 6;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 | 4 | 5 => println!("Three through Five"),
        _ => println!("Something else"),
    }
}
```

**知识点讲解**:

1. **match 表达式**: `match` 是 Rust 中的模式匹配操作符。它允许你根据值的不同模式来执行不同的操作。这是一个非常强大的控制流工具。

2. **模式**: 在 `match` 表达式中，每一个 `=>` 左边的部分都是一个模式。例如，`1` 和 `2` 都是值模式，而 `3 | 4 | 5` 是一个复合模式，表示匹配其中任何一个值。

3. **通配符模式**: `_` 是一个特殊的模式，称为通配符模式。它匹配任何值，并且常常用作 `match` 表达式的最后一个分支，以确保所有可能的值都被覆盖。

这种模式匹配的方式使得 Rust 的代码更加简洁和易读，同时也确保了代码的完整性和安全性，因为编译器会检查所有可能的模式是否都被处理。