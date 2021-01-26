use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert("1".to_string(), vec!["a".to_string()]);
    println!("{:?}", m);
    let mut v: Vec<String> = Vec::new();
    if let Some(s) = m.get_mut("1") {
        v.append(&mut s.clone());
    }
    println!("{:?}", v);
    println!("{:?}", m);
}
