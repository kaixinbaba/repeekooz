use bytes::{BytesMut, BufMut, Buf};

fn main() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.put_i8(1);
    bytes_mut.put_i32(1);
    println!("{:?}", bytes_mut.len());
    println!("{:?}", bytes_mut.capacity());
    println!("{:?}", bytes_mut);

    let len = bytes_mut.len();
    let mut bytes_mut2 = BytesMut::with_capacity(4 + len);
    bytes_mut2.put_i32(len as i32);
    bytes_mut2.extend(bytes_mut);
    println!("{:?}", bytes_mut2.to_vec());
    println!("{:?}", bytes_mut2.len());
}