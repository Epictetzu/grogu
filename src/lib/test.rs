
use grogu::*;
use tdameritradeclient::TDAClient;
use chrono::prelude::*;
use chrono::{FixedOffset, TimeZone};

fn is_market_open(client: &TDAClient){

    let today = Utc::now();
    let dtstring = today.to_rfc3339_opts(SecondsFormat::Secs, false);
    let todayshours = &client.get_todays_market_hours("OPTION");
    prettyprint(todayshours);
    
    
    
    
    //let (y, m ,d) = formatteddate;
    //let newyorktime = Eastern.ymd();
    println!("{:?}", dt);
}