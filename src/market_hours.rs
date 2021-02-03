//Functions for checking if market is open today
use chrono::prelude::*;
use tdameritradeclient::{TDAClient};

//
//Checks if the market will be or is open today. 
//Iterates down one level to get the first key because the key changes during the day.
pub fn is_open_today(client: &TDAClient, market: &str) -> bool{
    let todayshours: serde_json::Value = client.get_todays_market_hours(market);
    let secondkey: Vec<&String> = todayshours["option"].as_object()//as map for the "keys()" iterator
                                                        .unwrap()
                                                        .keys()
                                                        .collect();//Collect the keys into the vector
    return todayshours["option"][secondkey[0].as_str()]["isOpen"].as_bool().unwrap(); //Return the value for "isOpen" as a boolean
}
//
//Checks if the market is open now
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
        println!("{:?}", enddt);
        //If the duration since open is positve AND the duration until end negative then return true 
        if timesincestart.num_seconds() > 0 && timesinceend.num_seconds() < 0 {return true}else{return false};
    }
    return false;
}
//
//Returns a string for the date n weeks from now. Requires integers for weeks. Does not include today.
pub fn n_wks_from_now(n: i64) -> String{    
    Local::now().checked_add_signed(chrono::Duration::weeks(n)).unwrap().to_rfc3339_opts(SecondsFormat::Secs, false)
}
//
//Returns a string for the date n days from now. Requires integers for days. Does not include today.
pub fn n_days_from_now(n: i64) -> String{    
    Local::now().checked_add_signed(chrono::Duration::days(n)).unwrap().to_rfc3339_opts(SecondsFormat::Secs, false)
}
