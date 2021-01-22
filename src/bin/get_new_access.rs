use std::env;
use tdameritradeclient::{auth::TDauth};

fn main() {
    //Set variables from environment
    let refresh = env::var("TD_REFRESH_TOKEN").unwrap();
    let clientid = env::var("TD_CLIENT_ID").unwrap();

    //Get only a new access token using refresh token
    let newtdauth = TDauth::new_fromrefresh(&refresh, &clientid, false);
    println!("New Authenications {:?}", newtdauth);
    //Get the tokens from the TDauth struct and place in easy to use tuple
    let t: (&str, &str) = newtdauth.gettokens().into();
    println!("Tokens: {}, {}", t.0, t.1);
    //Assigin token to first value in tuple(access token)
    let token = t.0;
    //Set envirornment variable to new access token
    env::set_var("TD_ACCESS_TOKEN", token);
    
    
    //Check if variable is set
    let accesstest = env::var("TD_ACCESS_TOKEN");
    println!("New Access Token {:?}", &accesstest);

    
    
}