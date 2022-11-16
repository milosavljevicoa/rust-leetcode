pub fn reverse_integer(num: i32) -> i32 {
    let mut num_iter = num;
    let mut result = 0;

    while num_iter != 0 {
        result = result * 10;
        result = result + (num_iter % 10);
        num_iter = num_iter / 10;
    }

    result
}
