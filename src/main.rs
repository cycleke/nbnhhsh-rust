use serde::Deserialize;
use std::env;

const HELP_OPT: [&str; 4] = ["-h", "--h", "-help", "--help"];
const API_URL: &str = "https://lab.magiconch.com/api/nbnhhsh/guess";

#[derive(Deserialize)]
struct Guess {
    name: String,
    trans: Option<Vec<String>>,
    inputting: Option<Vec<String>>,
}

fn guess(words: &str) -> Result<(), ureq::Error> {
    let guess: Vec<Guess> = ureq::post(API_URL)
        .send_json(ureq::json!({
          "text": words,
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

fn help(exec: &str) {
    println!(
        "usage: {} sx
\tsx：拼音首字母缩写，支持多个缩写",
        exec
    );
}

fn main() -> Result<(), ureq::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1
        || args[1..]
            .iter()
            .any(|token| HELP_OPT.contains(&(&token.to_lowercase() as &str)))
    {
        help(&args[0]);
        return Ok(());
    }

    guess(&args[1..].join(","))
}
