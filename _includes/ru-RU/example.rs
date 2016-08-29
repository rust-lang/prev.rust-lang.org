// Этот код можно отредактировать и запустить!
fn main() {
    // Простой целочисленный калькулятор:
    // `+` или `-` означает прибавление или вычитание единицы
    // `*` или `/` означает умножение или деление на 2

    let program = "+ + * - /";
    let mut accumulator = 0;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* игнорируем всё остальное */ }
        }
    }

    println!("Программа \"{}\" производит значение {}",
              program, accumulator);
}
