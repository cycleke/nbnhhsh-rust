use serde::Deserialize;
use std::env;

const HELP_OPT: [&str; 4] = ["-h", "--h", "-help", "--help"];
const API_URL: &str = "https://lab.magiconch.com/api/nbnhhsh/guess";

fn main() -> Result<(), ureq::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || find_help(&args) {
        help(&args[0]);
        return Ok(());
    }

    guess(&args[1..])
}

fn find_help(args: &[String]) -> bool {
    args[1..]
        .iter()
        .any(|token| HELP_OPT.contains(&(&token.to_lowercase() as &str)))
}

fn help(exec: &str) {
    println!(
        "usage: {} sx
\tsx：拼音首字母缩写，支持多个缩写",
        exec
    );
}

#[derive(Deserialize)]
struct Guess {
    name: String,
    trans: Option<Vec<String>>,
    inputting: Option<Vec<String>>,
}

fn guess(words: &[String]) -> Result<(), ureq::Error> {
    let words: Vec<_> = words
        .iter()
        .flat_map(|word| word.split(|chr: char| !chr.is_ascii_alphanumeric()))
        .filter(|word| !word.is_empty())
        .collect();

    let guess: Vec<Guess> = ureq::post(API_URL)
        .send_json(ureq::json!({
          "text": &words[..].join(" "),
        }))?
        .into_json()?;

    for Guess {
        name,
        trans,
        inputting,
    } in guess
    {
        println!("{}", name);

        if let Some(trans) = trans {
            print!("\t释义：");
            for meaning in trans {
                print!(" {}", meaning);
            }
            println!();
        } else {
            println!("\t无对应释义");
        }

        if let Some(maybe) = inputting {
            print!("\t有可能是：");
            for meaning in maybe {
                print!(" {}", meaning);
            }
            println!();
        }

        println!();
    }

    Ok(())
}
