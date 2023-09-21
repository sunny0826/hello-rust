# 示例 7: 泛型和特质 (Traits)

```rust
trait Describable {
    fn describe(&self) -> String;
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl Describable for Circle {
    fn describe(&self) -> String {
        format!("Circle with radius: {}", self.radius)
    }
}

impl Describable for Square {
    fn describe(&self) -> String {
        format!("Square with side: {}", self.side)
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 4.0 };

    println!("{}", circle.describe());
    println!("{}", square.describe());
}
```

**知识点讲解**:

1. **特质 (Traits)**: `trait` 关键字允许你定义一个特质，它是一种告诉 Rust 编译器某个特定类型应该具有哪些方法的方式。

2. **泛型**: 泛型是允许你定义函数、结构体和枚举的参数化类型的功能。在这个例子中，我们没有直接展示泛型，但特质经常与泛型一起使用，以提供更多的灵活性。

3. **为类型实现特质**: 使用 `impl` 关键字，你可以为特定的类型实现特质。在这里，我们为 `Circle` 和 `Square` 结构体实现了 `Describable` 特质。

4. **特质方法**: 一旦为类型实现了特质，该类型的实例就可以调用特质中定义的方法。在 `main` 函数中，我们调用了 `circle` 和 `square` 的 `describe` 方法。

特质和泛型是 Rust 中实现多态和代码重用的主要工具。它们提供了一种定义和实现共享行为的强大方式。
