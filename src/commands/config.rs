use crate::commands::{ Config, CLICommand };
use std::io::{stdout, Write};
use crossterm::{
    execute, queue,
    style::{self, Stylize}, cursor, terminal, Result
};

impl CLICommand for Config {
    fn exec(&self) {
        let _ = run_config();    
   }
}

fn run_config() -> Result<()> {
    let mut stdout = stdout();

    // execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    return Ok(());
}