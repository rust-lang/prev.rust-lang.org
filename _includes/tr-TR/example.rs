// Bu kodu düzenleyebilir ve çalıştırabilirsiniz!
fn main() {
    // Basit hesap tam sayı makinası:
    // `+` bir ile toplama, `-` bir ile çıkarma.
    // `*` iki ile çarpma, `/` iki ile bölme.

    let program = "+ + * - /";
    let mut akumulator = 0;

    for sembol in program.chars() {
        match sembol {
            '+' => akumulator += 1,
            '-' => akumulator -= 1,
            '*' => akumulator *= 2,
            '/' => akumulator /= 2,
            _ => { /* diğer karakterleri es geç */ }
        }
    }

    println!("\"{}\" programının hesaplama sonucu: {}",
              program, akumulator);
}
