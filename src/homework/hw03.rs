const WIDTH: usize = 20;
const HEIGHT: usize = 20;

fn main() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if x == 0 || x == WIDTH - 1 || y == 0 || y == HEIGHT - 1 {
                // Границы рамки
                output.push('*');
            } else if x == y || x == WIDTH - y - 1 {
                // Диагональные линии
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}