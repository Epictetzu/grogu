//Example only will not run from folder its in.
//How to start a client and get a vector of the watchlists filtered by name
use std::env;
use grogu::*;
use tdameritradeclient::TDAClient;
pub use crate::market_hours::*;
pub use crate::watchlist::*;



fn main(){
    let accesstoken = env::var("TD_ACCESS_TOKEN").unwrap();//Get the access token from the environmental variable
    let client = TDAClient::new(accesstoken); //Start a client
    let userprincipals: serde_json::Value = client.getuserprincipals();//Get principals so we can get accountid
    prettyprint(&userprincipals);
    let accountid = userprincipals["primaryAccountId"].as_str().unwrap();

    let mut filteredwatchlists:  Vec<serde_json::Value> = Vec::new(); //Create a new empty mutable Vector of json Values
    watchlist::get_filtered_watchlists(&client, accountid, &mut filteredwatchlists);//Pass a mutable ref to this func so it can push json values to it
    println!("{:?}", filteredwatchlists);
}