# What is a trait in Rust

在 Rust 中，`trait` 是一种定义共享的行为或功能的方法。你可以将其视为其他语言中的接口。通过 `trait`，你可以定义一组方法，这些方法可以被多种不同的类型实现。这为 Rust 提供了一种多态性。

**用途：**

1. 定义共享的行为。
2. 为类型定义默认行为。
3. 与泛型结合使用，以约束泛型类型必须实现的行为。

**示例：**

```rust
// 定义一个 `Speak` trait，它有一个 `speak` 方法
trait Speak {
    fn speak(&self);
}

// 为 `Dog` 类型实现 `Speak` trait
struct Dog;
impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

// 为 `Cat` 类型实现 `Speak` trait
struct Cat;
impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn main() {
    let dog = Dog;
    dog.speak();  // 输出 "Woof!"

    let cat = Cat;
    cat.speak();  // 输出 "Meow!"
}

```

在上面的例子中，我们定义了一个 `Speak` trait，它有一个 `speak` 方法。然后，我们为 `Dog` 和 `Cat` 类型分别实现了这个 trait。这意味着我们可以对这两种类型的实例都调用 `speak` 方法，尽管它们的实现是不同的。
