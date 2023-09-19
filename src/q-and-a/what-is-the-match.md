# What is the match

`match` 是 Rust 核心概念，它允许你对值进行模式匹配并根据匹配的模式执行相应代码。

例如，考虑一个 `Option<i32>` 类型的值。我们可以使用 `match` 语句来处理这个 `Option` 是否有值：

```rust
let some_value: Option<i32> = Some(5);

match some_value {
    Some(i) => println!("Value is: {}", i),
    None => println!("No value"),
}
```

在这个例子中，`match` 语句检查 `some_value` 的值。如果它是 `Some(i)`，则执行第一个分支的代码并打印值；如果它是 `None`，则执行第二个分支的代码。

`match` 语句确保了代码的完整性，因为它要求处理所有可能的情况。
