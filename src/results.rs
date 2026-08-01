
pub enum CommandError {
    AgeRestriction,
    InvalidURL,
    NotInCall,
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
}
