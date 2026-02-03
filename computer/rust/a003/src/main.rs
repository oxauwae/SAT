fn main() {
    let mut g = String::new();
    std::io::stdin()
        .read_line(&mut g)  //这里我写的时候多了一个; 现在没了
        .expect("abc");
    println!("{g}");
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}",y + 2);// x = 5 and y + 2 = 12
    //当打印变量的值的时候，变量名可以写进大括号中。当打印表达式的执行结果时，格式化字符串中的大括号中留空，格式化字符串后跟逗号分隔的需要打印的表达式列表
    println!("x = {} and y + 2 = {}",x,y + 3);
}
