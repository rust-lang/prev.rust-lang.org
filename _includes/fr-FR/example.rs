// Ce code est modifiable et exécutable !
fn main() {
    // Une calculatrice simple avec des entiers :
    // `+` ou `-` signifie ajouter ou soustraire 1
    // `*` ou `/` signifie multiplier ou diviser par 2

    let programme = "+ + * - /";
    let mut accumulateur = 0;

    for symbole in programme.chars() {
        match symbole {
            '+' => accumulateur += 1,
            '-' => accumulateur -= 1,
            '*' => accumulateur *= 2,
            '/' => accumulateur /= 2,
            _ => { /* on ne tient pas compte des autres symboles */ }
        }
    }

    println!("Le programme \"{}\" a calculé la valeur {}",
              programme, accumulateur);
}
