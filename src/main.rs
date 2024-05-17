use std::{io};
use rand::{Rng};

fn get_int_from_user(text:String) -> u32 {
    let mut number = String::new();
    println!("{}", text);
    io::stdin().read_line(&mut number).expect("Fail Read Line :\\");
    let number:u32 = number.trim().parse().expect("Cant Convert To Int :\\");
    number
}
fn main(){
    println!("\nWhat if all the people roll the dice??? \n");
    let _min:u128 = 0;
    let _max = get_int_from_user(String::from("Please Enter maxmimum people: ") ) as u128;
    println!("");
    let mut sum = 0;
    let length = (_max + 1) - _min;
    let mut r = rand::thread_rng();
    for i in _min..(_max + 1)  {
        let tmp = r.gen_range(1..=6);
        if (i % 100) == 0 && i != _max{
            println!("We crossed the border of {} people", i);
        }else if i == _max {
            println!("We reached {} people",i)
        }
        sum += tmp
    }
    println!("\nAverage is {}", (sum as f64 / length as f64) );
    // println!("Sum is {} & Length is {}", sum ,length)
}
