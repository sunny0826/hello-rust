// Rust 的一个核心概念是所有权。这有助于 Rust 在没有垃圾收集的情况下管理内存。
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权被移动到 s2
    // println!("{}", s1); // 这里会报错，因为 s1 不再拥有数据的所有权

    let s3 = s2.clone(); // 创建 s2 的一个深拷贝，s2 仍然保持所有权
    println!("s2 = {}, s3 = {}", s2, s3);
}
