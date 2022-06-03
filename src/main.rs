mod error;
mod line_tcp_sender;
mod questdb_line_protocol;

use rand::Rng;
use std::thread;
use std::time::Duration;

use crate::line_tcp_sender::LineTcpSender;
use crate::questdb_line_protocol::{ColumnValue, DataPoint};

static DB_ADDRESS: &str = "localhost:9009";

fn main() {
    let mut rng = rand::thread_rng();
    let mut sender = LineTcpSender::connect(DB_ADDRESS);

    for _i in 0..10 {
        let data_point = DataPoint {
            table: "weather",
            symbol_set: vec![("location", "San Francisco")],
            column_set: vec![("temperature", ColumnValue::Integer(rng.gen_range(30..=120)))],
            timestamp: Some(questdb_line_protocol::now()),
        };

        let result = sender
            .write_line(data_point)
            .expect("Error sending line to server");
        println!("Sent {result} bytes");

        thread::sleep(Duration::from_secs(1));
    }
}
