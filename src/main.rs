mod environment;

extern crate reqwest;

use clokwerk::{AsyncScheduler, TimeUnits};
use std::error::Error;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use teloxide::Bot;
// Import week days and WeekDay
use clokwerk::Interval::*;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Store {
    name: String,
    link: String,
    in_stock_string_list: Vec<&'static str>,
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

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let bot = Bot::from_env().auto_send();
    let chat_id = environment::variables::get_application_chat_id();

    match find_xbox().await {
        Ok(messages) => {
            for message in messages {
                bot.send_message(ChatId::Id(chat_id), message).await?;
            }
        }
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    //let mut scheduler = JobScheduler::new();
    environment::variables::load();
    let time_in_minutes =
        environment::variables::get_application_scheduler_time_minutes().minutes();

    let mut scheduler = AsyncScheduler::new();

    scheduler.every(time_in_minutes).run(|| async {
        match run().await {
            Ok(_) => println!("OK"),
            Err(error) => println!("{:?}", error),
        }
    });

    loop {
        scheduler.run_pending().await;
        thread::sleep(Duration::from_millis(100));
    }
}

async fn find_xbox() -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
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
            ],
        },
        Store {
            name: String::from("EBGames"),
            link: String::from("https://www.gamestop.ca/Xbox%20Series%20X/Games/877779/xbox-all-access-xbox-series-x?xaa=1"),
            in_stock_string_list: vec!["Home Delivery", "Collect In Store"],
        },
        Store {
            name: String::from("Best Buy"),
            link: String::from("https://www.bestbuy.ca/en-ca/product/xbox-series-x-1tb-console/14964951"),
            in_stock_string_list: vec!["Available to ship", "Available for free store pickup"],
        },
    ];

    let mut messages: Vec<String> = vec![];

    for store in stores {
        match find_xbox_by(store).await {
            Ok(result) => messages.push(format!(
                "Xbox was found on: {} link: {}",
                result.name, result.link
            )),
            Err(err) => println!("{:?}", err.to_string()),
        };
    }

    Ok(messages)
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
