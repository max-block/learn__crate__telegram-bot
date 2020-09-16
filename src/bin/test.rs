use std::env;
use telegram_bot::{Error, Api, GetMe};


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let token = env::var("TOKEN").expect("TOKEN is not set");
    let api = Api::new(token);

    let result = api.send(GetMe).await?;
    println!("{:?}", result);


    Ok(())
}