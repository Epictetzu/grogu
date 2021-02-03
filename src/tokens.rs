use std::env;
use std::fs::File;
use std::io::prelude::*;
use chrono::prelude::*;
use tdameritradeclient::{auth::TDauth};

//
//Get new access token
pub fn get_new_access() -> std::io::Result<()>{ 
    //Get variables from environment
    let refresh = env::var("TD_REFRESH_TOKEN").unwrap();
    let clientid = env::var("TD_CLIENT_ID").unwrap();

    //Get only a new access token using refresh token
    let newtdauth = TDauth::new_fromrefresh(&refresh, &clientid, false);
    println!("New Authenications {:?}", newtdauth);
    //Get the tokens from the TDauth struct and place in easy to use tuple
    let t: (&str, &str) = newtdauth.gettokens().into();
    //println!("Tokens: {}, {}", t.0, t.1);
    
    //Assigin token to first value in tuple(access token)
    let token = t.0;
    
    save_access(token)?;
    Ok(())
}
//
//Get new access token
pub fn renew_access() -> std::io::Result<()>{ 
    if is_access_expired()?{
    //Get variables from environment
    let refresh = env::var("TD_REFRESH_TOKEN").unwrap();
    let clientid = env::var("TD_CLIENT_ID").unwrap();

    //Get only a new access token using refresh token
    let newtdauth = TDauth::new_fromrefresh(&refresh, &clientid, false);
    println!("New Authenications {:?}", newtdauth);
    //Get the tokens from the TDauth struct and place in easy to use tuple
    let t: (&str, &str) = newtdauth.gettokens().into();
    //println!("Tokens: {}, {}", t.0, t.1);
    
    //Assigin token to first value in tuple(access token)
    let token = t.0;
    
    save_access(token)?;
    Ok(())
    } else { println!("{:?}", "token not expired"); Ok(()) }
}

//
//Saves string to savefile
fn save_access(input: &str)-> std::io::Result<()>{
    let mut file =File::create("saved_access_token.txt")?;
    let bytes = str::as_bytes(input);
    file.write_all(bytes)?; //This is not right
    Ok(())
}
//
//Reads saved values from file
pub fn read_access()->std::io::Result<String>{
    let mut file = File::open("saved_access_token.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
//
//Calls save the token expiration and then saves the whole principals JSON
pub fn save_principals(userprincipals: &serde_json::Value)-> std::io::Result<()>{
    save_token_expiration(userprincipals)?;
    let mut file =File::create("saved_principals.txt")?;//Create or clear the file
    let principalstr = serde_json::to_string_pretty(userprincipals)?; //Convert JSON to string
    let bytes = str::as_bytes(principalstr.as_str()); //Convert string to bytes
    file.write_all(bytes)?; //Write bytes to file
    Ok(())
}
//
//Saves the time that the users access token will expire
fn save_token_expiration(userprincipals: &serde_json::Value)-> std::io::Result<String>{
    
    let expstr: String = userprincipals["tokenExpirationTime"].as_str().unwrap().to_string(); //Should return w/o quotess but is not working right
    let mut file = File::create("saved_token_expiration.txt")?;
    let bytes = str::as_bytes(expstr.as_str());
    file.write_all(bytes)?;
    Ok(expstr)
}
//
//Reads the users principals from the saved file
fn read_principals()->std::io::Result<serde_json::Value>{
    let mut file = File::open("saved_principals.txt")?;//Load the file
    let mut contents = String::new();//Create an empty string buffer
    file.read_to_string(&mut contents)?;//Read file into string 
    let principalvalue = serde_json::from_str(&contents)?; //Convert string back to JSON
    Ok(principalvalue)
}
//
//Reads the saved time the users access token expires
fn read_token_expiration()->std::io::Result<String>{
    let mut file = File::open("saved_token_expiration.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

//
//Checks if the access token associated with this account is expired
//TODO: I had to make this function more complicated because I could not get 
// "tokenExpirationTime" without quotes for some buggy reason.
fn is_access_expired() -> std::io::Result<bool>{ 
    let expstr = read_token_expiration()?; //Should return w/o quotess but is not working right
    let expdt = DateTime::parse_from_str(expstr.as_str(), "%Y-%m-%dT%H:%M:%S%:z").unwrap();// Manually parse from string because I can't get the value w/o quotes and I might as well parse it here.
    let nowdt = Utc::now();
    let timediff = expdt.signed_duration_since(nowdt);
    if timediff.num_seconds() > 0 { Ok(false) } else { Ok(true) }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_save_access() -> std::io::Result<()>{
        save_access("test input &str")?;
        Ok(())
    }
    #[test]
    fn test_is_access_expired() -> std::io::Result<()> {
        is_access_expired()?;
        Ok(())
    }
    #[test]
    fn test_read_token_expiration() -> std::io::Result<()> {
        read_token_expiration()?;
        Ok(())
    }
    #[test]
    fn test_read_principals() -> std::io::Result<()> {
        read_principals()?;
        Ok(())
    }
    #[test]
    fn test_save_token_expiration() -> std::io::Result<()> {
        let principals = read_principals()?;
        save_token_expiration(&principals)?;
        Ok(())
    }
    #[test]
    fn test_read_access() -> std::io::Result<()> {
        read_access()?;
        Ok(())
    }
    #[test]
    fn test_renew_access() -> std::io::Result<()> {
        renew_access()?;
        Ok(())
    }
    #[test]
    fn test_get_new_access() -> std::io::Result<()> {
        get_new_access()?;
        Ok(())
    }
}