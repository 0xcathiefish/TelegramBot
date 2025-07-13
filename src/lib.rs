use reqwest::blocking::Client;
use serde_json::json;
use log::{info, error};

use dotenvy::dotenv;
use std::env;

fn init_logger() {

    dotenv().ok();

    env_logger::init();
}

pub struct TelegramBot<'a>{

    bot_token: &'a str,
    chat_id: &'a str,
    text: &'a str
}


impl<'a> TelegramBot<'a> {

    pub fn new(bot_token: &'a str, chat_id: &'a str, text: &'a str) -> Self{

        TelegramBot{bot_token,chat_id,text}
    }

    pub fn send_msg(&self) { 

        init_logger();

        let client = Client::new();


        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);

        let result = client.post(url).json(&json!({

            "chat_id": self.chat_id,
            "text": self.text
        })).send();

        match result {

            Ok(response) => info!("Message sent Successfully !"),
            Err(e) => error!("Error: {}", e),
        }

    }
}

#[cfg(test)]

mod tests{

    use super::*;

    #[test]
    fn test_send_msg() {

        dotenv().ok();

        let bot_token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not found");
        let chat_id = env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID not found");
        let text = String::from("Rust tgbot test");

        let tg1 = TelegramBot::new(&bot_token,&chat_id,&text);

        tg1.send_msg();  
    }
}
