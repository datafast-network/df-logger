DF Logger
=========

## Use
`[log_level]([action_or_path_name], "message"; key1 => "value1", key2 => "value2")`

```rust
use df_logger::*;
use df_logger::loggers::init_logger;

fn main() {
    init_logger();
    info!(main, "Hello, world!");
    
    // Log with key-value pairs
    debug!(main_debug, "KeyValue"; key1 => "value1", key2 => "value2");
    info!(main_info, "KeyValue"; key => "value1", key2 => "value2");
    warn!(main_warn, "KeyValue"; key1 => "value1", key2 => "value2");
    error!(main_error, "KeyValue"; key1 => "value1", key2 => "value2");
    critical!(main_critical, "KeyValue"; key1 => "value1", key2 => "value2");
}
```