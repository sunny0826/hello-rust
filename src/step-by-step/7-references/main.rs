// 引用允许你使用数据而不取得其所有权。
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s); // 使用引用传递，不取得所有权
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
