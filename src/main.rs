const HEIGHT: usize = 14; // висота конверта
const WIDTH: usize = 30;  // ширина конверта

fn main() {
    let mut result = String::new();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 {  // верхній і нижній край
                result.push('*');
            } else if j == 0 || j == WIDTH - 1 {  // лівий і правий край
                result.push('*');
            } else if j == i * (WIDTH - 1) / (HEIGHT - 1) || j == (WIDTH - 1) - i * (WIDTH - 1) / (HEIGHT - 1) {
                result.push('*');
            } else if i == HEIGHT / 2 && (j == WIDTH / 2 || j == WIDTH / 2 - 1) {
                // центр конверта - дві зірочки
                result.push('*');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');  // новий рядок після кожного рядка конверта
    }

    print!("{}", result);
}
