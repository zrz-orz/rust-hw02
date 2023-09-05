fn main() {
    let init : Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let ans: Vec<char> = init.iter().map(|&x| char::from_u32(x as u32 + 1).unwrap()).collect();
    for letter in ans {
        print!("{} ", letter);
    }
}
