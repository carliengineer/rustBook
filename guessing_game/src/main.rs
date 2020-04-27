use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess!");

    let mut guess = String::new();
    //rut def format for numbers is i32
    let secret_number = rand::thread_rng().gen_range(1, 10);
    println!("{}", secret_number);    
    
    // let guess_array : [i32 ; 5] = [1, 2, 3, 4, 5];
    // let an_array : [3 ; 5]; // => [3, 3, 3, 3, 3];

    /*let y = {
        let x = 3;
        x +1 
    }; here y evaluates to 4
    */ 
    // if you add a semicolon to the end f an expression, you turn it into a statement, which will then not return a value.



    //Result types are enumerations "enums", so it means they can have fixed set of values that are called "variants".
    //For reult enum, variants are "Err" or "Ok".
    //You should use Result return value, otherwise it gives warnings
    //expect func of Result runs only when returned result instance is "Err"
    loop {
      //  let mut guess = String::new();
        //If the guess is defined outside of the loop, we can't keep owerwriting it a new value on top of it, and it doesn't work.
        guess.truncate(0);
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
        
        let guess : u32 = match dbg!(&guess).trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }





}
