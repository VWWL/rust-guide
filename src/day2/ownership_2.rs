pub fn ownership_2() -> i32 {
    let x = 2;
    let _y = x;
    // x is still usable.
    x
}
