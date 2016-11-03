// Esse código é editável e executável!
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
            _ => { /* ignore todo o resto */ }
        }
    }

    println!("O programa \"{}\" calcula o valor {}", programa, acumulador);
}
