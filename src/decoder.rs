use bytes::{Bytes, BytesMut};
use failure;
use std::str;

pub trait Decoder {
    type Item;
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, failure::Error>;
}

#[derive(Debug, PartialEq)]
pub enum RecordIoDecoderState {
    TrimWhitespaces,
    ReadLength,
    ReadRecord { len: u64 },
}

/// Decoder for [RecordIO](http://mesos.apache.org/documentation/latest/scheduler-http-api/#recordio-response-format-1)
/// format.
pub struct RecordIoDecoder {
    state: RecordIoDecoderState,
}
impl RecordIoDecoder {
    pub fn new() -> Self {
        Self {
            state: RecordIoDecoderState::TrimWhitespaces,
        }
    }

    fn is_whitespace(&self, b: &u8) -> bool {
        (*b == b' ' || *b == b'\n' || *b == b'\r' || *b == b'\t')
    }

    pub fn trim_whitespaces(&mut self, buf: &mut BytesMut) -> RecordIoDecoderState {
        let whitespaces: usize = buf.iter().take_while(|&b| self.is_whitespace(b)).count();
        buf.split_to(whitespaces);
        RecordIoDecoderState::ReadLength
    }

    pub fn decode_length(
        &mut self,
        buf: &mut BytesMut,
    ) -> Result<RecordIoDecoderState, failure::Error> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let length = buf.split_to(i);
            buf.split_to(1);

            let s = str::from_utf8(&length)?;
            let length: u64 = s.parse()?;
            Ok(RecordIoDecoderState::ReadRecord { len: length })
        } else {
            Ok(RecordIoDecoderState::ReadLength)
        }
    }

    pub fn decode_record(
        &mut self,
        length: u64,
        buf: &mut BytesMut,
    ) -> (RecordIoDecoderState, Option<Bytes>) {
        if (buf.len() as u64) < length {
            return (RecordIoDecoderState::ReadRecord { len: length }, None);
        } else {
            let record_buf = buf.split_to(length as usize);
            return (
                RecordIoDecoderState::TrimWhitespaces,
                Some(record_buf.freeze()),
            );
        }
    }
}
impl Decoder for RecordIoDecoder {
    type Item = Bytes;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Bytes>, failure::Error> {
        while buf.len() > 0 {
            match self.state {
                RecordIoDecoderState::TrimWhitespaces => {
                    self.state = self.trim_whitespaces(buf);
                }
                RecordIoDecoderState::ReadLength => {
                    self.state = self.decode_length(buf)?;
                }
                RecordIoDecoderState::ReadRecord { len } => {
                    let (new_state, record) = self.decode_record(len, buf);
                    self.state = new_state;
                    return Ok(record);
                }
            }
        }
        return Ok(None);
    }
}

#[cfg(test)]
mod tests {

    use bytes::{BufMut, Bytes, BytesMut};
    use decoder::{Decoder, RecordIoDecoder, RecordIoDecoderState};
    use spectral::prelude::*;

    #[test]
    fn trim_whitespaces() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"\t\n \r121\n{\"type\":\"HEARTBEAT\"}"[..]);
        let mut decoder = RecordIoDecoder::new();

        let state = decoder.trim_whitespaces(&mut buffer);
        assert_eq!(state, RecordIoDecoderState::ReadLength);
        assert_eq!(buffer, "121\n{\"type\":\"HEARTBEAT\"}");
    }

    #[test]
    fn decode_length() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"121\n"[..]);
        let mut decoder = RecordIoDecoder::new();

        let state = decoder.decode_length(&mut buffer);
        assert_that(&state)
            .is_ok()
            .is_equal_to(RecordIoDecoderState::ReadRecord { len: 121 });
    }

    #[test]
    fn decode_length_error() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"1f1\n"[..]);
        let mut decoder = RecordIoDecoder::new();

        let state = decoder.decode_length(&mut buffer);
        assert_that(&state).is_err();
    }

    #[test]
    fn decode_length_invalid() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"-42\n"[..]);
        let mut decoder = RecordIoDecoder::new();

        let state = decoder.decode_length(&mut buffer);
        assert_that(&state).is_err();
    }

    #[test]
    fn decode_record() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"{\"type\":\"HEARTBEAT\"}"[..]);
        let mut decoder = RecordIoDecoder::new();

        let (state, record) = decoder.decode_record(20, &mut buffer);
        assert_eq!(state, RecordIoDecoderState::TrimWhitespaces);
        assert_that(&record)
            .is_some()
            .is_equal_to(Bytes::from("{\"type\":\"HEARTBEAT\"}"));
    }

    #[test]
    fn not_decode_record() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"{\"type\":\"HEARTBEAT\"}"[..]);
        let mut decoder = RecordIoDecoder::new();

        let (state, record) = decoder.decode_record(42, &mut buffer);
        assert_eq!(state, RecordIoDecoderState::ReadRecord { len: 42 });
        assert_that(&record).is_none();
    }

    #[test]
    fn decode_multiple_records() {
        let mut buffer = BytesMut::with_capacity(1024);
        let body = b"\t  \r\n121\n{\"type\": \"SUBSCRIBED\",\"subscribed\": {\"framework_id\": {\"value\":\"12220-3440-12532-2345\"},\"heartbeat_interval_seconds\":15.0}\
            \t \r\n 20\n{\"type\":\"HEARTBEAT\"}";
        buffer.put(&body[..]);
        let mut decoder = RecordIoDecoder::new();

        let first = decoder.decode(&mut buffer);
        let expected = Bytes::from(
            "{\"type\": \"SUBSCRIBED\",\"subscribed\": {\"framework_id\": {\"value\":\"12220-3440-12532-2345\"},\"heartbeat_interval_seconds\":15.0}",
        );
        assert_that(&first).is_ok().is_some().is_equal_to(expected);

        let second = decoder.decode(&mut buffer);
        assert_that(&second)
            .is_ok()
            .is_some()
            .is_equal_to(Bytes::from("{\"type\":\"HEARTBEAT\"}"));

        let third = decoder.decode(&mut buffer);
        assert_that(&third).is_ok().is_none();
    }
}
