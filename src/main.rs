
fn main() {
    
    loop{      

        println!("\nQuer adicionar um novo TODO 📝?");

        println!("digite (sim) para confirmar 👍  ou (nao) para negar 👎");

        let resposta= input();

        if resposta  == "sim" {
            println!("\nQual TODO 📝 deseja criar?");

            let todo : String = input();    

            println!("\nvocê criou o TODO\n\n 🔹 {}", todo);
        } else {
            println!("\nAté a próxima 👋 e volte sempre!🫶");
            break;
        }  
    }
}



fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
 }
