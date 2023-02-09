use crate::todo::Todo;

use std::io::{Error, Stdin, Stdout, Write};

use console::style;

pub struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    pub fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        //let bold = style("bold");
        writeln!(
            self.stdout,
            "\nQuer adicionar um novo {} 📝?\ndigite {} para confirmar 👍  ou {} para negar 👎",
            style("TODO").on_yellow(),
            style("sim").green(),
            style("nao").red()
        )
        .map_err(TerminalError::Stdout)?;
        //println!("{} is {} {}", style("quite").cyan(), style("this").on_white(),style("outra").bold());
        let mut resposta = String::new();

        self.stdin
            .read_line(&mut resposta)
            .map_err(TerminalError::Stdin)?;

        if resposta.trim() == "sim" {
            writeln!(
                self.stdout,
                "\nQual {} 📝 deseja {}?",
                style("TODO").on_yellow(),
                style("criar").bold()
            )
            .map_err(TerminalError::Stdout)?;

            let mut novo_todo = String::new();

            self.stdin
                .read_line(&mut novo_todo)
                .map_err(TerminalError::Stdin)?;

            Ok(Some(Todo { message: novo_todo }))
        } else {
            writeln!(
                self.stdout,
                "\nAté a {} 👋 e {}!🫶\n",
                style("próxima").italic(),
                style("volte sempre!").underlined()
            )
            .map_err(TerminalError::Stdout)?;

            return Ok(None);
        }
    }

    pub fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        writeln!(
            self.stdout,
            "\nvocê criou o {}\n\n 🔷 {}",
            style("TODO").on_yellow(),
            todo.message
        )
        .map_err(TerminalError::Stdout)?;
        Ok(())
    }

    pub fn erro_todo(erro: TerminalError) {
        eprintln!(" {} {:?}", style("Deu Erro!").on_red(), erro);
    }
}

#[derive(Debug)]
pub enum TerminalError {
    Stdout(Error),
    Stdin(Error),
}
