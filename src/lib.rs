use tdameritradeclient::{TDAClient,OptionChain};

fn get_keys(response: &serde_json::Value, firstkey: &str,mut  keysvec: Vec<(String, String)>) {
    //Get Date keys:  use the first key ie; "putExpDateMap" to get the jsonValue down one level
    let incomingjsonvalue: &serde_json::Value = &response.get(firstkey).unwrap();
    /*Copy the map out of the jsonValue "putexpdatemap" into empty map "holdingmap" 
    so we can use .keys() and .values() methods */
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
           
    
}
fn append_delta_to_vector(response: &serde_json::Value, firstkey: &str, keysvec: Vec<(&str, &str)>) {
    //let (datekey, strikekey): (String, String) = keysvec[0].clone();
    //let (datekey, strikekey): (&str, &str) = (datekey.as_str(), strikekey.as_str());
    //println!("{:?}", datekey);
}
pub fn get_option_by_delta(client: &TDAClient, symbol: &str, strikes: u8, callput: &str, interval: f64, fromdate: &str, todate: &str, delta: f64 ){
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
    let firstkey: &str = "putExpDateMap";
    let keysvec: Vec<(String, String)> = Vec::new();
    get_keys(jsonoptionchain, firstkey, keysvec);
    //append_delta_to_vector(jsonoptionchain, firstkey, keysvec);
    
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

fn prettyprint(toprint: &serde_json::Value) {    
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}

fn titleprint(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}