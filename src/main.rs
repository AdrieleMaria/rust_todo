#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

use std::io::{Error, Stdin, Stdout, Write};

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

fn input() -> String {
    let string = String::new();
    string.to_string()
}

fn main() {
    let mut pergunta = Terminal::new();

    loop {
        let todo = pergunta.ask_for_new_todo();

        match todo {
            Ok(todo) => pergunta.show_todo(&todo),

            Err(Error) => pergunta.erro_todo(&Error),
        }
    }
}

impl Terminal {
    fn new() -> Self {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    fn ask_for_new_todo(&mut self) -> Result<Todo, Error> {
        writeln!(self.stdout, "\nQuer adicionar um novo TODO 📝?\ndigite (sim) para confirmar 👍  ou (nao) para negar 👎").unwrap();

        let mut resposta = input();

        self.stdin.read_line(&mut resposta)?;

        if resposta.trim() == "sim" {
            writeln!(self.stdout, "\nQual TODO 📝 deseja criar?")?;

            let mut novo_todo = input();

            self.stdin.read_line(&mut novo_todo)?;

            Ok(Todo { message: novo_todo })
        } else {
            writeln!(self.stdout, "\nAté a próxima 👋 e volte sempre!🫶\n")?;

            std::process::exit(0);
        }
    }

    fn show_todo(&mut self, todo: &Todo) {
        writeln!(self.stdout, "\nvocê criou o TODO\n\n 🔷 {}", todo.message);
    }

    fn erro_todo(&mut self, erro: &Error) {
        format!("\nDeu Erro!\n\n ");
    }
}
