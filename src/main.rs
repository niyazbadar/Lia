use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

mod bin;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let prefix = "LIA";

        let msg_content = msg.content.to_uppercase();

        let dm = msg.is_private() && !msg.is_own(&ctx.cache);

        if msg_content.starts_with(prefix) || dm{

            let msg_content = match !dm || msg_content.starts_with(prefix) {
                true => {
                    let msg_content
                        = String::from(msg_content.strip_prefix(prefix)
                        .unwrap());
                    msg_content
                },
                false => msg_content
            };

            match msg_content.split_whitespace().next().unwrap_or("NONE") {
                "HELLO" | "HI" | "HEYA" | "HENLO" | "HELLOW" | "HENLOW" | "HELO" | "HEWWO" => {
                    let name 
                        = msg.author_nick(&ctx.http)
                        .await
                        .unwrap_or(msg.author.name.clone());

                    bin::message::reply(ctx, msg, format!("henlo {} ^~^", name)).await; 
                }, 
                
                "PING" => {
                    bin::stats::ping(ctx, msg).await;
                },

                "STATUS" | "STAT" | "STATS" => {
                    bin::stats::status(ctx, msg).await;
                },

                "HELP" | "HALP" => {
                    let msg_content = match msg_content.split_whitespace().next().unwrap() {
                        prefix => msg_content.strip_prefix(prefix).unwrap().split_whitespace().next().unwrap_or_default(),
                    };
                    
                   bin::stats::help(ctx, msg, msg_content).await;
                },

                "NONE" | "?" => {
                    bin::message::reply(ctx, msg, format!("Yeah?")).await;
                },
                
                _ => {
                    bin::message::say(ctx, msg, String::from("No such command found")).await;
                }, // if typo will warn the user
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = bin::get_token().await;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = &client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}