use std::io;

pub fn menu() {
    loop {
        println!("Operations: ");
        println!("1. Add\n2. Subtract\n3. Multiply\n4. Divide\n5. Exit");
        println!("Enter choice: ");
        
        let mut ch = String::new();

        io::stdin().read_line(&mut ch).expect("Failed to read choice");
        println!("Your choice : {ch}");

        let a: i64;
        let b: i64;
        let res: i64;

        match ch.trim() {
            "1" => {
                (a, b) = input();
                res = a + b;
                println!("Result = {res}");
            }
            "2" => {
                (a, b) = input();
                res = a - b;
                println!("Result = {res}");
            }
            "3" => {
                (a, b) = input();
                res = a * b;
                println!("Result = {res}");
            }
            "4" => {
                (a, b) = input();
                if b==0 {
                    println!("Zero Division Error");
                } else {
                    res = a / b;
                    println!("Result = {res}");
                }
            }
            "5" => {
                println!("Exiting the program...");
                return;
            }
            _ => println!("Invalid choice!")
        }
    }
}

fn input() -> (i64, i64) {
    println!("Enter 2 numbers: ");

    let mut a = String::new();
    let mut b = String::new();
    io::stdin().read_line(&mut a).unwrap();
    io::stdin().read_line(&mut b).unwrap();

    let a: i64 = a.trim().parse().unwrap();
    let b: i64 = b.trim().parse().unwrap();

    (a, b)
}