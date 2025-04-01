use ch11_tests::add;
mod common;

#[test]
fn adder_works() {
    common::setup();

    let result = add(2, 2);
    assert_eq!(result, 4);
}
