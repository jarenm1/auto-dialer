# README

Uses twiml because it was the first voip provider I found. 

# !!!
in src/twilio.rs there is [twiml](https://www.twilio.com/docs/voice/twiml/stream) that needs to be changed. Currently links to where I was hosting the websocket for twiml. 

## how to env

env has 4 values:

1. ACCOUNT_SID is the twilio account sid
2. AUTH_TOKEN is the twilio auth token
3. FROM_NUMBER is the twilio from number
4. VALID_TOKEN is the 'token' that must match the token provided from the upload frontend

## Requirements

just needs cargo to run as well as a valid env
[rustup install](https://www.rust-lang.org/tools/install)

