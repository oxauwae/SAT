use std::io;
fn main() {
    //前文说std::io只是在字符串后面新加东西，现在我来试试：D 
    println!("输入值");
    let mut guess = String::new();
    guess = "abc".to_string(); //怎么要加to_String :(
    println!("{guess}");
    io::stdin()
        .read_line(&mut guess)
        .expect("错误");
    println!("这是新增以后的{}",guess);
    //看起来是成功能跑了，但是还是有一些报错，以后的区域以后再来探索吧:D 
}
