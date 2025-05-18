pub fn invert_the_case(s: String) -> String {
    s.chars()
        .flat_map(|c| {
            if c.is_uppercase() {
                Box::new(c.to_lowercase()) as Box<dyn Iterator<Item = char>>
            } else if c.is_lowercase() {
                Box::new(c.to_uppercase()) as Box<dyn Iterator<Item = char>>
            } else {
                Box::new(std::iter::once(c)) as Box<dyn Iterator<Item = char>>
            }
        })
        .collect()
}

fn main() {
    let original = String::from("Hello, Привет!");
    let inverted = invert_the_case(original.clone());
    println!("Original: {}", original);
    println!("Inverted: {}", inverted);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

        data.iter().for_each(|(a, b)| {
            assert_eq!(invert_the_case(a.to_string()), b.to_string());
            assert_eq!(invert_the_case(b.to_string()), a.to_string());
        });
    }
}
