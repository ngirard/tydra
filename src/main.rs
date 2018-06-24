extern crate serde;
extern crate serde_yaml;
extern crate termion;
extern crate tui;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate structopt;

mod actions;

use actions::{Action, ActionFile, Layout, Page, Return};
use failure::Error;
use structopt::StructOpt;
use termion::event;
use tui::backend::AlternateScreenBackend;
use tui::Terminal;

type Term = Terminal<AlternateScreenBackend>;

#[derive(Debug, StructOpt)]
struct AppOptions {
    #[structopt(value_name = "ACTION_FILE")]
    filename: String,

    /// Instead of showing the menu, validate the action file.
    #[structopt(long = "validate")]
    validate: bool,
}

fn main() {
    let options = AppOptions::from_args();
    let actions: ActionFile = load_actions(options.filename).expect("Failed to parse file");
    match actions.validate() {
        Ok(_) => {}
        Err(errors) => {
            eprintln!("Actions are invalid: {:#?}", errors);
            std::process::exit(1);
        }
    }

    if options.validate {
        eprintln!("File is valid.");
        std::process::exit(0);
    }

    run_menu(actions)
        .map_err(|error| {
            print!("{}", termion::cursor::Show);
            error
        })
        .unwrap();
}

fn load_actions(path: String) -> Result<ActionFile, Error> {
    std::fs::read_to_string(path)
        .map_err(|e| Error::from(e))
        .and_then(|data| serde_yaml::from_str(&data).map_err(|e| Error::from(e)))
}

fn open_alternative_screen() -> Result<Term, Error> {
    let backend = AlternateScreenBackend::new()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;
    Ok(terminal)
}

fn stop_alternative_screen(mut terminal: Term) -> Result<(), Error> {
    terminal.show_cursor()?;
    terminal.clear()?;
    terminal.show_cursor()?;
    drop(terminal);
    Ok(())
}

fn run_menu(actions: ActionFile) -> Result<(), Error> {
    let default_layout = actions.layout().unwrap_or_default();
    let mut current_page = actions.get_page("root");

    let mut terminal = open_alternative_screen()?;

    loop {
        render_menu(&mut terminal, current_page, default_layout)?;
        let action = process_input(current_page)?;
        match action {
            Action::Exit => break,
            Action::Redraw => {
                stop_alternative_screen(terminal)?;
                terminal = open_alternative_screen()?;
            }
            Action::Run { command, return_to } => {
                stop_alternative_screen(terminal)?;
                run_command(&command)?;
                terminal = open_alternative_screen()?;
                match return_to {
                    Return::Quit => break,
                    Return::Page(page_name) => current_page = actions.get_page(&page_name),
                }
            }
        }
    }

    terminal.show_cursor()?;
    Ok(())
}

fn render_menu<'a>(
    terminal: &mut Term,
    page: &'a Page,
    default_layout: Layout,
) -> Result<(), Error> {
    let current_layout = page.layout().unwrap_or(default_layout);

    current_layout.render(terminal, page)?;

    Ok(())
}

fn process_input<'a>(page: &'a Page) -> Result<Action, Error> {
    use termion::input::TermRead;
    let stdin = std::io::stdin();

    for evnt in stdin.keys().flat_map(Result::ok) {
        match evnt {
            event::Key::Esc => return Ok(Action::Exit),
            event::Key::Ctrl('l') => return Ok(Action::Redraw),
            event::Key::Char(c) => match page.entry_with_shortcut(c) {
                Some(entry) => return Ok(entry.into()),
                None => {}
            },
            _ => {}
        }
    }

    // stdin closed, or other state that makes stdin not produce any output anymore
    Err(format_err!("stdin eof"))
}

fn run_command(command: &str) -> Result<(), Error> {
    use std::process::Command;

    let _ = Command::new("/bin/sh").arg("-c").arg(&command).status()?;
    Ok(())
}
