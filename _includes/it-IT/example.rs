// Questo codice è modificabile e eseguibile!
fn main() {
    // Una semplice calcolatrice per numeri interi:
    // `+` o `-` aggiunge o sottrae 1
    // `*` o `/` moltiplica o divide per 2

    let programma = "+ + * - /";
    let mut accumulatore = 0;

    for simbolo in programma.chars() {
        match simbolo {
            '+' => accumulatore += 1,
            '-' => accumulatore -= 1,
            '*' => accumulatore *= 2,
            '/' => accumulatore /= 2,
            _ => { /* ignora gli altri caratteri */ }
        }
    }

    println!("Il programma \"{}\" calcola il valore {}",
              programma, accumulatore);
}
