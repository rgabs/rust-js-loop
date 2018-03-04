#[no_mangle]
pub fn loop_runner(limit: i32) -> i32 {
    let mut x = 0;
    for _n in 0..limit {
        x = x + 2;
    }
    return x;
}