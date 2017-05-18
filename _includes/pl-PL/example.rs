// Ten kod można edytować oraz uruchomić!
fn main() {
    // Prosty kalkulator dla liczb całkowitych:
    // `+` lub `-` oznacza dodanie lub odjęcie 1
    // `*` lub `/` oznacza pomnożenie lub podzielenie przez 2

    let program = "+ + * - /";
    let mut accumulator = 0;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* ignoruj wszystkie pozostałe znaki */ }
        }
    }

    println!("Program \"{}\" wylicza wartość {}",
              program, accumulator);
}
