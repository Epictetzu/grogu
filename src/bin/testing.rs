
use std::env;
use grogu::*;
use tdameritradeclient::TDAClient;
pub use crate::market_hours::*;



fn main(){
    let accesstoken = env::var("TD_ACCESS_TOKEN").unwrap();//Get the access token from the environmental variable
    let client = TDAClient::new(accesstoken); //Start a client
    
    println!("{:?}", market_hours::is_market_open(&client, "OPTION"));
}