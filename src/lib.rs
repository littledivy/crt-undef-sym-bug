extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

#[ctor::ctor]
unsafe fn init() {
    let _ = add(1, 2);
}
