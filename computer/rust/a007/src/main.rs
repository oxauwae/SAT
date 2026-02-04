use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    //生成一个随机数
    //定义一个不可变的随机函数
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //获取用户的输入
    println!("输入您的猜测！");
    //定义一个可变函数 
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("错误！");
    //打印用户的猜测
    println!("您的猜测是{guess}");
    //接下来把String类型转变成数字类型
    let guess: u32 = guess.trim().parse().expect("转变错误！");  //这里的.expect
                                                                 //应该是对错误后的处理
    //这里新建了一个guess的函数将之前的guess遮蔽。
    //将新变量绑到guess.trim().parse()表达式上。表达式中的guess指的是包含输入的字符串类型guess变量。
    //String实例的trim方法会去除字符串开头和结尾的空白字符，我们必须执行这个方法在能将String类型和u32比较，因为用户必须用enter键才能将read_line返回并输入他们的猜想，这会在字符串里新增一个换行（newline）符，trim方法会消除\n或者\r\n ，\n代表“换行”；\r代表“回车”
    //字符串的parse方法将字符串转换成其他类型。这里用它来把字符串击换为数值。
    //guess后面的:告诉Rust我们指定了变量的类型。Rust有一些内建的数字类型；u32是一个无符号的32位整型。符于不大的正整数来说，它是不错的默认类型。
    //parse方法只在字符逻辑上可以转换为数字的时候才能工作，所以非常容易出错。
    //对比guess和secret_number的大小
    match guess.cmp(&secret_number){
        Ordering::Less => println!("太小了！"),
        Ordering::Equal => println!("你赢了！"),
        Ordering::Greater => println!("太大了！"),  //这里都要用,结尾，而不是;
        //cmp方法对比了两个数的大小，然后返回一个Ordering的枚举成员，就是Less,Equal,Greater中的一个
        //调用一个match，根据对guess和secret_number调用cmp返回的Ordering成员来决定下一步做什么
        //一个match表达式由分支构成，一个分支包含一个模式和表达式开头的值与分支模式相匹yvi时应执行的代码。
        //Rust获取提供给match的值并挨个检查每个分支的模式。
        //？是不是说当一个随机数的范围是(1..=100)，但是出现在后50个数的几率更大时，应该把Greater成员放在最上面，把Equal成员放在最下面，这样最节约性能？但是我不确定它是从下查还是从上查的，问一下kimi
        //应该是对的，以后有机会可以用到(KIMI说从上到下检查)
        //不管了，往下学！
    }
    

}
