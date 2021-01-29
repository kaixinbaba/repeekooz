

fn main() {
    let mut v1 = Vec::new();
    v1.push("123".to_string());
    v1.push("12aa4".to_string());
    v1.push("12dfs".to_string());
    println!("{:?}", v1);
    let mut v2: Vec<&String> = Vec::new();
    for vv1 in v1.iter() {
        v2.push(vv1);
    }
    println!("{:?}", v1);
    println!("{:?}", v2);
}
