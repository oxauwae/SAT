use std::cmp::Ordering;
use std::io;
//woc怎么不用导入rand了，直接用random_range，是自己带了吗???
fn main() {
    //生成一个随机数
    let secret_number: u32 = rand::random_range(1..=100);
    //循环获取用户输入
    loop {
        let mut guess = String::new();
        println!("输入");
        io::stdin().read_line(&mut guess).expect("错误1");
        println!("输入为{guess}");
        //要比大小
        //转为数字
        //不要直接崩溃，要处理无效字符
        //trim用于去除无效字符
        //parse判断是否为有效数字，返回Ok或Err，并带数字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //再判断大小
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Equal => {
                println!("你赢了");
                break;
            }
            Ordering::Greater => println!("太大了"),
        }
    }
}
