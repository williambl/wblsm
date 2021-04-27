use crate::stack::Stack;

mod stack;

fn main() {
    let mut stack: Stack = Stack::new();
    stack.push(100);
    if let Some(x) = stack.peek() {
        println!("{}", x)
    }
    if let Some(x) = stack.pop() {
        println!("{}", x)
    }
    for i in 0..10 {
        stack.push(i);
    }
    println!("{}", stack.len());
    while !stack.is_empty() {
        if let Some(x) = stack.pop() {
            println!("{}", x)
        }
    }
    println!("{}", stack.len());
}
