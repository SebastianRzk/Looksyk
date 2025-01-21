use humansize::{format_size, DECIMAL};

pub fn filesize_as_human_string(size: u64) -> String {
    format_size(size, DECIMAL)
}

pub fn timestamp_as_human_string(timestamp: i64) -> String {
    chrono::DateTime::from_timestamp(timestamp, 0)
        .expect("Could not convert timestamp")
        .format("%d.%m.%Y %H:%M:%S")
        .to_string()
}
