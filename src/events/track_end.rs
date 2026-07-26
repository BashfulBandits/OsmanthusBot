use std::sync::Arc;

use serenity::{all::{ChannelId, GuildId, Http}, async_trait};
use songbird::{Event, events::EventHandler as SongbirdEventHandler};
use songbird::{EventContext};


pub struct SongEndNotifier {
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
    pub http: Arc<Http>
}

#[async_trait]
impl SongbirdEventHandler for SongEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in track_list.iter() {
                println!(
                    "Track {:?} in guild {} finished with state {:?}",
                    handle.uuid(),
                    self.guild_id,
                    state.playing
                );
                let _ = self.channel_id.say(&self.http, "Next song Playing").await;
            }
        }
        None
    }
}
