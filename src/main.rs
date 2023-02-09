#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

use std::io::{Error, Stdin, Stdout, Write};

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

fn main() {
    if let Err(erro) = start() {
        Terminal::erro_todo(erro);
    }
}

fn start() -> Result<(), TerminalError> {
    let mut pergunta = Terminal::new();

    loop {
        let todo = pergunta.ask_for_new_todo()?;

        match todo {
            Some(todo_msg) => pergunta.show_todo(&todo_msg)?,
            None => return Ok(()),
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

    fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        writeln!(self.stdout, "\nQuer adicionar um novo TODO 📝?\ndigite (sim) para confirmar 👍  ou (nao) para negar 👎").map_err(TerminalError::Stdout)?;

        let mut resposta = String::new();

        self.stdin
            .read_line(&mut resposta)
            .map_err(TerminalError::Stdin)?;

        if resposta.trim() == "sim" {
            writeln!(self.stdout, "\nQual TODO 📝 deseja criar?").map_err(TerminalError::Stdout)?;

            let mut novo_todo = String::new();

            self.stdin
                .read_line(&mut novo_todo)
                .map_err(TerminalError::Stdin)?;

            Ok(Some(Todo { message: novo_todo }))
        } else {
            writeln!(self.stdout, "\nAté a próxima 👋 e volte sempre!🫶\n")
                .map_err(TerminalError::Stdout)?;

            return Ok(None);
        }
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        writeln!(self.stdout, "\nvocê criou o TODO\n\n 🔷 {}", todo.message)
            .map_err(TerminalError::Stdout)?;
        Ok(())
    }

    fn erro_todo(erro: TerminalError) {
        eprintln!(" Deu Erro! {:?}", erro);
    }
}

#[derive(Debug)]
enum TerminalError {
    Stdout(Error),
    Stdin(Error),
}
