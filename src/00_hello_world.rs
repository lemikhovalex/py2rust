
/// docstring for a function
fn multiply(x: i64, y: i64) -> i64 {
    // one line comment
    x * y
    /* multi line comment
     * with 2 lines
     */
}

fn main() {
    let z = multiply(5, 6);
    println!("result {}", z);
    let txt = format!("result: {z}", z=z);
    println!("{}", txt);
}