const HEIGHT: usize = 6; // Висота верхньої частини ромба (без середньої лінії)
const CHAR: char = '*';

pub fn draw_diamond() {
    let mut output = String::new();

    // Верхня частина (включно з середньою лінією)
    for i in 0..HEIGHT {
        let stars = 1 + 2 * i;
        let spaces = HEIGHT - 1 - i;
        output += &format!("{}{}\n", " ".repeat(spaces), CHAR.to_string().repeat(stars));
    }

    // Нижня частина (без середньої лінії)
    for i in (0..HEIGHT - 1).rev() {
        let stars = 1 + 2 * i;
        let spaces = HEIGHT - 1 - i;
        output += &format!("{}{}\n", " ".repeat(spaces), CHAR.to_string().repeat(stars));
    }

    print!("{}", output); // Один print! для всього результату
}

fn main() {
    draw_diamond();
}
