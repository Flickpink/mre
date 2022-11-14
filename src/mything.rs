mod a;
mod b;

use std::ops::Index;

struct MyStruct {
    a: i32
}

pub trait SomeTrait {
    fn do_stuff(input: MyStruct) -> i32;
}

impl SomeTrait for MyStruct {
    fn do_stuff(input: MyStruct) -> i32 {
        input.a
    }
}