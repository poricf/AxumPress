# Learning Rust: Part 2

In Part 2 of my Rust journey, I dive deeper into ownership, borrowing, and lifetimes.

## Ownership and Borrowing
Rust’s ownership system ensures memory safety. At first, it’s confusing, but it prevents a lot of bugs!

- Each value has a single owner
- You can borrow values with references (`&T`)
- Mutable references (`&mut T`) let you change data, but only one at a time

## Lifetimes
Lifetimes tell Rust how long references are valid. It’s tricky, but the compiler helps you get it right.

## Next Steps
I’m planning to build a small CLI tool in Rust. More updates soon!
