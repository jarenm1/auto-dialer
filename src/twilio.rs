use reqwest::Client;
use std::error::Error;

use crate::utils;

//call error handling
pub enum CallReponse {
    ClientError,
    Success,
}

pub async fn prep_twilio(file_path: String) {
    let from_number = std::env::var("FROM_NUMBER").expect("from number expected in env");
    
    let records: Vec<String> = utils::read_csv(&file_path).unwrap();

    for to_number in records {
        //currently routes to my voip ws 
        //needs to be changed later
        let twiml = (r#"<Response><Connect><Stream name="Test" url="wss://voip.jarenmchugh.com/ws" /></Connect></Response>"#).to_string();

        //need to add better error handling later 
        match make_call(&to_number, &from_number, &twiml).await {
            Ok(CallReponse::Success) => println!("Call to {} successful", to_number),
            Ok(CallReponse::ClientError) => println!("Client error to: {}", to_number),
            Err(e) => eprintln!("Error making call: {}", e),
        }
    }
    
}

async fn make_call(to: &str, from: &str, twiml: &str) -> Result<CallReponse, Box<dyn Error>> {
    
    // need to change this later
    // currently takes from .env
    let account_sid = std::env::var("ACCOUNT_SID").expect("account sid expected");
    let auth_token = std::env::var("AUTH_TOKEN").expect("auth token expected");

    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{account_sid}/Calls.json");

    let client = Client::new();
    let parameters = [("To", to), ("From", from), ("Twiml", twiml)];

    let reponse = client
        .post(&url)
        .basic_auth(account_sid, Some(auth_token))
        .form(&parameters)
        .send()
        .await?;

    if reponse.status().is_client_error() {
        println!(
            "Client error: {}, reponse: {}",
            reponse.status(),
            reponse.text().await?
        );
        return Ok(CallReponse::ClientError);
    }

    reponse.error_for_status()?;

    Ok(CallReponse::Success)
}
