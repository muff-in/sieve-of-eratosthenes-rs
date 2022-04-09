use std::io;
use std::cmp::Ordering;
fn main() {
    println!("Input a integer: ");


    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: u32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a valid integer");
            return;
        }
    };

    let non_prime_number = 1;

    match user_input.cmp(&non_prime_number) {
        Ordering::Less => println!("Enter number greater than 1"),
        Ordering::Equal => println!("Enter number greater than 1"),
        Ordering::Greater => println!("All Prime Numbers till {} are \n{:?}", user_input, sieve_algorithm(user_input)),
    }
}



    fn sieve_algorithm(user_input: u32) -> Vec<u32>
    {

        let mut bool_vec: Vec<bool> = Vec::new();

        let mut output_vec: Vec<u32> = Vec::new();

         for _i in 0..= user_input {
            bool_vec.push(true);
        }
        
          let mut p = 2;
        while p * p <= user_input {
            if bool_vec[p as usize] == true {
               
                let mut mp = p * 2;
                while mp <= user_input {
                    bool_vec[mp as usize] = false;
                    mp += p;
                }
            }
            p += 1;
        }

        
        let mut i = 2;
        while i <= user_input {
            if bool_vec[i as usize] == true {
                output_vec.push(i);
            }
            i += 1;
        }

        output_vec
}




