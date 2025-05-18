pub fn draw_tree(levels: usize) {
    let max_width = 1 + 2 * (levels + 2); // максимальна ширина нижнього шару

    for level in 1..=levels {
        for row in 0..level {
            let stars = 1 + 2 * row;
            let spaces = (max_width - stars) / 2;
            let line: String = " ".repeat(spaces) + &"*".repeat(stars);
            println!("{}", line);
        }
    }

    // стовбур (один "*", по центру, 1 рядок)
    let trunk = " ".repeat((max_width - 1) / 2) ;
    println!("{}", trunk);
}

fn main() {
    draw_tree(6); // змінити число для іншої кількості трикутників
}
