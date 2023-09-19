# How Threads Work in Rust

Rust 的并发模型基于其核心概念：所有权、借用和生命周期。这些规则在编译时确保了并发安全性，从而避免了数据竞争和其他并发相关的错误。

1. **所有权和借用**：Rust 的所有权系统确保在任何给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。这避免了数据竞争。
2. **线程**：Rust 的标准库提供了基于操作系统的线程实现，允许你创建新的线程并并行执行代码。
3. **Mutex**：`Mutex` 是一个互斥锁，它允许多个线程共享和修改数据。当一个线程想要访问数据时，它必须首先获得锁。如果另一个线程已经拥有锁，那么该线程将被阻塞，直到锁被释放。
    
    示例：
    
    ```rust
    use std::sync::Mutex;
    
    let m = Mutex::new(5);
    
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    
    ```
    
4. **Arc**：`Arc` 是一个原子引用计数类型。它允许多个线程安全地共享数据的所有权。当你需要在多个线程之间共享数据，并确保数据在最后一个线程完成后被清理时，`Arc` 是非常有用的。
    
    示例：
    
    ```rust
    use std::sync::Arc;
    
    let foo = Arc::new(vec![1.0, 2.0, 3.0]);
    
    let a = foo.clone();
    let b = foo.clone();
    
    // 两个线程可以同时访问 foo 的数据
    
    ```
    

结合 `Mutex` 和 `Arc` 可以在多个线程之间安全地共享和修改数据。

## Example

当你想在多个线程之间共享可变数据时，`Arc` 和 `Mutex` 经常一起使用。`Arc` 确保数据在多个线程之间安全地共享，而 `Mutex` 确保数据的互斥访问。

下面是一个简单的示例，其中多个线程增加一个共享的计数器：

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 使用 Arc 和 Mutex 包装一个简单的 i32 计数器
    let counter = Arc::new(Mutex::new(0));

    // 创建一个空的线程向量来存储线程句柄
    let mut handles = vec![];

    for _ in 0..10 {
        // 对 Arc 进行克隆，以便在新线程中使用
        let counter = Arc::clone(&counter);

        // 启动一个新线程
        let handle = thread::spawn(move || {
            // 锁定 Mutex
            let mut num = counter.lock().unwrap();

            // 修改数据
            *num += 1;
        });

        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 打印最终的计数器值
    println!("Result: {}", *counter.lock().unwrap());
}
```

在这个示例中，我们创建了 10 个线程，每个线程都增加了计数器的值。使用 `Mutex` 确保了每次只有一个线程可以修改计数器，而 `Arc` 允许我们在多个线程之间安全地共享计数器。

最后，我们应该看到 `"Result: 10"`，因为每个线程都增加了计数器的值。

---

详细解释下面这段代码

```rust
let handle = thread::spawn(move || {
    // 锁定 Mutex
    let mut num = counter.lock().unwrap();

    // 修改数据
    *num += 1;
});

```

1. **`thread::spawn`**:
这是 Rust 的标准库中的一个函数，用于创建并启动一个新的线程。它接受一个闭包作为参数，该闭包定义了线程应该执行的代码。
2. **`move` 关键字**:
这是一个 Rust 的闭包特性。使用 `move` 关键字意味着闭包将获取其环境中变量的所有权，而不是仅仅借用或引用它们。在这个例子中，我们使用 `move` 关键字确保 `counter`（一个 `Arc<Mutex<i32>>`）被移动到闭包内，这样新线程可以安全地访问和修改它。
3. **`counter.lock().unwrap()`**:
    - `counter.lock()`: 这会尝试获取 `counter`（一个 `Mutex`）的锁。如果锁当前未被其他线程持有，它会立即返回一个 `Result`，该 `Result` 包含锁定的数据。如果锁被其他线程持有，当前线程将被阻塞，直到锁变得可用。
    - `unwrap()`: 这是一个方法，用于处理上述 `Result`。如果 `lock()` 调用成功并返回 `Ok`，`unwrap()` 会返回锁定的数据。如果 `lock()` 调用失败并返回 `Err`（例如，因为死锁），`unwrap()` 会导致程序崩溃。在实际应用中，你可能希望更加优雅地处理这种错误，而不是简单地使用 `unwrap()`。
4. **`num += 1;`**:
这行代码增加了 `num` 的值，其中 `num` 是一个指向 `i32` 的可变引用。`` 是解引用操作符，它允许我们访问引用指向的实际数据。因此，这行代码实际上是增加了 `counter` 中的整数值。
5. **`let handle = ...`**:
`thread::spawn` 返回一个 `JoinHandle`，它是一个代表新线程的句柄。你可以使用这个句柄来等待线程完成，或查询线程的状态。在这个例子中，我们将这个句柄存储在 `handle` 变量中，以便稍后使用。
