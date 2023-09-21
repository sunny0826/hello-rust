# 示例 4: 枚举和Option

```rust
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Click { x: i32, y: i32 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 10, y: 10 };

    inspect(pressed);
    inspect(click);
}
```

**知识点讲解**:

1. **枚举 (Enums)**: `enum` 关键字允许你定义一个类型，该类型可以有多种不同的值。在这个例子中，我们定义了一个名为 `WebEvent` 的枚举，它可以代表四种不同的事件。

2. **枚举的变体**: 每一个在 `enum` 中定义的值都被称为一个变体。变体可以是无数据的（如 `PageLoad` 和 `PageUnload`），也可以带有数据（如 `KeyPress` 和 `Click`）。

3. **模式匹配与枚举**: 我们再次使用了 `match` 表达式来处理不同的 `WebEvent` 变体。注意如何匹配带有数据的变体并使用这些数据。

4. **Option 枚举**: 虽然这个示例中没有直接展示 `Option`，但它是 Rust 的一个核心概念。`Option<T>` 是一个枚举，它可以有两个值：`Some(T)` 和 `None`。它用于表示一个值可能存在或可能不存在，这是 Rust 的方式来避免空指针异常。

枚举是 Rust 中的另一个强大的构建块，它们允许你创建可以有多种不同值的类型，并为这些值定义相关的行为。

当然可以！

## Option 枚举

```rust
fn find_divisor(number: u32, divisor: u32) -> Option<u32> {
    if divisor == 0 {
        None
    } else if number % divisor == 0 {
        Some(divisor)
    } else {
        None
    }
}

fn main() {
    let number = 20;
    let divisor = 4;

    match find_divisor(number, divisor) {
        Some(d) => println!("{} is a divisor of {}", d, number),
        None => println!("{} is not a divisor of {}", divisor, number),
    }
}
```

**知识点讲解**:

1. **Option 枚举**: `Option<T>` 是 Rust 的标准库中的一个枚举，用于表示一个值可能存在 (`Some(T)`) 或可能不存在 (`None`)。这是 Rust 的方式来安全地处理可能的空值，而不是使用空指针。

2. **返回 Option**: 在 `find_divisor` 函数中，我们返回了 `Option<u32>`。这意味着函数可能返回一个 `u32` 值（包装在 `Some` 中），或者可能不返回任何值（`None`）。

3. **使用 Option**: 在 `main` 函数中，我们使用了 `match` 表达式来处理 `find_divisor` 函数的返回值。这确保了我们必须处理 `Some` 和 `None` 两种情况，从而增加了代码的安全性。

`Option` 枚举是 Rust 中处理可能的空值的推荐方式，它避免了许多其他语言中常见的空指针异常问题。
