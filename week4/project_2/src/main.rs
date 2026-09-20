use std::io;

fn main() {




    // input experience
    println!("Are you experienced? yes/no");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read line");



   if experience.trim().to_lowercase() == "yes" {
   println!("Enter your age" );
     let mut age = String::new();
   io::stdin().read_line(&mut age).expect("input valid age ");
   let age: i32 = age.trim().parse().expect("input not an integar");

if age >= 40 {
    println!("Your annual incentive is #1,560,000")
}
else if age > 30 && age <= 39 {
    println!("Your annual incentive is 1,480,000")    
}
else if age <28{
    println!("Your annual incentive is #1,300,000")
}






   
   }
   else if experience.trim().to_lowercase() == "no"{
    println!("Annual incentive is #100,000");
   }








}
