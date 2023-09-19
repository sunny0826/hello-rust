use std::cell::RefCell;

fn main() {
    let c = RefCell::new(5);
    let m = c.borrow_mut();
    let n = c.borrow();  // 这会导致运行时错误
}
