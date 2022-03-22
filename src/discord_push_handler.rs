use std::{env, str::FromStr};
use std::sync::Arc;

use tokio::sync::mpsc::Receiver;

use futures::StreamExt;

use serenity::{
    model::{channel::Message, gateway::Ready},
    model::id::ChannelId,
    http::Http,
    prelude::*,
};

use tokio_stream::wrappers::ReceiverStream;

use serde_json::{Value, Map, to_vec};

use crate::prototypes::MessageToChannel;

struct Handler;

// pub async fn push_to_channel(channel_id: u64, message: String) {
//     let token = "discord-bot-token".to_string();

//     let mut client =
//         Client::builder(&token).await.expect("Err creating client");

//     let http_client = Http::new_with_token(&token);

//     let channel = ChannelId(channel_id);

//     let resp = channel.send_message(&http_client, |m| {
//         m.content("test");

//         m.embed(|mut e| {
//             e.title("This is an embed");
//             e.description("With a description");

//             e
//         });

//         m
//     }).await;

//     if let Err(reason) = resp {
//             println!("ERROR WHY {:?}", reason);
//         }
// }


pub async fn channel_handle(rx: Receiver<MessageToChannel>) {
    let token = "fsdf".to_string();
    let http_client = &Http::new_with_token(&token);

    let streeeem: ReceiverStream<_> = rx.into();

    // println!("BEFORE CALL");

    streeeem.for_each_concurrent(3, |message| async move {
        // println!("CHANNEL {:?}", message.channel_id);
        let channel = ChannelId(message.channel_id);
        // println!("IN CALL");
        let res = channel.send_message(http_client, |m| {
            m.content(message.content);
            m.embed(|mut e| {
                e.title("This is an embed");
                e.description("With a description");
    
                e
            });
    
            m
        }).await;
        println!("RESPONSE {:?}", res);

    }).await;
}