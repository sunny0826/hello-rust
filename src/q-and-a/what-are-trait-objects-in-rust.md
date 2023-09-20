# What are "Trait Objects" in Rust?

> 请描述 Rust 中的“trait 对象”是什么？它们与普通的 trait 有何不同？并请给出一个简单的使用示例。

在 Rust 中，`trait` 定义了一组方法，可以被多种类型实现。而“trait 对象”则是一种使用 trait 的方式，它允许你使用动态分发来调用 trait 的方法。

1. **Trait 对象的定义**：
   当你将 trait 用作一个类型的时候，你实际上是在使用“trait 对象”。这意味着你不知道实现该 trait 的具体类型，只知道某个类型实现了这个 trait。

2. **与普通 trait 的区别**：
   - **静态分发 vs 动态分发**：普通的 trait 使用静态分发，这意味着编译器在编译时知道调用哪个方法。而 trait 对象使用动态分发，这意味着在运行时决定调用哪个方法。
   - **性能**：由于静态分发在编译时确定，它通常比动态分发更快。
   - **灵活性**：trait 对象更加灵活，因为它们允许你在运行时处理不同的类型。

3. **示例**：

   ```rust
   trait Animal {
       fn speak(&self);
   }

   struct Dog;
   impl Animal for Dog {
       fn speak(&self) {
           println!("Woof!");
       }
   }

   struct Cat;
   impl Animal for Cat {
       fn speak(&self) {
           println!("Meow!");
       }
   }

   fn animal_speak(animal: &dyn Animal) {
       animal.speak();
   }

   fn main() {
       let dog = Dog;
       let cat = Cat;

       animal_speak(&dog);  // 输出 "Woof!"
       animal_speak(&cat);  // 输出 "Meow!"
   }
   ```

   在上面的示例中，我们定义了一个 `Animal` trait 和两个实现了该 trait 的结构体：`Dog` 和 `Cat`。然后，我们定义了一个 `animal_speak` 函数，它接受一个 trait 对象 `&dyn Animal` 作为参数，并调用其 `speak` 方法。
