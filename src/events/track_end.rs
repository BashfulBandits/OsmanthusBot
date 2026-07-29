use std::sync::Arc;

use serenity::{all::{ChannelId, Context, GuildId, Http}, async_trait};
use songbird::{Event, events::EventHandler as SongbirdEventHandler};
use songbird::{EventContext};

use crate::common::queue;


pub struct SongEndNotifier {
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
    pub ctx: Context,
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

                queue::remove_first_track_metadata(&self.ctx, self.guild_id).await;

                //let _ = self.channel_id.say(&self.http, "Next song Playing").await;
                let manager = songbird::get(&self.ctx)
                    .await
                    .expect("Songbird Voice client placed in at initialisation.")
                    .clone();

                if let Some(handler_lock) = manager.get(self.guild_id) {
                    let handler = handler_lock.lock().await;
                    if handler.queue().is_empty() {
                        let _ = self.channel_id.say(&self.http, "Queue empty").await;
                    }
                };
            }
        }
        None
    }
}
