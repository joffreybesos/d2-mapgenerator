mod generate;

fn main() {
    let seed = "634463";
    let difficulty = "2";


    let seed_data: String = generate::generate_data(seed, difficulty);
    println!("{}", &seed_data);
}