#[derive(Debug)]
struct SomeStruct{
    num: i32
}

fn do_some_stuff(some_struct: &SomeStruct) {
    println!("{:?}", some_struct);
}

fn main() {
    let some_struct: SomeStruct = SomeStruct{num: 10};
    do_some_stuff(&some_struct);
    do_some_stuff(&some_struct);
}
