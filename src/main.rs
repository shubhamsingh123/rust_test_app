fn main(){
   let x = 100;
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
