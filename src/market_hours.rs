//Functions for checking if market is open today
use chrono::prelude::*;
use tdameritradeclient::{TDAClient};

pub fn is_open_today(client: &TDAClient, market: &str) -> bool{
    let todayshours: serde_json::Value = client.get_todays_market_hours("OPTION");
    return todayshours["option"]["EQO"]["isOpen"].as_bool().unwrap();
}
pub fn is_market_open(client: &TDAClient, market: &str) -> bool{
    if is_open_today(&client, market) { //First check if market will be open today
        let nowdt = Utc::now(); // Create a dateTime object for now
                
        let markethours: serde_json::Value = client.get_todays_market_hours(market); //Get the market hours for today as a json value
        
        //Get starting hours then get the duration since/until open. Negative number is before(until) positve is after(since)
        let startdt = DateTime::parse_from_rfc3339(markethours["option"]["EQO"]["sessionHours"]["regularMarket"][0].get("start").unwrap().as_str().unwrap()).unwrap();
        let timesincestart = nowdt.signed_duration_since(startdt);
        
        //Get starting hours then get the duration since/until close. Negative number is before(until) positve is after(since)
        let enddt = DateTime::parse_from_rfc3339(markethours["option"]["EQO"]["sessionHours"]["regularMarket"][0].get("end").unwrap().as_str().unwrap()).unwrap();
        let timesinceend = nowdt.signed_duration_since(enddt);
        
        //If the duration since open is positve AND the duration until end negative then return true 
        if timesincestart.num_seconds() > 0 && timesinceend.num_seconds() < 0 {return true}else{return false};
    }
    return false;
}
