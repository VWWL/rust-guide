use std::io::Error;
use learn_rust::day6::bird::Bird;
use learn_rust::day6::bird_can_fly::bird_can_fly;
use learn_rust::day6::create_bird::create_bird;
use learn_rust::day6::direct_panic::direct_panic;
use learn_rust::day6::fly_things::fly_things;
use learn_rust::day6::fly_things_2::fly_things_2;
use learn_rust::day6::largest::largest;
use learn_rust::day6::longest::longest;
use learn_rust::day6::pair::Pair;
use learn_rust::day6::read_username_from_file::read_username_from_file;
use learn_rust::day6::recoverable_panic::recoverable_panic;
use learn_rust::day6::unimplemented::unimplemented;
use learn_rust::day6::unreachable::unreachable;

#[test]
#[should_panic(expected = "Crash and burn!")]
pub fn should_panic_directly() {
    direct_panic();
}

#[test]
pub fn should_recoverable_panic_be_caught() {
    assert_eq!(recoverable_panic(), "Error");
}

#[test]
#[should_panic(expected = "No such file or directory")]
pub fn should_panic_directly_with_question_mark() {
    let a = read_username_from_file();
    match a {
        Err(e) => panic!("{}", e),
        _ => {}
    }
}

#[test]
#[should_panic(expected = "not implemented")]
pub fn unimplemented_method() {
    unimplemented();
}

#[test]
#[should_panic(expected = "")]
pub fn unreachable_method() {
    unreachable();
}

#[test]
pub fn should_get_max_in_int() {
    assert_eq!(largest(&vec![34, 50, 25, 100, 65]), 100);
}

#[test]
pub fn should_get_max_in_double() {
    assert_eq!(largest(&vec![34.2, 50.0, 25.3, 100.9, 65.6]), 100.9);
}

#[test]
pub fn should_get_max_in_char() {
    assert_eq!(largest(&vec!['a', 'b', 'c', 'f', 'g']), 'g');
}

#[test]
pub fn bird_with_big_wing_can_fly() {
    assert_eq!(bird_can_fly("big"), "Bird can fly with big wings.");
}

#[test]
pub fn should_something_can_fly() {
    assert_eq!(fly_things(&Bird::new("big")), "Bird can fly with big wings.");
}

#[test]
pub fn should_something_can_fly_which_arg_is_trait_bound() {
    assert_eq!(fly_things_2(&Bird::new("big")), "Bird can fly with big wings.");
}

#[test]
pub fn should_create_fly_things_with_wing() {
    assert_eq!(create_bird("big").fly(), "Bird can fly with big wings.");
}

#[test]
pub fn should_show_biggest_in_pair() {
    assert_eq!(Pair::new(1, 2).cmp_display(), &2);
}

#[test]
pub fn find_longest() {
    assert_eq!(longest("12", "123"), "123");
}