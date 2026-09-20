use std::io;

fn main() {

    println!("Enter value for a:");
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).expect("failed to read line");
    let a: f64 = input_a.trim().parse().expect("please enter a valid number");

    println!("Enter value for b:");
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("failed to read line");
    let b: f64 = input_b.trim().parse().expect("please input a valid number");


       println!("Enter value for c:");
       let mut input_c = String::new();
       io::stdin().read_line(&mut input_c).expect("failed to read line");
       let c: f64 = input_c.trim().parse().expect("please input a valid number");


      let d = b * b - 4.0 * a * c;
      println!("d is = {}", d);

      if d > 0.0 {
        println!("There are two distinct root");
      }
        else if d == 0.0 {
            println!("There are exactly one roots");
        }

        else{
            println!("There are no real roots ");
        }
      
}
