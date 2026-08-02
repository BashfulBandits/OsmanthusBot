
pub enum CommandError {
    AgeRestriction,
    InvalidURL,
    NotInCall,
    BotNotInCall,
    GetGuild,
    
    CommandNotImplemented,
    Other,
}

pub enum CommandSuccess {
    Join,
    Leave,

    Play,
    Pause,
    Resume,
    Skip,
    Stop,

    Help,

    Aura,
    
    AddedToQueue
}
