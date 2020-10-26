extern crate pcsc;

use std::{io, process};

use structopt::StructOpt;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::Terminal;
use tui::widgets::{Block, Borders};
use std::str::FromStr;
use std::string::ParseError;

mod banner;

enum Command {
    SelectApplication,
    GetPinTryCounter,
    GetProcessingOptions,
    GenerateAC,
    VerifyPIN,
    ReadRecord,
    Raw,
}

#[derive(Debug, StructOpt)]
enum ConnectionMode {
    USB,
    App,
}

fn parse_connection(conn: &str) -> Result<ConnectionMode, String> {
    match conn {
        "usb" => Ok(ConnectionMode::USB),
        "app" => Ok(ConnectionMode::App),
        _ => Err(format!("Unknown connection mode: {}", conn))
    }
}

fn error(message: &str) {
    eprintln!("{}", message);
    process::exit(1);
}

#[derive(Debug, StructOpt)]
struct CliOptions {
    /// Activate interactive mode
    #[structopt(short, long)]
    interactive: bool,
    /// Connection mode
    #[structopt(short, long, parse(try_from_str = parse_connection), default_value = "usb")]
    connection: ConnectionMode,
}


struct UserInput {
    command: Command,
    args: Vec<String>,
}

fn main() -> Result<(), io::Error> {
    println!("{}", banner::BANNER);
    let options: CliOptions = CliOptions::from_args();
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size)
    });

    Ok(())
}

fn execute_usb() {
    let card = emv::connect().expect("No card detected!");
    let command = parse_input();

    match command.command {
        Command::SelectApplication => {} //emv::select_application(&card, command.args[0]),
        Command::GetPinTryCounter => emv::read_pin_try_counter(&card),
        Command::GetProcessingOptions => emv::get_processing_options(&card),
        Command::ReadRecord => {} //emv::read_record(&card, command.args[0], command.args[1]),
        Command::GenerateAC => {}
        Command::VerifyPIN => {}
        Command::Raw => {}
    }
}


fn parse_input() -> UserInput {
    let args: Vec<String> = [String::from("")].to_vec();
    let command = Command::Raw;
    UserInput { args, command }
}
