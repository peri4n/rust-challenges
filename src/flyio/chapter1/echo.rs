use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Request {
    src: String,
    dest: String,
    body: RequestPayload,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RequestPayload {
    Echo { msg_id: u64, echo: String },
    Init { msg_id: u64, node_id: String, node_ids: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Response {
    src: String,
    dest: String,
    body: ResponsePayload,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ResponsePayload {
    EchoOk {
        msg_id: u64,
        in_reply_to: u64,
        echo: String,
    },
    InitOk { in_reply_to: u64 },
}

fn main() {

    let mut lines = io::stdin().lock().lines();

    let mut node_identifier = String::new();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let request: Request = serde_json::from_str(&line).unwrap();
        match request.body {
            RequestPayload::Echo { msg_id, echo } => {
                println!(
                    "{}",
                    serde_json::to_string(&Response {
                        src: node_identifier.clone(),
                        dest: request.src,
                        body: ResponsePayload::EchoOk {
                            msg_id,
                            in_reply_to: msg_id,
                            echo,
                        },
                    })
                    .unwrap()
                );
            }
            RequestPayload::Init { msg_id, node_id, node_ids: _ } => {
                node_identifier = node_id;
                println!(
                    "{}",
                    serde_json::to_string(&Response {
                        src: request.dest,
                        dest: request.src,
                        body: ResponsePayload::InitOk { in_reply_to: msg_id },
                    })
                    .unwrap()
                );

            }
        }
    }
}
