// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar' for the type `String'.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a hint.

// I AM DONE

trait AppendBar {
    fn append_bar(self) -> Self;
    fn append(x: String, y: Self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self: Self) -> Self {
        self + "Bar"
    }
    fn append(x: String, y: Self) -> Self {
        x + y.as_str()
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    // let y = String::append("yo".into(), s);
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
