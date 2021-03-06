use rust_guide::day4::learn_option::learn_option;
use rust_guide::day4::learn_use_enum::learn_use_enum;
use rust_guide::day4::learn_use_enum_2::learn_use_enum_2;

#[test]
fn should_create_two_kinds_of_structs() {
    let (v4, v6) = learn_use_enum();
    assert_eq!(v4.address, "127.0.0.1");
    assert_eq!(v6.address, "::1");
}

#[test]
fn should_create_two_enum_with_arguments() {
    let (_v4, _v6) = learn_use_enum_2();
}

#[test]
fn should_create_two_option() {
    assert_eq!(learn_option().unwrap(), 5);
}
