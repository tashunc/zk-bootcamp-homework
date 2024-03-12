use std::ffi::c_uint;
use std::io;
use rand::Rng;

fn main() {
    println!("Please Enter Your Secret :");

    let mut secret = String::new();
    let mut rng = rand::thread_rng();


    match io::stdin().read_line(&mut secret) {
        // Read the secret value from the cmd
        Ok(_) => {

            let secret_parsed: u128 = secret.trim().parse()
                .expect("Please enter a valid unsigned integer");

            // generate the random coefficients along with the secret

            let (a0, a1, a2, a3, a4) = (secret_parsed, rng.gen(), rng.gen(), rng.gen(), rng.gen());

            // Let P  = 11 and k = 4
            // let n = 7

            for i in 0..5 {
                let x = rng.gen_range(0..=2_u128.pow(22));
                let y = generate_y(&a0, &a1, &a2, &a3, &a4, x);
                println!("{} {}", x, y);
            }
            // println!("secret is {:?}", random_number);
        }
        Err(error) => {
            eprintln!("Error: {}", error)
        }
    }
}
/*
 * Use to generate the redundant points
 *  /// y = a4 x ^ 4 + a3 x ^ 3 + a2 x ^ 2 + a1 x ^ 1 + a0
*/
fn generate_y(a0: &u128, a1: &u32, a2: &u32, a3: &u32, a4: &u32, x: u128) -> u128 {
    *a4 as u128 * x.pow(4) as u128 + *a3 as u128 * x.pow(3) as u128 + *a2 as u128 * x.pow(2) as u128 + *a1 as u128 * x as u128 + *a0 as u128
}
