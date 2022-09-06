#![no_main]

use serenity::{
    model::channel::Message,
    prelude::*,     
    builder::{CreateEmbed, CreateEmbedFooter}, 
    utils::{ Colour, MessageBuilder, EmbedMessageBuilding}
};

use super::message; // as bin name is both in bin folder and bin.rs 
use super::save;

pub async fn profile(ctx: Context, msg:Message) {
    let path = String::from("Profiles");
    let name = msg.author.id.to_string();
    let profile = save::load(path, name).await;

    let mut embed = CreateEmbed::default();

    let title = &msg.author.name;

    let title = format!("{title}'s Profile");

    embed.title(title);

    embed.thumbnail(msg.author.avatar_url().unwrap());

    let money = profile.money;

    let description = format!("
:coin: {money}
    ");

    embed.description(description);

    embed.colour(msg.author.accent_colour.unwrap_or(Colour::from_rgb(255, 255, 255)));

    message::embeded(ctx, msg, embed).await;
}