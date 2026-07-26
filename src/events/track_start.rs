
use std::sync::Arc;

use serenity::{all::{ChannelId, Context, GuildId, Http}, async_trait};
use songbird::{Event, events::EventHandler as SongbirdEventHandler};
use songbird::{EventContext};

use crate::{AuxMetadataKey, common::queue::{self, get_track_metadata}};


pub struct SongStartNotifier {
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
    pub ctx: Context,
    pub http: Arc<Http>
}

#[async_trait]
impl SongbirdEventHandler for SongStartNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in track_list.iter() {
                println!(
                    "Track {:?} in guild {} started with state {:?}",
                    handle.uuid(),
                    self.guild_id,
                    state.playing
                );
                let track_metadata = get_track_metadata(&self.ctx, &self.guild_id).await;
                let _ = self.channel_id.say(&self.http, format!("Playing: {}", track_metadata.title)).await;
            }
        }
        None
    }
}
