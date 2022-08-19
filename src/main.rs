use std::path::Path;

mod generate;
mod cache;

fn main() {
    let seed = "6344463";
    let difficulty = "2";
    let cached_seed_data_file = cache::cached_file_name(seed, difficulty);
    let seed_data: String = if Path::new(&cached_seed_data_file).exists() {
        cache::read_cached_file(cached_seed_data_file)
    } else {
        generate::generate_data(seed, difficulty)
    };
    println!("{}", &seed_data);
}