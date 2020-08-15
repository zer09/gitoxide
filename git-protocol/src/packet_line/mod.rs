pub(crate) const U16_HEX_BYTES: usize = 4;
pub(crate) const MAX_DATA_LEN: usize = 65516;
pub(crate) const MAX_LINE_LEN: usize = MAX_DATA_LEN + U16_HEX_BYTES;
pub(crate) const FLUSH_LINE: &[u8] = b"0000";
pub(crate) const DELIMITER_LINE: &[u8] = b"0001";
pub(crate) const RESPONSE_END_LINE: &[u8] = b"0002";
pub(crate) const ERR_PREFIX: &[u8] = b"ERR ";

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Channel {
    Data = 1,
    Progress = 2,
    Error = 3,
}

mod borrowed;
pub use borrowed::Borrowed;

mod read {
    use crate::packet_line::{decode, Borrowed, MAX_LINE_LEN};
    use std::io;

    /// Read pack lines one after another, without consuming more than needed from the underlying
    /// `Read`. `Flush` lines cause the reader to stop producing lines forever, leaver `Read` at the
    /// start of whatever comes next.
    pub struct Reader<T> {
        pub inner: T,
        buf: Vec<u8>,
        is_done: bool,
    }

    impl<T> Reader<T>
    where
        T: io::Read,
    {
        pub fn new(inner: T) -> Self {
            Reader {
                inner,
                buf: vec![0; MAX_LINE_LEN],
                is_done: false,
            }
        }

        fn read_line_inner<'a>(
            reader: &mut T,
            buf: &'a mut Vec<u8>,
        ) -> io::Result<Result<Borrowed<'a>, decode::Error>> {
            let (hex_bytes, data_bytes) = buf.split_at_mut(4);
            reader.read_exact(hex_bytes)?;
            let num_data_bytes = match decode::hex_prefix(hex_bytes) {
                Ok(decode::PacketLineOrWantedSize::Line(line)) => return Ok(Ok(line)),
                Ok(decode::PacketLineOrWantedSize::Wanted(additional_bytes)) => additional_bytes as usize,
                Err(err) => return Ok(Err(err)),
            };

            let (data_bytes, _) = data_bytes.split_at_mut(num_data_bytes);
            reader.read_exact(data_bytes)?;
            match decode::to_data_line(data_bytes) {
                Ok(line) => Ok(Ok(line)),
                Err(err) => Ok(Err(err)),
            }
        }

        pub fn read_line(&mut self) -> Option<io::Result<Result<Borrowed, decode::Error>>> {
            if self.is_done {
                return None;
            }
            match Self::read_line_inner(&mut self.inner, &mut self.buf) {
                Ok(Ok(line)) if line == Borrowed::Flush => {
                    self.is_done = true;
                    None
                }
                err => Some(err),
            }
        }
    }
}
pub use read::Reader;

pub mod decode;
pub mod encode;