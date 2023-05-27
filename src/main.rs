use arboard::Clipboard;
use clap::Parser;
use enigo::*;
use std::{process, thread::sleep, time::Duration};

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, help = "do NOT clear the clipboard at the end")]
    no_clear: bool,
    #[arg(short, long, help = "confirm password by automatically hitting Return")]
    confirm: bool,
    #[arg(
        long,
        help = "countdown in seconds before starting to enter the password"
    )]
    countdown: Option<u64>,
}

trait AsKeySeq {
    fn as_key_seq(self) -> String;
}

fn shift(c: char) -> String {
    format!("{{+SHIFT}}{}{{-SHIFT}}", c)
}

impl AsKeySeq for char {
    fn as_key_seq(self) -> String {
        match self {
            '/' => shift('/'),
            '"' => shift('"'),
            '+' => shift('+'),
            '_' => shift('_'),
            '{' => shift('['),
            '}' => shift(']'),
            ':' => shift(':'),
            '<' => shift('<'),
            '>' => shift('>'),
            '?' => shift('?'),
            '|' => shift('|'),
            '~' => shift('~'),
            '!' => shift('!'),
            '@' => shift('@'),
            '#' => shift('#'),
            '$' => shift('$'),
            '%' => shift('%'),
            '^' => shift('^'),
            '&' => shift('&'),
            '*' => shift('*'),
            '(' => shift('('),
            ')' => shift(')'),
            c => String::from(c),
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut enigo = Enigo::new();
    let mut clip = match Clipboard::new() {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Access to clipboard denied");
            process::exit(1);
        }
    };
    let pass = match clip.get_text() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Clipboard doesn't contain text");
            process::exit(1);
        }
    };
    let countdown = args.countdown.unwrap_or_else(|| 3);
    println!("Typing password from clipboard in {}s...", countdown);
    sleep(Duration::from_secs(countdown));
    let sequence = pass
        .chars()
        .map(|i| i.as_key_seq())
        .reduce(|acc, i| acc + &i)
        .unwrap_or_else(|| String::new());
    enigo.key_sequence_parse(&sequence);
    if args.confirm {
        sleep(Duration::from_millis(100));
        enigo.key_click(enigo::Key::Return);
    }
    if !args.no_clear {
        if let Err(_) = clip.clear() {
            eprintln!("Failed to clear the clipboard.");
            process::exit(1);
        }
        println!("Clipboard cleared");
    }
}
