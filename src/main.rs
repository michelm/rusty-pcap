fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        println!("args: {:?}", args);
    }

    let left = 34;
    let right = 27;
    let sum = rust_github_template::add(left, right);

    println!("{} + {} = {}", left, right, sum);
}
