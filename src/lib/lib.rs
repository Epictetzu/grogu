
use tdameritradeclient::{TDAClient,OptionChain};

fn get_keys(response: &serde_json::Value, firstkey: &str, mut keysvec: Vec<(String, String)>) -> Vec<(String, String)>{ /*Gets unknown keys from down the hierarchy of the response json value*/
    //use the first key ie; "putExpDateMap" to get the jsonValue down one level
    let incomingjsonvalue: &serde_json::Value = &response.get(firstkey).unwrap();
    /*Copy the map out of the jsonValue "putexpdatemap" into empty map "holdingmap" so we can use .keys() and .values() methods */ 
    let datesmap = incomingjsonvalue.as_object().unwrap().clone();
    //Create a new empty vectors of strings to hold dates and strike keys
    let mut datesvec: Vec<&str> = Vec::new();
    
    //Iterate down in the map to get the keys
    datesmap.keys().for_each(|date|{
        datesvec.push(&date);//Push keys for dates to vector holding dates  Right type for not returning the value
        datesmap[date].as_object().unwrap().keys().for_each(|strike| {
            keysvec.push(((date.as_str().to_string()), (strike.as_str().to_string()))); //Get the keys...as strings...then convert to strings so the keys will not have extra "\" in them. Put that in your brain and smoke it.
        });
    });
    datesvec.iter().for_each(|x| println!("Dates: {:?}", x));//Debug print */                   
    keysvec.iter().for_each(|x| println!("Strikes: {:?}", x));//Debug print */
           
    return keysvec;
}
fn append_delta(response: &serde_json::Value, firstkey: &str, keysvec: Vec<(String, String)>) -> Vec<(String, String, f64)>{
    let (datekey, strikekey): (String, String) = keysvec[0].clone();//Get Keys tuple out
    println!("append_delta debug\n {:?}{:?}", datekey, strikekey);
    let keysvec: Vec<(String, String, f64)> = keysvec.iter().map(|(x,y)| //For each iteration in keysvec we get the keys tuple and map the new tuple, with delta addded, over the old tuple. Then collect all of it back into keysvec
        (x.clone(),y.clone(), response[firstkey][&x][&y][0].get("delta").unwrap().as_f64().unwrap())).collect(); //Get the value from delta key. Unwrap it to get result. Get that as float. Unwrap that. 
    
    return keysvec;
}
pub fn get_option_by_delta(client: &TDAClient, symbol: &str, strikes: u8, callput: &str, interval: f64, fromdate: &str, todate: &str, targetdelta: f64)-> serde_json::Value{
    let jsonoptionchain: &serde_json::Value = &client.getoptionchain(
    &[
        OptionChain::Symbol(symbol),
        OptionChain::StrikeCount(strikes),
        OptionChain::ContractType(callput),
        OptionChain::Interval(interval),
        OptionChain::FromDate(fromdate),
        OptionChain::ToDate(todate),
    ]
    );
    let firstkey: &str = if callput == "PUT" {"putExpDateMap"} else if callput == "CALL" {"callExpDateMap"} else{panic!("Wrong CALL PUT input")}; //Function parameter callput must be a string of either "CALL" or "PUT"
    let mut keysvec: Vec<(String, String)> = Vec::new();//Init the empty vec to fill with keys
    keysvec = get_keys(jsonoptionchain, firstkey, keysvec);//Get all of the unknown key names from the response 
    let keysvec = append_delta(jsonoptionchain, firstkey, keysvec);
    //Append to the vector the difference between the target delta and current option's delta
    let mut keysdeltadiffvec: Vec<(String, String, f64, f64)> = append_deltas_difference(keysvec, targetdelta);
    keysdeltadiffvec.sort_by(|a,b| a.3.partial_cmp(&b.3).unwrap().reverse());//Sort vector by delta then reverse the vector
    keysdeltadiffvec.iter().for_each(|(x,y,z,a)|println!("get_option_by_delta debug\n {:?}{:?}{:?}{:?}", x,y,z,a));//Debug Print
    
    return jsonoptionchain[firstkey][&keysdeltadiffvec[0].0][&keysdeltadiffvec[0].1].clone();//Use all the painstakingly gathered and sorted keys to get the closest matching option and send it on its way
}
fn append_deltas_difference(keysvec: Vec<(String, String, f64)>, targetdelta: f64) -> Vec<(String, String, f64, f64)>{ //TODO
    // Append to the vector the difference between the target delta and current option's delta
    let keysdeltadiffvec: Vec<(String, String, f64, f64)> = keysvec.iter().map(|(x,y,z)|{
        if targetdelta < *z {(x.clone(),y.clone(),z.clone(), targetdelta % z)} else {(x.clone(),y.clone(),z.clone(), z % targetdelta)}}).collect();
    return keysdeltadiffvec;       
}
pub fn gather_options(client: &TDAClient, symbol: &str, strikes: u8, callput: &str, interval: f64, fromdate: &str, todate: &str){
    titleprint("Option Chain:");
    prettyprint(&client.getoptionchain(
    &[
        OptionChain::Symbol(symbol),
        OptionChain::StrikeCount(strikes),
        OptionChain::ContractType(callput),
        OptionChain::Interval(interval),
        OptionChain::FromDate(fromdate),
        OptionChain::ToDate(todate),
    ]));
}
pub fn prettyprint(toprint: &serde_json::Value) {    
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}
pub fn titleprint(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}
pub fn is_open(client: &TDAClient, market: &str) -> bool{
    let markethours: serde_json::Value = client.get_todays_market_hours(market);
    chrono::FixedOffset::west(0);
    return false;
}
fn get_eastern_time(){
    //chrono::FixedOffset::west(5 * 3600).
}