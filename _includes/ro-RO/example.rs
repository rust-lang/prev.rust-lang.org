// Acest cod este modificabil și executabil.
fn main() {
    // Un calculator simplu de întregi:
    // `+` sau `-` înseamnă adunare sau scădere cu 1
    // `*` sau `/` înseamnă multiplicare sau împărțire cu 2

    let program = "+ + * - /";
    let mut acumulator = 0;

    for simbol in program.chars() {
        match simbol {
            '+' => acumulator += 1,
            '-' => acumulator -= 1,
            '*' => acumulator *= 2,
            '/' => acumulator /= 2,
            _ => { /* ignoră orice altceva */ }
        }
    }

    println!("Programul \"{}\" a calculat valoarea {}",
              program, acumulator);
}
