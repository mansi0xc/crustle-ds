
mod linear;
mod binary;

fn main() {
    println!("Hello, world!");
    let nums = [1, 2, 3, 4, 5];
    let result = binary::bsearch(&nums, 4);
    println!("Result = {result}");
}
