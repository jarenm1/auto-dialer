use csv::Reader;
use std::error::Error;


//csv opener
//has no error handling
pub fn read_csv(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut numbers_vector: Vec<String> = Vec::new();
    let mut reader = Reader::from_path(file_path)?;
    for result in reader.records() {
        let number = result?.get(0).unwrap().to_string();
        numbers_vector.push(number);
    }
    Ok(numbers_vector)
}

