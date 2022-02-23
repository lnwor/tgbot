use std::env;
use std::time::Duration;
use teloxide::prelude2::*;
use teloxide::types::InputFile;
use teloxide::net;
pub mod lib;
use lib::error::Error;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use home;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    token: String,
}

#[tokio::main]
async fn main() {
    run().await.unwrap_or_else(|error| error.exit());
}

async fn run() -> Result<(), Error> {
    let f = std::fs::File::open(format!("{}{}",home::home_dir().unwrap().display(),"/.config/tgbot.yml")).expect("Could not open file.");
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    teloxide::enable_logging!();
    log::info!("Starting uploading...");
    let args: Vec<String> = env::args().collect();

    let client = net::default_reqwest_settings().timeout(Duration::from_secs(60)).build().unwrap();
    let bot = Bot::with_client(scrape_config.token, client);

    if !args[1].parse::<i64>().is_ok(){
        println!("The chat_id \"{}\" is not valid", &args[1]);
        std::process::exit(-1);
    }

    if args.len() == 3{
        let chat_id: i64 = args[1].parse().unwrap();
        bot.send_document(chat_id, InputFile::file(&args[1]))
            .send()
            .await?;
        log::info!("Upload successful");
    }
    else if args.len() == 4 {
        let chat_id: i64 = args[1].parse().unwrap();
        bot.send_document(chat_id, InputFile::file(&args[2]))
            .caption(&args[3])
            .send()
            .await?;
        log::info!("Upload successful");
    } else {
        log::error!("You didn't provide 2 arguments");
        println!("\nUsage: tgbot /path/to/file \"caption of file\"");
    }
    Ok(())
}