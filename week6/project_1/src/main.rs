use std::io;

fn main() {



    println!("======MENU=======");
    println!(" P -> Poundo Yam/Edinkaiko Soup : #3,200 ");
    println!(" F -> Fried Rice & Chicken : #3,000 ");
    println!("A -> Amala & Ewedu Soup : #2,500");
    println!(" E -> Eba & Egusi Soup : #2,000");
    println!(" W -> White Rice & stew :#2,500 ");

   println!("Enter Food Code (P,F.A,E,W) ");
    let mut food_code =String::new();
    io::stdin().read_line(&mut food_code).expect("failed to read");



let p = 3200;

if food_code.trim() == "P"{
    println!("amount is #3200");
}
else if food_code.trim() == "F"{
    println!("amount is #3000");
}
else if food_code.trim() == "A"{
    println!("amount is #2500");

}
else if food_code.trim() == "E"{
    print!("amount is #2000");
}
else if food_code.trim() == "W"{
    print!("amount is #2500");
}
else{
    println!("invalid FOOD CODE");
};

println!("Enter quantity");
let mut quantity = String::new();
io::stdin().read_line(&mut quantity).expect("failed to read");

}
