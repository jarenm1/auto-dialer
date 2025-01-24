use core::f32;
use std::{collections::HashMap, net::SocketAddr};
use axum::extract::ws::{Message, WebSocket};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Start {
    #[serde(rename = "accountSid")]
    account_sid: String,
    #[serde(rename = "streamSid")]
    stream_sid: String,
    #[serde(rename = "callSid")]
    call_sid: String,
    tracks: Vec<String>,
    #[serde(rename = "mediaFormat")]
    media_format: MediaFormat,
    #[serde(rename = "customParameters")]
    #[serde(flatten)]
    custom_parameters: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Deserialize)]
struct MediaFormat {
    encoding: String,
    #[serde(rename = "sampleRate")]
    sample_rate: String,
    channels: String,
}

#[derive(Deserialize, Serialize)]
struct Media {
    track: Option<String>,
    chunk: Option<String>,
    timestamp: Option<String>,
    payload: String,
}

#[derive(Deserialize)]
struct Dtmf {
    track: String,
    digit: String,
}

#[derive(Deserialize, Serialize)]
struct Mark {
    name: String,
}

#[derive(Deserialize)]
#[serde(tag = "event")]
enum TwilioMessage {
    #[serde(rename = "connected")]
    ConnectedMessage {
        protocol: String,
        version: String,
    },
    #[serde(rename = "start")]
    StartMessage {
        #[serde(rename = "sequenceNumber")]
        sequence_number: String,
        start: Start,
    },
    #[serde(rename = "media")]
    MediaMessage {
        #[serde(rename = "sequenceNumber")]
        sequence_number: String,
        media: Media,
        #[serde(rename = "streamSid")]
        stream_sid: String,
    },
    #[serde(rename = "dtmf")]
    DtmfMessage {
        #[serde(rename = "streamSid")]
        stream_sid: String,
        #[serde(rename = "sequenceNumber")]
        sequence_number: String,
        dtmf: Dtmf,
    },
    #[serde(rename = "mark")]
    MarkMessage {
        #[serde(rename = "streamSid")]
        stream_sid: String,
        #[serde(rename = "sequenceNumber")]
        sequence_number: String,
        mark: Mark,
    },
    #[serde(rename = "stop")]
    StopMessage {
        #[serde(rename = "streamSid")]
        stream_sid: String,
    },
}


//not used because no message is being send back yet
#[derive(Serialize)]
enum TwilioResponse {
    #[serde(rename = "media")]
    MediaMessage {
        #[serde(rename = "streamSid")]
        stream_sid: String,
        media: Media,
    },
    #[serde(rename = "mark")]
    MarkMessage {
        #[serde(rename = "streamSid")]
        stream_sid: String,
        mark: Mark,
    },
    #[serde(rename = "clear")]
    ClearMessage {
        #[serde(rename = "streamSid")]
        stream_sid: String,
    }
}

//no concurrency currently
//only allows for 1 call at a time
pub async fn handle_socket(mut socket: WebSocket) {
    let (mut sender, mut reciever) = socket.split();
    let mut mulaw_data: Vec<u8> = Vec::new();

    while let Some(Ok(message)) = reciever.next().await {
        match message {
            Message::Text(text) => {

                match serde_json::from_str::<TwilioMessage>(&text) {
                    Ok(twilio_msg) => {
                        
                        match twilio_msg {
                            TwilioMessage::ConnectedMessage { protocol, version } => {
                                println!("Connected, protocol: {}, version: {}", protocol, version);
                                //some dashboard logic here maybe
                                //https://www.twilio.com/docs/voice/media-streams/websocket-messages#connected-message
                            },
                            TwilioMessage::StartMessage { sequence_number, start } => {
                                println!("Started connection; streamSid: {}, sequence_number: {}, callSid: {}", start.stream_sid, sequence_number, start.call_sid);
                                //https://www.twilio.com/docs/voice/media-streams/websocket-messages#start-message
                            },
                            TwilioMessage::MediaMessage { sequence_number, media, stream_sid } => {
                                //https://www.twilio.com/docs/voice/media-streams/websocket-messages#media-message
                                // write audio stuff
                                todo!()
                                
                            },
                            TwilioMessage::DtmfMessage { stream_sid, sequence_number, dtmf } => {
                                println!("Digit: {}, pressed from {}, streamSid: {}, sequenceNumber: {}", dtmf.digit, dtmf.track, stream_sid, sequence_number);
                                // nothing needed here for now
                                // https://www.twilio.com/docs/voice/media-streams/websocket-messages#dtmf-message
                            },
                            TwilioMessage::MarkMessage { stream_sid, sequence_number, mark } => {
                                println!("Message finished playing: {}, streamSid: {}, sequenceNumber: {}", mark.name, stream_sid, sequence_number);
                                //https://www.twilio.com/docs/voice/media-streams/websocket-messages#mark-message
                                //sends after audio is completed.
                                //
                            },
                            TwilioMessage::StopMessage { stream_sid } => {
                                println!("{} stopped", stream_sid);
                                //https://www.twilio.com/docs/voice/media-streams/websocket-messages#stop-message
                                //end message

                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error parsing twilio message: {}", e);
                    }
                }
            }
            Message::Close(_) => {
                println!("Client disconnected");
                break;
            }
            _ => {}
        }
    }
}

