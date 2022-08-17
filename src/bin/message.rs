#![no_main]

use serenity::{
    model::channel::Message, 
    prelude::*,
    builder::CreateEmbed,
};

pub async fn say(ctx: Context, msg:Message, text:String) -> Message { 
    match msg.channel_id.say(&ctx.http, text).await{
        Ok(msg) => msg,
        Err(why) => {println!("Error sending message: {:?}", why); msg}
    }
}

pub async fn reply(ctx: Context, msg:Message, text:String) -> Message{ 
    match msg.reply(ctx.http, text).await{
        Ok(msg) => msg,
        Err(why) => {println!("Error sending message: {:?}", why); msg}
    }
}

pub async fn reply_ping(ctx: Context, msg:Message, text:String) -> Message{ 
    match msg.reply_ping(ctx.http, text).await{
        Ok(msg) => msg,
        Err(why) => {println!("Error sending message: {:?}", why); msg}
    }
}

pub async fn edit(ctx: Context, mut msg:Message, text:String){
    if let Err(why) =  msg.edit(ctx.http, |m| m.content(text)).await{//edit the referancy message to display latancy value
        println!("Error sending message: {:?}", why);
    }
}

pub async fn delete(ctx: Context, msg:Message){
    if let Err(why) = msg.delete(ctx.http).await{
        println!("Error editing message: {:?}", why);
    }
}

pub async fn embeded(ctx: Context, msg: Message, embed: CreateEmbed) -> Message{
    match msg.channel_id.send_message(&ctx.http, |m| m.embed(|e| { *e = embed; e })).await{
        Ok(msg) => msg,
        Err(why) => {println!("Error sending message: {:?}", why); msg}
    }
}