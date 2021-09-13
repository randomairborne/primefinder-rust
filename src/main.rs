use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use num_format::{Locale, ToFormattedString};

fn main() {
    let mut primes: Vec<u64> = Vec::new();
    let mut current_number_being_checked = 1;
    let mut largest_number_wanted_input_vec: Vec<String> = env::args().collect();
    if largest_number_wanted_input_vec.len() != 2 {
        largest_number_wanted_input_vec.push(1000000.to_string())
    }
    let largest_number_wanted_input_string = &largest_number_wanted_input_vec[1];
    let largest_number_wanted: u64 = match largest_number_wanted_input_string.parse() {
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
    let before = Instant::now();
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
    println!("Total time to find {} primes: {:.2?}", primes.len().to_formatted_string(&Locale::en), before.elapsed());
    println!("Finished finding primes, writing to file.");
    let joined: String = primes.iter().map(|&prime| prime.to_string() + "\n").collect();
    match file.write(joined.as_bytes()) {
        Err(why) => panic!("There was an error: `{:?}` writing to file!", why),
        Ok(file) => file,
    };
}
