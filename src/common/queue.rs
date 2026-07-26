use serenity::all::{Context, GuildId};

use crate::{AuxMetadataKey, TrackMetadata};


pub async fn add_track_metadata(ctx: &Context, guild_id: GuildId, metadata: TrackMetadata) {
    let data = ctx.data.read().await;
    let map_lock = data.get::<AuxMetadataKey>().unwrap().clone();
    drop(data); // release the outer lock before taking the inner one

    let mut map = map_lock.write().expect("IDk");
    map.entry(guild_id).or_insert_with(Vec::new).push(metadata);
}

pub async fn remove_track_metadata(ctx: &Context, guild_id: GuildId) {
    let data = ctx.data.read().await;
    let map_lock = data.get::<AuxMetadataKey>().unwrap().clone();
    drop(data); // release the outer lock before taking the inner one

    let mut map = map_lock.write().expect("IDk");
    map.entry(guild_id).or_insert_with(Vec::new).pop();
}
