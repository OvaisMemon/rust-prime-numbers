use std::io;

fn main(){
    
    let mut input = String::new(); // variable declaration
    io::stdin().read_line(&mut input).expect("Invalid input."); // reading input from user
    
    // parsing the input from string to integer, i.e. u32
    let input_int:u32 = match input.trim().parse(){
        Ok(input) => input,
        Err(_) => panic!("Please input proper integer value.")
    };

    generate_prime_numbers(input_int); // calling function
}

fn generate_prime_numbers(num:u32){
    let range = num;
    
    println!("--- PRIME NUMBERS ARE ---");

    for elem in 2..range{
        let mut counter = 2;

        loop{

            if elem % counter == 0 && elem != counter {                
                break;
            }
            
            counter += 1;
                        
            if elem <= counter{                
                println!("{}", elem);
                break;
            }
        };       
    }
}