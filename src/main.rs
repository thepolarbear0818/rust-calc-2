use std::io;
use std::{i32};



fn main() {
    let mut calc_option = String::new();
    
    let mut a = String::new();

    let mut b = String::new(); 


    println!("Input Your First Number");

    io::stdin()
    .read_line(&mut a)
    .expect("Failed to read line");

    let a: i32 = a.trim().parse().ok().expect("Input not an integer");

    println!("Input Your Second Number");

    io::stdin()
    .read_line(&mut b)
    .expect("Failed to read line");
    let b: i32 = b.trim().parse().ok().expect("Input not an integer");

    println!("Input 0 For Addition");
    println!("Input 1 For Subtraction");
    println!("Input 2 For Multiplication");
    println!("Input 3 For Division");
    println!("Input 4 For Mod");
    println!("Input 5 for Exponents");
    println!("Input 6 to calculate the slope of 2 points");



    io::stdin()
    .read_line(&mut calc_option)
    .expect("Failed to read line");

    let x: i32 = calc_option.trim().parse().expect("Input not an integer");
    if x == 0{
       println!("You have choosen Addition");

       println!("Your Number is {}", a + b);
        
    }

    if x == 1{
        println!("You Have Choosen Subtraction");
        println!("Your Number is {}", a - b);
    }

    if x == 2{
        println!("You Have Choosen Multiplication");
        println!("Your Number is {}", a * b);
    }

    if x == 3{
        println!("You Have Choosen Division");
        println!("If the number is a fraction or a decimal, This program will return 0");
        println!("Your Number is {}", a / b);
    }

    if x == 4{
        println!("You Have Choosen Mod");
        println!("Your Number is {}", a % b);
    }

    if x == 5{
        println!("You have choosen Exponents");
        
        let mut base = String::new();
        let mut exponent = String::new();

        println!("Please Input Your Base value");
        io::stdin()
        .read_line(&mut base)
        .expect("Failed to read line");
        let base: i32 = base.trim().parse().ok().expect("Input not an integer");

        println!("Please Input Your Exponent value");
        io::stdin()
        .read_line(&mut exponent)
        .expect("Failed to read line");
        let exponent: i32 = exponent.trim().parse().ok().expect("Input not an integer");

        let base = i32::pow(base,exponent.try_into().unwrap());

        println!("{}",base)


    }

    if x == 6 {
        println!("You Have choosen Slope");
       
        
        let mut x1 = String::new();

        let mut x2 = String::new();
    
        let mut y1 = String::new();
    
        let mut y2 = String::new();


        println!("Please Input Your x1 value");
        io::stdin()
        .read_line(&mut x1)
        .expect("Failed to read line");
        let x1: i32 = x1.trim().parse().ok().expect("Input not an integer");

        println!("Please Input your y1 value");
        io::stdin()
        .read_line(&mut y1)
        .expect("Failed to read line");
        let y1: i32 = y1.trim().parse().ok().expect("Input not an integer");

        println!("Please Input your x2 value");
        io::stdin()
        .read_line(&mut x2)
        .expect("Failed to read line");
        let x2: i32 = x2.trim().parse().ok().expect("Input not an integer");

        println!("Please Input your y2 value");
        io::stdin()
        .read_line(&mut y2)
        .expect("Failed to read line");
        let y2: i32 = y2.trim().parse().ok().expect("Input not an integer");

        let x3 = x2 - x1;

        let y3 = y2 - y1;

        if x3 == 0 || y3 == 0{
            println!("The slope is 0");
            println!("The program will abort if the quotent will be 0, this way we don't fail to divide by 0");
            std::process::abort();
        }
     
        let final_slope = y3 / x3;

        if final_slope == 0{
           println!("Your slope is:");
           println!("{}",y3);
           println!("______");
           println!("{}",x3);

        } else {
            println!("Your Slope is {}", final_slope);
        }


    }

}