#![no_main]

use serenity::{
    model::channel::Message,
    prelude::*,     
    builder::{CreateEmbed, CreateEmbedFooter}, 
    utils::Colour,
};

use chrono;

use super::{message, save}; // as bin name is both in bin folder and bin.rs 

pub async fn profile(ctx: Context, msg:Message) {
    let path = String::from("Profiles");
    let user = match msg.mentions.len() > 0{
        true => msg.mentions[0].clone(),
        false => msg.author.clone()
    };
    let name = user.id.to_string();

    let mut data = save::load(path.clone(), name.clone()).await;

    let mut embed = CreateEmbed::default();

    embed.title(user.tag());

    embed.thumbnail(user.avatar_url().unwrap());

    let money = data.get_sub_class("profile", "money").await.parse().unwrap_or(0);
    let rep = data.get_sub_class("profile", "rep").await.parse().unwrap_or(0);
    save::save(path.clone(), name.clone(), &data).await;

    let description = format!("
:coin: {money} coins

:rosette: {rep} reputaion points
    ");

    embed.description(description);

    embed.colour(Colour::from_rgb(255, 255, 255));

    message::embeded(ctx, msg, embed).await;
}

pub async fn rep (ctx: Context, msg:Message) {
    let path = String::from("Profiles");
    let name = msg.author.id.to_string();
    let mut data = save::load(path.clone(), name.clone()).await; //access author profile

    if data.get_sub_class("dates", "rep_date").await != chrono::Utc::now().date().to_string() { //date check
        if msg.mentions.len() > 0 {
            if &msg.author != &msg.mentions[0] { //checking if author is self repping 
                    let reciever_name = &msg.mentions[0].id.to_string();
                    let mut reciever_data = save::load(path.clone(), reciever_name.clone()).await; //access reviever profile
        
                    let rep = reciever_data.get_sub_class("profile", "rep").await.parse().unwrap_or(0) + 1;
                    reciever_data.index.get_mut("profile").unwrap().insert(String::from("rep"), format!("{rep}"));

                    let rep_date = chrono::Utc::now().date().to_string();
                    
                    data.index.get_mut("dates").unwrap().insert(String::from("rep_date"), rep_date);
        
                    save::save(path.clone(), name, &data).await;
                    save::save(path, reciever_name.clone(), &reciever_data).await;
        
                    message::say(ctx.clone(), msg.clone(), format!(":rosette: {} has given a reputaion point to {}", &msg.author.name, &msg.mentions[0].mention().to_string())).await;
            }
            else {
                message::say(ctx.clone(), msg.clone(), format!("You can not give youself a reputation")).await;
            }
        }
        else {
            rep_yes(ctx, msg).await;
        }
    }
    else {
        rep_no(ctx, msg).await;
    }
}

async fn rep_no(ctx: Context, msg:Message) {
    let mut embed = CreateEmbed::default();

    embed.title(":rosette: No Reputation point available");

    embed.description("You have already given a reputation point today! Please try again tomorrow. Reputation point allowance resets daily at 00:00 UTC");

    let mut footer = CreateEmbedFooter::default();

    footer.icon_url(msg.author.avatar_url().unwrap());
    footer.text(msg.author.tag());

    embed.set_footer(footer);

    embed.colour(Colour::from_rgb(255, 255, 255));

    message::embeded(ctx, msg, embed).await;
}

async fn rep_yes(ctx: Context, msg:Message) {
    let mut embed = CreateEmbed::default();

    embed.title(":rosette: Reputation point available");

    embed.description("You can give someone a reputation point");

    let mut footer = CreateEmbedFooter::default();

    footer.icon_url(msg.author.avatar_url().unwrap());
    footer.text(msg.author.tag());

    embed.set_footer(footer);

    embed.colour(Colour::from_rgb(255, 255, 255));

    message::embeded(ctx, msg, embed).await;
}