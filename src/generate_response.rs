//use chatgpt api to response to the audio from the caller. use elevenlabs to generate tts audio
//for the response. hope this all flows smoothly. If not work on reducing the reponse time. Maybe
//local host LLM so api is not needed????

//gpt needs a json as an input
//make a request with reqwest with json. change the prompt based on the websocket audio transcirpt.
pub async fn llm_response(user_text: String) -> String {
    todo!()
}

//use the prompt to get an elevenlabs voice response to pass to the websocket.
fn eleven_labs_request() {
    todo!()
}
