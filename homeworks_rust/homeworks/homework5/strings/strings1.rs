// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)



fn main() {
    let answer = current_favorite_course();
    println!("My course is {}", answer);
}

fn current_favorite_course() -> String {
    "Solana".to_string()
}
