use std::io;

fn main(){

    println!("Enter a number : ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

//    let x = 100;

let x: u32 = input.trim().parse().expect("Please enter a valid number");
   let is_even = ans(x);

   if is_even{
    println!("{}" , "Congratulation it is an even number")
   }else{
    println!("{}" , "It is an odd number")
   }
}

fn ans(x:u32)->bool{
    return x%2==0;
}
