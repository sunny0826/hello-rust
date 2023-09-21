# 示例 3: 结构体和方法

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect.area());
}
```

**知识点讲解**:

1. **结构体 (Structs)**: `struct` 关键字允许你创建一个自定义数据类型。在这个例子中，我们定义了一个名为 `Rectangle` 的结构体，它有两个字段：`width` 和 `height`。

2. **impl 块**: `impl` 关键字用于为结构体或枚举定义方法或关联函数。在这里，我们为 `Rectangle` 结构体定义了一个方法 `area`。

3. **方法与自引用**: 方法的第一个参数总是 `self`，它代表调用该方法的结构体实例。在这个例子中，`area` 方法计算矩形的面积。

4. **创建结构体实例**: 使用 `{}` 语法，我们可以创建一个结构体的实例。在 `main` 函数中，我们创建了一个 `Rectangle` 的实例并为其字段赋值。

5. **调用方法**: 使用 `.` 语法，我们可以调用结构体的方法。在这里，我们调用了 `rect` 的 `area` 方法来计算矩形的面积。

结构体是 Rust 中的基本构建块，它们允许你创建自定义数据类型，并为它们定义方法和关联函数，从而实现面向对象的编程风格。
