// Watchlist utility functions
use tdameritradeclient::{TDAClient};

//Pushes copies of json values, representing the watchlists the bot is set to watch, to a vector init somewhere else
pub fn get_filtered_watchlists(client: &TDAClient, accountid: &str,  filteredwatchlists: &mut Vec<serde_json::Value>){
    let unfilteredwatchlists: serde_json::Value = client.get_watchlists(accountid);//Get all watchlists
    unfilteredwatchlists.as_array().unwrap().iter().for_each(|x|{      //For each value in the array(The first value is the json response is an array)
        println!("{:?}", x["name"].as_str().unwrap());    //Print the names of watchlists
        match x["name"].as_str().unwrap() { //Check if the name == A watchlist assigned to a bot
            "default"|"Bullish Market" =>  {
                filteredwatchlists.push(x.clone()); //Push value to filtered vector 
                println!("Pushing: {:?} to filtered watchlist vector", x["name"].as_str().unwrap())
                },
            "Quotes" =>   {
                filteredwatchlists.push(x.clone()); //Push value to a possible second filtered vector(example currently use same vector)
                println!("Pushing: {:?} to filtered watchlist vector", x["name"].as_str().unwrap())
                },
            _ => println!("{:?}", "List Did not match"),
        }
        
    });
}