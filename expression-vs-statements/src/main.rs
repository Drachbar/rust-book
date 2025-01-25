fn main() {
    let y = {
        let mut x = 3;
        x = x + 1;
        x + 3
    };

    println!("The value of y is: {y}");
}
