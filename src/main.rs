use std::env;
use rust_trading_bot::{gather_options, get_option_by_delta};
use tdameritradeclient::{TDAClient, OptionChain, auth::TDauth};


fn main() {
    let accesstoken = env::var("TD_ACCESS_TOKEN").unwrap();
    let client = TDAClient::new(accesstoken);
    
    gather_options(&client, "TQQQ", 3, "PUT", 1.0, "2021-01-28", "2021-02-06");
    get_option_by_delta(&client, "TQQQ", 3, "PUT", 1.0, "2021-01-28", "2021-01-31", -0.75);
}
