use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() { 
    //生成一个随机数
    let mut a1 = rand::thread_rng().gen_range(1..=100);
    println!("a1:{a1}");
    let a2 = rand::thread_rng().gen_range(1..=100);
    println!("a2:{a2}");
    let a3 = rand::thread_rng().gen_range(1..=2); 
    println!("a3:{a3}");
    loop {
        match a3.cmp(&1){
            Ordering::Equal => {
                break;
            }
            Ordering::Greater => {
                a1 = a2;
                break;
            }
            Ordering::Less => {
                break;
            }
        }
    }
    println!("{a1}");
}
