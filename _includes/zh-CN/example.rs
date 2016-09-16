// 这个代码是可以编辑并且能够运行的
fn main() {
    // 一个简易计算器
    // `+` 或 `-` 意味着加减1
    // `*` 或 `/` 意味着乘除2

    let program = "+ + * - /";
    let mut accumulator = 0;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* 忽略其他 */ }
        }
    }

    println!("程序 \"{}\" 的结果是 {}",
              program, accumulator);
}
