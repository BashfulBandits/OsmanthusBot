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
    map.entry(guild_id).or_insert_with(Vec::new).remove(0);
}

pub async fn get_track_metadata(ctx: &Context, guild_id: &GuildId) -> TrackMetadata {
    let data = ctx.data.read().await;
    let map_lock = data.get::<AuxMetadataKey>().unwrap().clone();
    drop(data); // release the outer lock before taking the inner one
                            
    let title = {
        let map = map_lock.read().expect("IDk");
        map.get(guild_id)
            .unwrap()
            .first()
            .unwrap()
            .title
            .clone() // <-- own the String, don't borrow it
    };

    TrackMetadata { title }
}
