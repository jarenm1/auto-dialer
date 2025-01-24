
use std::result::Result;
use serde::Serialize;
use crate::utils;

#[derive(Serialize)]
struct VonageCall {
    ncco: Ncco,
    to: To,
    from: From,
}

#[derive(Serialize)]
struct To {
    #[serde(rename = "type")]
    call_type: String,
    #[serde(rename = "number")]
    to_number: String,
}

#[derive(Serialize)]
struct From {
    #[serde(rename = "type")]
    call_type: String,
    #[serde(rename = "number")]
    from_number: String,
}

#[derive(Serialize)]
struct Ncco {
    action: String,
    endpoint: EndPoint,

}

#[derive(Serialize)]
struct EndPoint {
    #[serde(rename = "type")]
    ep_type: String,
    uri: String,
    content_type: String,
    headers: Option<Headers>,
}

#[derive(Serialize)]
struct Headers {
}

pub enum VonageResponse {
    Success,
    ClientError,
}

pub async fn prep_call(file_path: String, from_number: String, uri: String) {
    let records: Vec<String> = utils::read_csv(&file_path).unwrap();

    for to_number in records {
        match make_call(&from_number, &to_number, &uri).await {
            Ok(VonageResponse::Success) => println!("Call to {} successful", to_number),
            Ok(VonageResponse::ClientError) => println!("Client error to: {}", to_number),
            Err(e) => eprintln!("Error making call: {}", e),
        }
    }
}

pub async fn make_call(from_number: &str, to_number:&str, uri: &str) -> Result<VonageResponse, Box<dyn std::error::Error>> {

    let json = VonageCall {
        ncco: Ncco {
            action: "connect".to_string(),
            endpoint: EndPoint {
                ep_type: "websocket".to_string(),
                uri: uri.to_string(),
                content_type: "audio/l16;rate=8000".to_string(),
                headers: None,
            },
        },
        to: To {
            call_type: "phone".to_string(),
            to_number: to_number.to_string(),
        },
        from: From {
            call_type: "phone".to_string(),
            from_number: from_number.to_string(),
        }
    };

    let client = reqwest::Client::new();
    let jwt = "";
    let header = format!("Bearer <{}>", jwt);
    let response = client.post("https://api.nexmo.com/v1/calls/")
        .json(&json)
        .header("Authorization", header)
        .send()
        .await?;
    
    if response.status().is_client_error() {
        
        return Ok(VonageResponse::ClientError);
    }

    Ok(VonageResponse::Success)
}
