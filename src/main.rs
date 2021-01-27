use std::env;
use std::cmp;
use grogu::*;
use chrono::{TimeZone, Utc};
use tdameritradeclient::{TDAClient, OptionChain, auth::TDauth};


fn main() {
    let accesstoken = env::var("TD_ACCESS_TOKEN").unwrap();//Get the access token from the environmental variable
    let client = TDAClient::new(accesstoken); //Start a client
    
    let userprincipals: serde_json::Value = client.getuserprincipals();//Get principals so we can get accountid
    prettyprint(&userprincipals);
    let accountid = userprincipals["primaryAccountId"].as_str().unwrap();

    let watchlists: serde_json::Value = client.get_watchlists(accountid);
    prettyprint(&watchlists);
    prettyprint(&client.get_todays_market_hours("OPTION"));
}
