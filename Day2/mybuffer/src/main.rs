mod mybuffer;

fn main() {
    let b = vec!(1, 2, 3, 4, 5, 6, 7);
    let a = mybuffer::Buffer::new(b);
    println!("{}", a.sum());
}
