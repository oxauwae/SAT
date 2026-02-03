fn main() {
    let mut g = String::new();
    std::io::stdin()
        .read_line(&mut g)  //这里我写的时候多了一个; 现在没了
        .expect("abc");
    println!("{g}");
}
