use pokedriver;

fn main() {
    match pokedriver::start() {
        Ok(_) => {}
        Err(e) => println!("error: {}", e.to_string())
    }
}