use core::f32;
use std::collections::HashMap;
use std::io::Read;
use std::time::{Duration, Instant};
use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};

use base64::{Engine as _, engine::general_purpose::STANDARD as base64};
use tokio::process;
use whisper_rs::{FullParams, WhisperContext, WhisperContextParameters, WhisperState};

use crate::generate_response::{self, llm_response};
use crate::utils;

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

pub async fn handle_socket(mut socket: WebSocket) {
    let (mut sender, mut reciever) = socket.split();
    let mut audio_vector: Vec<i16> = Vec::new();

    while let Some(Ok(message)) = reciever.next().await {
        match message {
            Message::Text(text) => {

                match serde_json::from_str::<TwilioMessage>(&text) {
                    Ok(twilio_msg) => {
                        
                        match twilio_msg {
                            TwilioMessage::ConnectedMessage { protocol, version } => {
                                println!("Connected, protocol: {}, version: {}", protocol, version);
                            },
                            TwilioMessage::StartMessage { sequence_number, start } => {
                                println!("Started connection; streamSid: {}, sequence_number: {}, callSid: {}", start.stream_sid, sequence_number, start.call_sid);
                            },
                            TwilioMessage::MediaMessage { sequence_number, media, stream_sid } => {
                                //collect audio data into vec, send vec when the person stops
                                //talking
                                //
                                let bytes = base64.decode(media.payload).unwrap();
                                
                                for byte in bytes {
                                    println!("{}", byte);
                                    if byte < 250 {
                                        audio_vector.push(byte.into());
                                    }
                                }
                                
                                
                            },
                            TwilioMessage::DtmfMessage { stream_sid, sequence_number, dtmf } => {
                                println!("Digit: {}, pressed from {}, streamSid: {}, sequenceNumber: {}", dtmf.digit, dtmf.track, stream_sid, sequence_number);
                            },
                            TwilioMessage::MarkMessage { stream_sid, sequence_number, mark } => {
                                println!("Message finished playing: {}, streamSid: {}, sequenceNumber: {}", mark.name, stream_sid, sequence_number);

                            },
                            TwilioMessage::StopMessage { stream_sid } => {
                                println!("{} stopped", stream_sid);
                                write_wav_file(&audio_vector);
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

fn write_wav_file(samples: &[i16]) -> Result<(), hound::Error> {

    let spec = hound::WavSpec {
        bits_per_sample: 16,
        channels: 1,
        sample_format: hound::SampleFormat::Int,
        sample_rate: 8000,
    };

    let mut writer = hound::WavWriter::create("newwave.wav", spec)?;

    for &sample in samples {
        writer.write_sample(sample)?;
    }

    writer.finalize()?;
    Ok(())
}

