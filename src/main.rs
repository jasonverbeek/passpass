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
    println!("Typing password from clipboard in 3s...");
    sleep(Duration::from_secs(3));
    let operations = pass
        .chars()
        .map(|i| i.as_key_seq())
        .collect::<Vec<String>>();
    let mut sequence: String = String::new();
    for op in &operations {
        sequence += op;
    }
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
