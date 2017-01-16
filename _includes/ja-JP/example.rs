// このコードは編集して実行出来ます！
fn main() {
    // 簡単な整数電卓:
    // `+` 、 `-` は１を足す、引く
    // `*` 、 `/` は２で掛ける、割る

    let program = "+ + * - /";
    let mut accumulator = 0;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* 他の全ては無視 */ }
        }
    }

    println!("プログラム \"{}\" の計算結果は {}。",
              program, accumulator);
}
