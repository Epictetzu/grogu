use std::env;
use std::cmp;
use grogu::{gather_options, get_option_by_delta};
use tdameritradeclient::{TDAClient, OptionChain, auth::TDauth};


fn main() {
    let accesstoken = env::var("TD_ACCESS_TOKEN").unwrap();//Get the access token from the environmental variable
    let client = TDAClient::new(accesstoken); //Start a client
    
    gather_options(&client, "TQQQ", 3, "PUT", 1.0, "2021-01-28", "2021-02-06");
    println!("{:?}", get_option_by_delta(&client, "TQQQ", 3, "PUT", 1.0, "2021-01-28", "2021-01-31", -0.75));
}
