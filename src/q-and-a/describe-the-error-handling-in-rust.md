# Describe the error handling in Rust

> 请描述 Rust 中的错误处理模型。特别是，解释 Result 和 Option 类型以及它们的用途。另外，请描述如何使用 ? 运算符进行错误传播。

在 Rust 中，错误处理是通过两个主要的枚举类型来完成的：`Option<T>` 和 `Result<T, E>`。

1. **`Option<T>`**:
   `Option<T>` 是一个枚举，用于表示一个值可能存在或不存在。它有两个变量：
   - `Some(T)`: 表示存在一个值 `T`。
   - `None`: 表示没有值。

   `Option` 类型通常用于表示可能缺失的值，这样你可以避免使用特殊值（如 `null` 或 `-1`）来表示缺失的情况。

   示例：
   ```rust
   fn find_name(id: u32) -> Option<String> {
       if id == 1 {
           Some("Alice".to_string())
       } else {
           None
       }
   }
   ```

2. **`Result<T, E>`**:
   `Result<T, E>` 是一个枚举，用于表示一个操作可能成功或失败。它有两个变量：
   - `Ok(T)`: 表示操作成功，并返回一个值 `T`。
   - `Err(E)`: 表示操作失败，并返回一个错误 `E`。

   `Result` 类型是 Rust 的主要错误处理机制。它允许你明确地处理错误，而不是简单地崩溃或返回特殊值。

   示例：
   ```rust
   fn divide(a: f64, b: f64) -> Result<f64, String> {
       if b == 0.0 {
           Err("Cannot divide by zero".to_string())
       } else {
           Ok(a / b)
       }
   }
   ```

3. **`?` 运算符**:
   `?` 运算符是 Rust 中的一个方便的错误传播机制。如果 `Result` 是 `Ok`，它会返回内部的值；如果是 `Err`，它会立即从当前函数返回错误。这使得错误处理代码更加简洁和直观。

   示例：
   ```rust
   fn foo() -> Result<i32, String> {
       let x = bar()?;  // 如果 bar 返回 Err，foo 也会立即返回 Err
       Ok(x + 1)
   }

   fn bar() -> Result<i32, String> {
       // ... some code ...
   }
   ```

这些错误处理机制确保了 Rust 代码的健壮性和可靠性，同时也使得错误处理变得更加直观和简洁。
