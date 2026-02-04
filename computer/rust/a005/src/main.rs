use std::io;
use rand::Rng; //引入一个rand::Rng
               //Rng是一个trait，它定义了随机数生成器应实现的方法
fn main() {
    println!("猜测一个数字！");
    let s_n = rand::thread_rng().gen_range(1..=100);  //随机生成一个在1..=100之间的数
                                                     //rand::thread_rng函数提供实际使用的随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取seed（什么是seed
                                                     //.gen_range调用了随机数生成器的gen_range方法，gen_range方法获取一个范围表达式作为参数，并生成一个在此范围之间的随机数
                                                     //A..=B，它对上下边界均为闭区间
                                                     //！！！这里写的时候报错了if this is
                                                     //intentional, prefix it with an
                                                     //underscore:"_sn"
                                                     //因为我没有应用到它，所以报错，如果是有意为之的，那么在其前面加_
//    println!("这个随机数是{s_n}");
    println!("请输入你猜测的数字！");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)  //将用户输入的值追加到mut guess字符串的后面
        .expect("错误！");
    println!("你的猜测是{guess}");
}
