mod macros;

pub use macros::*;
pub use log;
pub mod loggers {
    pub fn init_logger() {
        env_logger::try_init().unwrap_or_default();
    }
}

#[cfg(test)]
mod tests {
    use crate::loggers::init_logger;
    use super::*;

    #[test]
    fn test_loggers() {
        init_logger();
        debug!(test_loggers_macros, "message only");
        info!(test_loggers_macros, "message only");
        warn!(test_loggers_macros, "message only");
        critical!(test_loggers_macros, "message only");
        error!(test_loggers_macros, "message only");

        debug!(test_loggers_macros, "KeyValue"; key1 => "value1", key2 => "value2");
        info!(test_loggers_macros, "KeyValue"; key => "value1", key2 => "value2");
        warn!(test_loggers_macros, "KeyValue"; key1 => "value1", key2 => "value2");
        error!(test_loggers_macros, "KeyValue"; key1 => "value1", key2 => "value2");
        critical!(test_loggers_macros, "KeyValue"; key1 => "value1", key2 => "value2");
    }
}
