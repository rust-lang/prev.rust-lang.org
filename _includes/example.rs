fn main() { 
    let greetings = ["Hello", "Hola", "Bonjour",
                     "こんにちは", "您好"];

    for (num,greeting) in greetings.iter().enumerate() {
        println!("{}", greeting);
        match num {
            0 =>  println!("This code is editable and runnable!"),
            1 =>  println!("Este código es editable y ejecutable!"),
            2 =>  println!("Ce code est modifiable et exécutable!"),
            3 =>  println!("このコードは編集して実行出来ます！"),
            4 =>  println!("这个代码是可以编辑并且能够运行的！"),
            _ =>  {},
        }
    }
}
