fn main() {
    let greetings = ["Hallo", "Hello", "Hola", "Bonjour",
                     "Ciao", "こんにちは", "안녕하세요",
                     "Cześć", "Olá", "Здравствуйте",
                     "Chào bạn", "您好"];

    for (num, greeting) in greetings.iter().enumerate() {
        print!("{} : ", greeting);
        match num {
            0 =>  println!("Dieser Code kann bearbeitet und ausgeführt werden!"),
            1 =>  println!("This code is editable and runnable!"),
            2 =>  println!("¡Este código es editable y ejecutable!"),
            3 =>  println!("Ce code est modifiable et exécutable !"),
            4 =>  println!("Questo codice è modificabile ed eseguibile!"),
            5 =>  println!("このコードは編集して実行出来ます！"),
            6 =>  println!("여기에서 코드를 수정하고 실행할 수 있습니다!"),
            7 =>  println!("Ten kod można edytować oraz uruchomić!"),
            8 =>  println!("Esse código é editável e executável!"),
            9 =>  println!("Этот код можно отредактировать и запустить!"),
            10 => println!("Bạn có thể edit và run code trực tiếp!"),
            11 => println!("这段代码是可以编辑并且能够运行的！"),
            _ =>  {},
        }
    }
}
