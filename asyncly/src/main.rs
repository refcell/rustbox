#![allow(dead_code)]

use std::future::Future;

async fn foo1() -> u32 {
    42
}

fn foo2() -> impl Future<Output = u32> {
    async {
        42
    }
}

fn main() {
    println!("Hello, world!");

    let x = foo1();
    println!("Foo1: {:?}", x);
}
