use base64::{
    alphabet::{self, Alphabet},
    engine::{self, general_purpose},
    Engine,
};
use csv::Reader;
use std::error::Error;

pub fn read_csv(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut numbers_vector: Vec<String> = Vec::new();
    let mut reader = Reader::from_path(file_path)?;
    for result in reader.records() {
        let number = result?.get(0).unwrap().to_string();
        numbers_vector.push(number);
    }
    Ok(numbers_vector)
}

pub async fn transcribe_audio(audio_payload: String) {
    let vector = general_purpose::STANDARD.decode(audio_payload).unwrap();
}
