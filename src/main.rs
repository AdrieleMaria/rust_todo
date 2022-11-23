#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

use std::io::{Stdin, Stdout, Write};

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

fn main() {
    let mut pergunta = Terminal::new();

    loop {       
        let todo = pergunta.ask_for_new_todo();

        pergunta.show_todo(&todo);
    }
}

impl Terminal {
    fn new() -> Self {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    fn ask_for_new_todo(&mut self) -> Todo {
        println!("\nQuer adicionar um novo TODO 📝?");

        println!("digite (sim) para confirmar 👍  ou (nao) para negar 👎");

        let mut resposta = String::new();

        self.stdin.read_line(&mut resposta).expect("sim");

        if resposta.trim() == "sim" {
            println!("\nQual TODO 📝 deseja criar?");

            let mut novo_todo = String::new();

            self.stdin.read_line(&mut novo_todo).unwrap();

            Todo { message: novo_todo }
        } else {
            println!("\nAté a próxima 👋 e volte sempre!🫶");

            std::process::exit(0);
        }
    }

    fn show_todo(&mut self, todo: &Todo) {
        writeln!(self.stdout, "\nvocê criou o TODO\n\n 🔹 {}", todo.message).unwrap();
    }
}
