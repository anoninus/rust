pub fn logic_greet(name: Option<String>) {
    match name {
        Some(n) => println!("Ledgerly greets {n}"),
        None => println!("Hello, from Ledgerly!\n\nUsage:\nledgerly help"),
    }
}
