/// Створює вектор вручну (імітація генерації випадкових чисел)
fn gen_random_vector(_n: usize) -> Vec<i32> {
    vec![
        45, 87, 49, 64, 50, 37, 45, 72, 55, 64,
        90, 86, 60, 54, 78, 72, 83, 44, 89, 22
    ]
}

/// Знаходить пару індексів з мінімальною сумою сусідніх елементів
fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, min_index + 1, min_sum)
}

/// Форматований вивід
fn print_data_analysis(data: &[i32]) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    print!("data:    [");
    for (i, val) in data.iter().enumerate() {
        print!("{:>2}", val);
        if i != data.len() - 1 {
            print!(", ");
        }
    }
    println!("]");

    let (i1, i2, min_sum) = min_adjacent_sum(data);

    // Print visual indicators
    print!("indexes: ");
    for i in 0..data.len() {
        if i == i1 {
            print!("\\__ ");
        } else if i == i2 {
            print!("__/ ");
        } else {
            print!("     ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{}:{}",
        data[i1], data[i2], min_sum, i1, i2
    );
}

fn main() {
    let data = gen_random_vector(20);
    print_data_analysis(&data);
}