// このコードは編集して実行出来ます！
fn main() {
    // 簡単な整数電卓:
    // `+` は1足す、、`-` は1引く
    // `*` は2掛ける、`/` は2割る

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
             program,
             accumulator);
}
