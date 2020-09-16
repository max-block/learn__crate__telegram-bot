use std::env;

use telegram_bot::{Api, CanSendMessage, ChatId, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let token = env::var("TOKEN").expect("TOKEN is not set");
    let chat_id = env::var("CHAT_ID").expect("TOKEN is not set");
    let chat_id = chat_id.parse::<i64>().expect("CHAT_IS is not number");

    let api = Api::new(token);

    let chat = ChatId::new(chat_id);
    let result = api.send(chat.text("bla bla bla")).await?;
    println!("{:#?}", result);
    
    Ok(())
}