mod two_sum;
use two_sum::two_sum;

fn main() {
    let input = [-1, -2, -3, -4, -5]; //[0, 4, 3, 0]; //[2, 7, 11, 15];
    let vec = input.to_vec();
    let target = -8;
    let res = two_sum(vec, target);

    println!("{:?}", res);
}
