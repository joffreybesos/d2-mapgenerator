use std::path::Path;

mod generate;
mod cache;

fn main() {
    let seed = "63444463";
    let difficulty = "2";
    let cached_seed_data_file = cache::cached_file_name(seed, difficulty);
    let seed_data: String = if Path::new(&cached_seed_data_file).exists() {
        println!("Reading cached map data from file {}", &cached_seed_data_file.to_str().unwrap());
        cache::read_cached_file(cached_seed_data_file)
    } else {
        println!("Generating fresh data for seed {} and difficulty {}", seed, difficulty);
        generate::generate_data(seed, difficulty)
    };
    // println!("{}", &seed_data);
}