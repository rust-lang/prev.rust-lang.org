// Ten kod można edytować oraz uruchomić!
fn main() {
    // Prosty kalkulator dla liczb całkowitych:
    let program = "4+ 3* 2- 1/";
    let mut accumulator = 0;
    let mut figure = 0;
    for token in program.chars() {
        match token {
            '+' => {
                accumulator += figure;
                figure = 0;
            }
            '-' => {
                accumulator -= figure;
                figure = 0;
            }
            '*' => {
                accumulator *= figure;
                figure = 0;
            }
            '/' => {
                accumulator /= figure;
                figure = 0;
            }
            '0'...'9' => {
                figure *= 10;
                figure += token as i8 - '0' as i8;
            }
            _ => { /* ignore everything else */ }
        }
    }
    println!("The program \"{}\" calculates the value {}",
             program, accumulator);
}
