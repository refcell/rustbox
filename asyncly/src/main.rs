#![allow(dead_code)]

use std::{future::Future, fs::read_to_string};

async fn foo1() -> u32 {
    println!("foo1");
    42
}

fn foo2() -> impl Future<Output = u32> {
    async {
        println!("foo1");
        let fut = read_to_string("file1").unwrap();

        // println!("foo2");
        // println!("foo1");
        // read_to_string("file2").await;
        // println!("foo2");
        // println!("foo1");
        // read_to_string("file3").await;
        // println!("foo2");
        // println!("foo1");
        // read_to_string("file4").await;
        // println!("foo2");
        42
    }
}

fn main() {
    println!("Hello, world!");

    let _x = foo1();
    // println!("Foo1: {:?}", x);
}
