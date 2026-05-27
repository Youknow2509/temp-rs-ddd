/// Consumer topic for completed conversation turns.
pub const EX_01_TOPIC: &str = "topic1";

/// Consumer topic for session end events.
pub const EX_02_TOPIC: &str = "topic2";

/// All consumer topics handled by this service.
pub const CONSUMER_TOPICS: [&str; 2] = [EX_01_TOPIC, EX_02_TOPIC];
