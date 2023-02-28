mod models;
use crate::terminal::{Terminal, TerminalError};
use models::{terminal, todo};

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
