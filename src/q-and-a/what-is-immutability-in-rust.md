# What is "immutability" in Rust?

> 请描述 Rust 中的“不可变性”是什么？它如何帮助确保代码的安全性和可预测性？另外，能否给出一个简单的示例来说明不可变性的重要性？

在 Rust 中，不可变性是默认的。这意味着一旦你创建了一个变量并为其赋值，你就不能更改它，除非你明确地标记它为 `mut`（可变的）。

1. **为什么不可变性很重要**：
   - **安全性**：不可变性可以防止意外地修改数据，这可以减少 bugs 和意外的副作用。
   - **并发**：不可变性使得在并发环境中共享数据变得更加简单和安全，因为你知道数据不会被另一个线程修改。
   - **可预测性**：代码更容易理解和预测，因为你知道哪些数据可以更改，哪些数据不能更改。

2. **示例**：

   ```rust
   let x = 5;
   x = 6; // 这会引发编译错误，因为 x 是不可变的
   ```

   如果你想修改 `x`，你需要这样做：

   ```rust
   let mut x = 5;
   x = 6; // 这是有效的，因为 x 是可变的
   ```

总的来说，Rust 中的不可变性是一种强大的工具，可以帮助你编写更安全、更可预测的代码。它鼓励你思考哪些数据应该是可变的，哪些数据应该是不可变的，从而使你的程序更加健壮。