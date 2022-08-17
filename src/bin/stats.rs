#![no_main]

use serenity::{
    model::channel::Message,
    prelude::*,     
    builder::{CreateEmbed, CreateEmbedFooter}, 
    utils::{ Colour, MessageBuilder, EmbedMessageBuilding}
};

use super::message; // as bin name is both in bin folder and bin.rs 

async fn ping_string(ctx: Context, msg:Message) -> String{
    let msg_timestamp = msg.timestamp; // timestamp of input message
    
    let ref_msg = message::say(ctx.clone(), msg.clone(), String::from("calculating Latency")).await; // displays a reference message in discord 

    let ref_timestamp = ref_msg.timestamp; //get the timestamp of the referance message

    let latency = ref_timestamp.unix_timestamp() - msg_timestamp.unix_timestamp(); // calculation difference in seconds

    message::delete(ctx, ref_msg).await; // deletes reference message

    format!("🏓 Pong! | time taken: **{:?}s**", latency) //returnes string
}

pub async fn ping(ctx: Context, msg:Message){
    let mut embed = CreateEmbed::default();
    embed.description(ping_string(ctx.clone(), msg.clone()).await);
    embed.colour(Colour::from_rgb(255, 255, 255));

    message::embeded(ctx, msg, embed).await;
}

pub async fn status(ctx: Context, msg:Message){
    let mut embed = CreateEmbed::default();

    embed.title("Status Report");

    embed.thumbnail(ctx.http.get_current_user().await.unwrap().avatar_url().unwrap());

    embed.colour(Colour::from_rgb(255, 255, 255));

    let version = format!{"Version: {}", env!("CARGO_PKG_VERSION")};
    
    let ping = ping_string(ctx.clone(), msg.clone()).await;

    let discription = format!("{version}\n\n{ping}");
    embed.description(discription);

    message::embeded(ctx, msg, embed).await;
}

pub async fn help(ctx: Context, msg:Message, text: &str){
    match text {
        "HELLO" | "HI" | "HEYA" | "HENLO" | "HELLOW" | "HENLOW" | "HELO" | "HEWWO" => {
            help_hi(ctx, msg).await
        },

        "STATUS" | "STAT" | "STATS"  => {
            help_stats(ctx, msg).await
        },

        "PING" => {
            help_ping(ctx, msg).await
        },

        _ => help_commands(ctx, msg).await
    }
}

pub async fn help_commands(ctx: Context, msg:Message){
    let mut embed = CreateEmbed::default();

    embed.title("Lia Commands");

    embed.colour(Colour::from_rgb(255, 255, 255));

    let lia = MessageBuilder::new().push_named_link("`lia`", msg.link() + " 'prefix for commands (not required in bot dm)'").build();
    let hi = MessageBuilder::new().push_named_link("`hi`", msg.link() + " 'say hi to Lia'").build();
    let stats = MessageBuilder::new().push_named_link("`stats`", msg.link() + " 'get bot dev stats'").build();
    let ping = MessageBuilder::new().push_named_link("`ping`", msg.link() + " 'get response latency'").build();

    let description = format!("
:pushpin: **prefix:** {lia}

:pen_ballpoint: **Standard Commands:**
➜ {hi}
    
:gear: **Developer Commands:** 
➜ {stats} {ping}
    ");

    embed.description(description);

    let mut footer = CreateEmbedFooter::default();

    footer.text("Hover over the command or type 'help [command]' to get info on that command");

    embed.set_footer(footer);

    message::embeded(ctx, msg, embed).await;
}

pub async fn help_hi(ctx: Context, msg:Message){
    let mut embed = CreateEmbed::default();

    embed.title("Standard Command: 'hi'");

    embed.description("Say hi to Lia");

    let mut footer = CreateEmbedFooter::default();

    footer.text("lia [hello | hi | heya | henlo | hellow | henlow | helo | hewwo]");

    embed.set_footer(footer);

    embed.colour(Colour::from_rgb(255, 255, 255));

    message::embeded(ctx, msg, embed).await;
}

pub async fn help_stats(ctx: Context, msg:Message){
    let mut embed = CreateEmbed::default();

    embed.title("Developer Command: 'stats'");

    embed.description("Get bot development status");

    let mut footer = CreateEmbedFooter::default();

    footer.text("lia [stats | status | stat]");

    embed.set_footer(footer);

    embed.colour(Colour::from_rgb(255, 255, 255));

    message::embeded(ctx, msg, embed).await;
}

pub async fn help_ping(ctx: Context, msg:Message){
    let mut embed = CreateEmbed::default();

    embed.title("Developer Command: 'ping'");

    embed.description("Get response latency");

    let mut footer = CreateEmbedFooter::default();

    footer.text("lia ping");

    embed.set_footer(footer);

    embed.colour(Colour::from_rgb(255, 255, 255));

    message::embeded(ctx, msg, embed).await;
}