pub fn compare_string(x: &str, y: &str) -> bool {
    let mxlen = if x.len() > y.len() {
        x.len()
    } else {
        y.len()
    };
    for i in 0..mxlen {
        if y.chars().nth(i) == None {
            return true;
        }
        if x.chars().nth(i) == None {
            break;
        }
        if x.chars().nth(i).unwrap() > y.chars().nth(i).unwrap() {
            return true;
        }
    }
    false
}