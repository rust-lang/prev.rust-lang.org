// Esse código é editável e rodável!
fn main() {
    // Uma calculadora simples de inteiros:
    // `+` ou `-` significa somar ou subtrair 1
    // `*` ou `/` significa multiplicar ou dividir por 2

    let programa = "+ + * - /";
    let mut acumulador = 0;

    for simbolo in programa.chars() {
        match simbolo {
            '+' => acumulador += 1,
            '-' => acumulador -= 1,
            '*' => acumulador *= 2,
            '/' => acumulador /= 2,
            _ => {
                // ignore todo o resto
            }
        }
    }

    println!("O programa \"{}\" gera o valor {}", programa, acumulador);
}
