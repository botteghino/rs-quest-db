use crate::error::LineSenderError;
use std::convert::TryInto;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};

pub struct LineTcpSender {
    tcp_stream: TcpStream,
}

impl LineTcpSender {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Self {
        Self {
            tcp_stream: TcpStream::connect(addr).expect("Couldn't connect to address"),
        }
    }

    pub fn write_line<S>(&mut self, msg: S) -> Result<usize, LineSenderError>
    where
        S: TryInto<String>,
        <S as TryInto<String>>::Error: std::fmt::Display,
    {
        let msg_str: String = msg
            .try_into()
            .map_err(|e| LineSenderError::StringConversionError(e.to_string()))?;
        if !msg_str.ends_with('\n') {
            return Err(LineSenderError::UnterminatedLine(msg_str));
        }
        let written_bytes = self.tcp_stream.write(msg_str.as_bytes())?;
        Ok(written_bytes)
    }
}
