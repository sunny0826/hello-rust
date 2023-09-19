# match

`match` 是 Rust 中的一个核心特性，它允许你对值进行模式匹配并根据匹配的模式执行相应的代码。以下是关于 `match` 的详细介绍：

## 1. 基本用法

`match` 表达式接受一个值，并尝试将其匹配到一系列模式中。每个模式后面都跟着一个 `=>`，然后是要执行的代码块。

```rust
let number = 5;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    4 => println!("Four"),
    5 => println!("Five"),
    _ => println!("Other"),
}
```

在上面的例子中，`number` 与模式 `5` 匹配，所以输出是 "Five"。

## 2. 使用模式

你可以使用各种模式进行匹配，包括：

- **字面值**：如上面的例子所示。
  
- **变量模式**：可以匹配任何值并将其绑定到变量。
  
- **通配符模式**：使用 `_`，它匹配任何值但不绑定它。

- **解构模式**：可以解构数组、元组或结构体。

```rust
let pair = (2, -2);

match pair {
    (x, y) if x == y => println!("These are twins"),
    (x, y) if x + y == 0 => println!("These are opposites"),
    (x, _) if x % 2 == 0 => println!("The first one is even"),
    _ => println!("No correlation..."),
}
```

- **范围模式**：匹配一个范围内的值。

```rust
match number {
    1..=5 => println!("It's between 1 and 5"),
    6..=10 => println!("It's between 6 and 10"),
    _ => println!("It's something else"),
}
```

## 3. 使用守卫

你可以在模式后面加上 `if` 条件来添加一个额外的守卫。只有当守卫为 `true` 时，模式才会匹配。

```rust
let pair = (2, 5);

match pair {
    (x, y) if x + y == 0 => println!("These numbers add up to zero!"),
    (x, y) if x % 2 == 0 => println!("The first number is even"),
    _ => println!("No special conditions..."),
}
```

## 4. 匹配枚举和结构体

`match` 也可以用于匹配枚举和结构体的值。

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

let result: Result<i32, String> = Result::Ok(5);

match result {
    Result::Ok(value) => println!("Got a value: {}", value),
    Result::Err(error) => println!("Got an error: {}", error),
}
```

## 5. 覆盖性

Rust 要求 `match` 表达式的模式是覆盖的，这意味着所有可能的值都必须被考虑。这确保了模式匹配的完整性和安全性。

## 总结

`match` 是 Rust 中的强大工具，它提供了一种简洁、安全和直观的方式来处理各种情况。通过模式匹配，你可以轻松地处理复杂的数据结构，如枚举和结构体，并确保代码的完整性和安全性。
