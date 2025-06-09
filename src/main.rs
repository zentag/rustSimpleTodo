use::std::io;

fn main() {
    let mut todo = String::new();
    io::stdin().read_line(&mut todo).expect("Couldn't read stdin");
    println!("back to start");
}
