// Acest cod poate fi editat și rulat!
fn main() {
    // Un calculator simplu de numere întregi:
    // `+` sau `-` înseamnă a aduna sau scădea 1
    // `*` sau `/` înseamnă a înmulți sau divide la 2

    let program = "+ + * - /";
    let mut acumulator = 0;

    for simbol in program.chars() {
        match simbol {
            '+' => acumulator += 1,
            '-' => acumulator -= 1,
            '*' => acumulator *= 2,
            '/' => acumulator /= 2,
            _ => { /* ignoră restul */ }
        }
    }

    println!("Programul \"{}\" calculează valoarea {}",
              program, acumulator);
}
