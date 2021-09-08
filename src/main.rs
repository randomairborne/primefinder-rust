use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut primes: Vec<i128> = Vec::new();
    let mut current_number_being_checked = 1;
    let mut largest_number_wanted_input_vec: Vec<String> = env::args().collect();
    if largest_number_wanted_input_vec.len() != 2 {
        largest_number_wanted_input_vec.push(1000.to_string())
    }
    let largest_number_wanted_input_string = &largest_number_wanted_input_vec[1];
    let largest_number_wanted: i128 = match largest_number_wanted_input_string.parse() {
        Err(_err) => 100,
        Ok(result) => result,
    };
    let path = Path::new("primes.txt");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    primes.push(2);
    println!("Finding primes...");
    while current_number_being_checked < largest_number_wanted {
        current_number_being_checked = current_number_being_checked + 2;
        for divisor in primes.clone() {
            if current_number_being_checked % divisor == 0 {
                break;
            } else if (divisor as f64) > (current_number_being_checked as f64).sqrt() {
                primes.push(current_number_being_checked);
                break;
            }
        }
    }
    println!("Finished finding primes, writing to file.");
    let joined: String = primes.iter().map( |&prime| prime.to_string() + "\n").collect();
    file.write(joined.as_bytes());
}
