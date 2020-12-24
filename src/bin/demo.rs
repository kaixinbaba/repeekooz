

async fn foo() {
    println!("{}", 5);
}

fn main() {
    let a = async {
        foo().await;
    };
    async {
        a.await;
    };
    println!("Hello");
}