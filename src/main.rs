use std::io;
use std::{i32};

fn selection(){
    println!("_____________________");
    println!("Input 0 For Addition");
    println!("Input 1 For Subtraction");
    println!("Input 2 For Multiplication");
    println!("Input 3 For Division");
    println!("Input 4 For Mod");
    println!("Input 5 for Exponents");
    println!("Input 6 to calculate the slope of 2 points");
    println!("Input 99 for help");
    println!("_____________________");

}


fn addition(a: i128,b: i128 ) {
    println!("You have choosen Addition");
    println!("_____________________");
    println!("Your Number is {}", a + b);
}

fn subtraction(a: i128, b: i128){
    println!("You Have Choosen Subtraction");
    println!("_____________________");
    println!("Your Number is {}", a - b);
}
fn multiplication(a: i128, b: i128){
    println!("You Have Choosen Multiplication");
    println!("_____________________");
    println!("Your Number is {}", a * b);
}
fn division(a: i128, b:i128) {
    println!("You Have Choosen Division");
    println!("_____________________");
    let is_whole = a/b;

    if is_whole == 0 {
        println!("{}",a);
        println!("-");
        println!("{}",b);
    }
    else{
        println!("Your Number is {}", a / b);
    }
    

}
fn modulo(a: i128, b: i128){
    println!("You Have Choosen Mod");
    println!("_____________________");
    println!("Your Number is {}", a % b);
}
fn exponent(a: i128, b: i128){
    println!("You have choosen Exponents");
    println!("NOTE: Since this rust program uses u128 numbers, there is a limit of 3.40282367e38 - 1, which is 340282366920938463463374607431768211456");
    println!("I could switch to u128, but thats another project for another time");
    println!("_____________________");
        
    println!("We will use your A value as the base");
    println!("Your base value is: {}",a);
    println!("We are going to use your B value as the exponent");
    println!("Your exponent value is: {}",b);
    println!("_____________________");


    let base = u128::pow(a.try_into().unwrap(),b.try_into().unwrap());

    println!("{}",base)


}
fn slope(){
    println!("You Have choosen Slope");
    println!("_____________________");
       
        
    let mut x1 = String::new();

    let mut x2 = String::new();

    let mut y1 = String::new();

    let mut y2 = String::new();


    println!("Please Input Your x1 value");
    io::stdin()
    .read_line(&mut x1)
    .expect("Failed to read line");
    let x1: i32 = x1.trim().parse().ok().expect("Input not an integer");
    println!("_____________________");

    println!("Please Input your y1 value");
    io::stdin()
    .read_line(&mut y1)
    .expect("Failed to read line");
    let y1: i32 = y1.trim().parse().ok().expect("Input not an integer");
    println!("_____________________");

    println!("Please Input your x2 value");
    io::stdin()
    .read_line(&mut x2)
    .expect("Failed to read line");
    let x2: i32 = x2.trim().parse().ok().expect("Input not an integer");
    println!("_____________________");

    println!("Please Input your y2 value");
    io::stdin()
    .read_line(&mut y2)
    .expect("Failed to read line");
    let y2: i32 = y2.trim().parse().ok().expect("Input not an integer");
    println!("_____________________");

    let x3 = x2 - x1;

    let y3 = y2 - y1;

    if x3 == 0 || y3 == 0{
        println!("_____________________");
        println!("The slope is 0");
        println!("The program will abort if the quotent will be 0, this way we don't fail to divide by 0");
        println!("_____________________");
        std::process::abort();
    }
 
    let final_slope = y3 / x3;

    if final_slope == 0{
        println!("_____________________");
       println!("Your slope is:");
       println!("{}",y3);
       println!("______");
       println!("{}",x3);
       println!("_____________________");

    } else {
        println!("_____________________");
        println!("Your Slope is {}", final_slope);
        println!("_____________________");
    }


}
fn help(){
    println!("_____________________");
    println!("This is the help page");
    println!("_____________________");
}

fn main() {
    let mut calc_option = String::new();
    
    let mut a = String::new();

    let mut b = String::new(); 


    println!("_____________________");
    println!("Input Your First Number");
    println!("_____________________");

    io::stdin()
    .read_line(&mut a)
    .expect("Failed to read line");

    let a: i128 = a.trim().parse().ok().expect("Input not an integer");

    println!("_____________________");
    println!("Input Your Second Number");
    println!("_____________________");

    io::stdin()
    .read_line(&mut b)
    .expect("Failed to read line");
    let b: i128 = b.trim().parse().ok().expect("Input not an integer");

    selection();
    
     io::stdin()
    .read_line(&mut calc_option)
    .expect("Failed to read line");

    let x: i32 = calc_option.trim().parse().expect("Input not an integer");
    if x == 0{
        addition(a,b);
     }

    if x == 1{
        subtraction(a,b);
    }

    if x == 2{
        multiplication(a,b);
    }

    if x == 3{
        division(a,b);
    }

    if x == 4{
        modulo(a,b);
    }

    if x == 5{
        exponent(a,b);
    }

    if x == 6 {
        slope();
    }
    if x == 99 {
        help();
    }

}