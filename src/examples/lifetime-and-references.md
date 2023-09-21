# 示例 6: 生命周期和引用

```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);
}
```

**知识点讲解**:

1. **生命周期注解**: 在函数签名中，`'a` 是一个生命周期注解。它告诉 Rust 关于函数参数和返回值的引用如何与彼此相互关联。

2. **引用**: 在 Rust 中，引用允许你访问数据而不需要获取其所有权。在这个例子中，`longest` 函数接受两个字符串引用并返回一个字符串引用。

3. **生命周期的目的**: 生命周期确保 Rust 在编译时检查引用的有效性，从而避免悬挂引用和其他相关的错误。在这个例子中，生命周期注解 `'a` 表示两个输入字符串引用和返回的字符串引用必须具有相同的生命周期。

4. **String 和 str**: 在 Rust 中，`String` 是一个可增长的、堆分配的字符串类型，而 `str` 是一个字符串切片。使用 `as_str()` 方法，我们可以从 `String` 获取一个 `str` 引用。

生命周期是 Rust 中确保内存安全的关键概念之一。它们帮助编译器确保引用始终有效，从而避免运行时错误。
