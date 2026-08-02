use std::time::Duration;

use serenity::all::{ChannelId, Context, GuildId};
use songbird::{Event, TrackEvent, input::{AudioStreamError, AuxMetadata, Compose, YoutubeDl}};

use crate::{AuxMetadataKey, HttpKey, TrackMetadata, common::queue, events::{track_end::SongEndNotifier, track_start::SongStartNotifier}, results::{CommandError, CommandSuccess}};


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

pub async fn add_track_to_queue(ctx: &Context, guild_id: &GuildId, channel_id: &ChannelId, url: String) -> Result<CommandSuccess, CommandError> {
    let http_client = {
        let data = ctx.data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .expect("Guaranteed to exist in the typemap.")
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();


    if let Some(handler_lock) = manager.get(*guild_id) {
        let mut handler = handler_lock.lock().await;

        let mut src = YoutubeDl::new(http_client, url.clone());

        queue::add_track_metadata(ctx, *guild_id, src.aux_metadata().await).await?;

        let track_handle = handler.enqueue_input(src.into()).await;
        let _ = track_handle.add_event(
            Event::Track(TrackEvent::End),
        SongEndNotifier {
                guild_id: *guild_id,
                channel_id: *channel_id,
                ctx: ctx.clone(),
                http: ctx.http.clone(),
            },
        );
        let _ = track_handle.add_event(
            Event::Track(TrackEvent::Play),
        SongStartNotifier {
                guild_id: *guild_id,
                channel_id: *channel_id,
                ctx: ctx.clone(),
                http: ctx.http.clone(),
            },
        );

        println!("queue: {:?}", handler.queue());
        Ok(CommandSuccess::AddedToQueue)
    } else {
        Err(CommandError::BotNotInCall)
    }
}
