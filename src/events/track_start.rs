
use std::sync::Arc;

use serenity::{all::{ChannelId, Context, CreateMessage, GuildId, Http}, async_trait};
use songbird::{Event, events::EventHandler as SongbirdEventHandler};
use songbird::{EventContext};

use crate::{AuxMetadataKey, common::{embeds, queue::{self, get_track_metadata}}};


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
                if let Err(why) = self.channel_id.send_message(&self.http, CreateMessage::new().add_embed(embeds::queue_embed(&self.ctx, &self.guild_id).await)).await {
                    println!("Error on queue embed message send in track start event: {why}")
                }
            }
        }
        None
    }
}
