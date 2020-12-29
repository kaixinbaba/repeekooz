fn main() {
    let ss = "127.0.0.1:2181,127.0.0.2:2182/";
    let x: Vec<&str> = ss.split("/").collect();
    println!("{:?} len: {}", x, x.len());

    let i = 354234;
    println!("{}", i as u8);
}