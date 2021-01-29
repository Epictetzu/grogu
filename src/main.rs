use std::env;
use grogu::*;
mod market_hours;
mod watchlist;

use tdameritradeclient::{TDAClient, OptionChain, auth::TDauth};


fn main() {
    let accesstoken = env::var("TD_ACCESS_TOKEN").unwrap();//Get the access token from the environmental variable
    let client = TDAClient::new(accesstoken); //Start a client
    
    let userprincipals: serde_json::Value = client.getuserprincipals();//Get principals so we can get accountid
    //prettyprint(&userprincipals);
    let accountid = userprincipals["primaryAccountId"].as_str().unwrap();
    
    let mut filteredwatchlists:  Vec<serde_json::Value> = Vec::new(); //Create a new empty mutable Vector of json Values
    watchlist::get_filtered_watchlists(&client, accountid, &mut filteredwatchlists);//Pass a mutable ref to this func so it can push json values to it
    
    prettyprint(&client.get_todays_market_hours("OPTION"));
    market_hours::is_market_open(&client, "OPTION");
}
