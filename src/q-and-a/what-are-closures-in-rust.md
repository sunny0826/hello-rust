# What are closures in Rust?

> 请描述 Rust 中的闭包（closures）是什么，以及它们与普通函数有何不同？另外，能否给出一个简单的闭包示例？

闭包（closures）是 Rust 中的匿名函数，可以捕获其环境中的值。它们与普通函数有几个主要的不同之处：

1. **匿名性**：闭包是匿名的，它们没有名称。
2. **捕获环境值**：闭包可以捕获其环境中的值，这意味着它们可以访问和使用其定义之外的变量。
3. **灵活的参数和返回类型**：闭包的参数和返回类型可以是隐式的，编译器会自动推断它们。
4. **简短的语法**：闭包提供了一种简洁的方式来定义小的函数块。

示例：

```rust
fn main() {
    let x = 10;

    // 闭包，捕获了 x 的值
    let add_to_x = |y| x + y;

    println!("{}", add_to_x(5)); // 输出 15
}
```

在上面的示例中，我们定义了一个闭包 `add_to_x`，它捕获了变量 `x` 的值。当我们调用这个闭包并传递 `5` 作为参数时，它会返回 `15`。

与此相对，普通函数是具名的，不能捕获其环境中的值，必须在模块级别定义，并且它们的参数和返回类型必须明确指定。