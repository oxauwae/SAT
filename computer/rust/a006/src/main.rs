use std::io;
use rand::Rng;
use std::cmp::Ordering;  //从标准库中引入了一个叫做std::cmp::Ordering的类型到作用域中。Ordering也是一个枚举，不过它的成员是Less,Greater和Equal。这是比较两个值时可能出现的三种结果。

fn main() {
    //我发现这玩意好像要加在刚刚的代码上：（
    //那再写一次吧，首先生成一个1~100的随机数
    
    let secret_number = rand::thread_rng().gen_range(1..=100);  //引用rand::thread_rng()这个随机数生成器，后面的.gen_range()可以看作是在修饰rand::thread_rng，而1..=100是一个闭区间的范围。
    //再者获取用户的输入
    //先定义一个可变函数
    let mut guess = String::new();
    //再用io::stdin()将用户输入的值加在mut guess后面
    io::stdin()
        .read_line(&mut guess)
        .expect("错误！");
    let guess: u32 = guess.trim().parse().expect("Please type a number");
    //这里用一个新的值guess来遮蔽之前的值，这个功能常用来将一个类型的值转换成另一个米型的值。
    println!("你的猜测是{guess}");
    match guess.cmp(&secret_number){
        //match用于对guess和secret_number调用cmp返回的ordering成员来决定接下来做什么。
        //用cmp方法来比较两个值并可以在任何可比较的值上调用。
        Ordering::Less => println!("太小了！"),
        Ordering::Equal => println!("你赢了！"),
        Ordering::Greater => println!("太大了！"),
    }
}
