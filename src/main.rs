fn main() {
    println!("Hello, world!");
}

#[cfg(feature = "error")]
#[test]
fn error(){
    panic!();
}

#[cfg(not(feature = "error"))]
#[test]
fn error(){
    println!("ok");
}
