
use grogu::*;
use tdameritradeclient::{TDAClient};



fn main() -> std::io::Result<()>{
    
    tokens::get_new_access().unwrap();
    //let accesstoken = env::var("TD_ACCESS_TOKEN").unwrap();//Get the access token from the environmental variable
    //println!("{:?}",tokens::readaccess().unwrap());
    let client = TDAClient::new(tokens::read_access().unwrap()); //Start a client
    
    let userprincipals: serde_json::Value = client.getuserprincipals();//Get principals so we can get accountid, tokenExp, and other stuff
    
    tokens::save_principals(&userprincipals)?;
    let accountid = get_accountid(&userprincipals);
    
    let mut filteredwatchlists:  Vec<serde_json::Value> = Vec::new(); //Create a new empty mutable Vector of json Values
    watchlist::get_filtered_watchlists(&client, accountid, &mut filteredwatchlists);//Pass a mutable ref to this func so it can push json values to it
    
    prettyprint(&client.get_todays_market_hours("OPTION"));
    println!("{:?}", market_hours::is_market_open(&client, "OPTION"));
    println!("{:?}", tokens::read_access());
    tokens::renew_access()?;
    Ok(())
}