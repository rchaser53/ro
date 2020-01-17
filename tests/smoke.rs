#[ro::hello]
fn wrapped_function(a: u64) {}

#[ro::hello]
fn wrapped_function2(a: u32) {}

#[test]
fn works() {
    assert_eq!(42 + 3, wrapped_function(3));
    assert_eq!(42 + 1, wrapped_function2(1));
}
