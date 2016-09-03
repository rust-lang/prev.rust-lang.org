// 여기에서 코드를 수정하고 실행할 수 있습니다!
fn main() {
    // 간단한 정수 계산기:
    // `+` 또는 `-`는 1을 더하거나 뺍니다
    // `*` 또는 `/`는 2를 곱하거나 나눕니다

    let program = "+ + * - /";
    let mut accumulator = 0;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* 다른 문자를 무시 */ }
        }
    }

    println!("프로그램 \"{}\"의 계산값은 {}입니다",
              program, accumulator);
}
