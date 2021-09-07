extern crate job_scheduler;
extern crate reqwest;
extern crate select;

use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use dotenv::dotenv;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::error::Error;
use scraper::{Html, Selector};
use teloxide::Bot;
use teloxide::types::InputFile;
use teloxide::{prelude::*, utils::command::BotCommand};
use std::borrow::Borrow;

#[derive(Debug)]
struct Store {
    name: String,
    link: String,
    in_stock_string_list: Vec<&'static str>
}

impl Clone for Store {
    fn clone(&self) -> Self {
        Store {
            name: self.name.clone(),
            link: self.link.clone(),
            in_stock_string_list: self.in_stock_string_list.clone(),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut scheduler = JobScheduler::new();
    teloxide::enable_logging!();

    scheduler.add(Job::new("1/2 * * * * *".parse().unwrap(), || {

    }));

    loop {
        scheduler.tick();
        println!("I get executed every 2 seconds");
        std::thread::sleep(Duration::from_millis(500))
    }
}

async fn find_xbox() -> Result<(), Box<dyn Error + Send + Sync>> {
    let stores = vec![
        Store {
            name: String::from("Walmart"),
            link: String::from("https://www.walmart.ca/en/ip/xbox-series-x/6000201786332"),
            in_stock_string_list: vec![
                "Out of Stock",
                "Available at nearby stores",
                "This item is sold online only",
                "Arrives",
                "Out of stock at nearby stores",
            ]
        },
        Store {
            name: String::from("EBGames"),
            link: String::from("https://www.walmart.ca/en/ip/xbox-series-x/6000201786332"),
            in_stock_string_list: vec![
                "Available at nearby stores",
                "This item is sold online only",
                "Arrives",
                "Out of stock at nearby stores",
                "Out"
            ]
        }
    ];

    for store in stores {
        match find_xbox_by(store).await {
                Ok(result) => {
                    let bot = Bot::from_env().auto_send();
                    let message_string = format!("Xbox found at Store: {} Link: {}", result.name, result.link);
                    teloxide::repl(bot, |message| async move {
                        message.answer("Xbox found");
                        respond(())
                }).await
            },
            Err(err) => println!("{}", err)
        }
    }

    Ok(())
}



async fn find_xbox_by(store: Store) -> Result<Store, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let response = client.get(&store.link)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.63 Safari/537.36")
        .header("referer", "https://www.amazon.com/s?k=nike+shoes+men&crid=28WRS5SFLWWZ6&sprefix=nike%2Caps%2C357&ref=nb_sb_ss_organic-diversity_2_4")
        .send().await?;

    assert!(response.status().is_success());

    let body = response.text().await?;
    let in_stock_string_list = store.in_stock_string_list.clone();

    for in_stock_string in in_stock_string_list.iter() {
        if body.as_str().find(in_stock_string) != None {
            return Ok(store);
        }
    }

    Err(Box::from("Xbox is not available in this store"))
}