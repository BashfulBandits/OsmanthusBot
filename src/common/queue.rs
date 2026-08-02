use std::{error::Error, time::Duration};

use serenity::all::{Context, GuildId};
use songbird::input::{AudioStreamError, AuxMetadata};

use crate::{AuxMetadataKey, TrackMetadata, results::{CommandError, CommandSuccess}};


pub async fn add_track_metadata(ctx: &Context, guild_id: GuildId, metadata: Result<AuxMetadata, AudioStreamError>) -> Result<(), CommandError> {
    let data = ctx.data.read().await;
    let map_lock = data.get::<AuxMetadataKey>().unwrap().clone();
    drop(data); // release the outer lock before taking the inner one
    
    let metadata = match metadata {
        Ok(okay) => okay,
        Err(_) => return Err(CommandError::InvalidURL),
    };

    let track_metadata = TrackMetadata {
        title: metadata.title.unwrap_or("No Title".to_string()),
        creator: metadata.channel.unwrap_or("No Channel".to_string()),
        duration: metadata.duration.unwrap_or(Duration::new(0, 0)),
        url: metadata.source_url.unwrap_or("Unknown Url".to_string()),
    };

    match map_lock.write() {
        Ok(mut map) => {
            map.entry(guild_id).or_insert_with(Vec::new).push(track_metadata);
            Ok(())
        }
        Err(_) => Err(CommandError::Other),
    }
}

pub async fn remove_first_track_metadata(ctx: &Context, guild_id: GuildId) {
    let data = ctx.data.read().await;
    let map_lock = data.get::<AuxMetadataKey>().unwrap().clone();
    drop(data); // release the outer lock before taking the inner one

    let mut map = map_lock.write().expect("IDk");
    let map_entry = map.entry(guild_id).or_insert_with(Vec::new);

    if !map_entry.is_empty() { map_entry.remove(0); }
}

pub async fn remove_all_track_metadata(ctx: &Context, guild_id: GuildId) {
    let data = ctx.data.read().await;
    let map_lock = data.get::<AuxMetadataKey>().unwrap().clone();
    drop(data); // release the outer lock before taking the inner one

    let mut map = map_lock.write().expect("IDK");
    map.entry(guild_id).or_insert_with(Vec::new).clear();
}

pub async fn get_track_metadata(ctx: &Context, guild_id: &GuildId) -> Vec<TrackMetadata> {
    let data = ctx.data.read().await;
    let map_lock = data.get::<AuxMetadataKey>().unwrap().clone();
    drop(data); // release the outer lock before taking the inner one
                            
    let metadata = {
        let map = map_lock.read().expect("IDK");
        match map.get(guild_id) {
            Some(metadata) => metadata.clone(),
            None => vec![],
        }
    };

    metadata.to_vec()
}
