mod compstr;

fn main() {
    let a : &str = "123123123";
    let b : &str = "1231231233";
    if compstr::compare_string(a, b) {
        println!("Great!");
    } else {
        println!("No!");
    }
}
