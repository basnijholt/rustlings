// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(word.as_str()) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

// q: what is difference between String and str?
// a: String is a heap allocated string, str is a string slice

// q: when to use str vs String?
// a: when you want to own the string, use String, when you want to borrow the string, use str

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
