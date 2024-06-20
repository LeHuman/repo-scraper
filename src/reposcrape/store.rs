use std::collections::BTreeSet;
use std::io::{Read, Write};

use bincode::{enc::write::Writer, Decode, Encode};
use flate2::bufread::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use super::repo::Repo;

struct BinIO<'a> {
    encoder: &'a mut ZlibEncoder<Vec<u8>>,
}

impl BinIO<'_> {
    fn new(encoder: &mut ZlibEncoder<Vec<u8>>) -> BinIO {
        return BinIO { encoder: encoder };
    }

    fn decompress(compressed_bytes: &[u8]) -> Result<Vec<u8>, std::io::Error> {
        let mut decoder = ZlibDecoder::new(compressed_bytes);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer)?;
        return Ok(buffer);
    }
}

impl Writer for BinIO<'_> {
    fn write(&mut self, bytes: &[u8]) -> Result<(), bincode::error::EncodeError> {
        let result = self.encoder.write(bytes);
        if result.is_err() {
            return Err(bincode::error::EncodeError::Other(
                "Encoder returned an error",
            ));
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq, Encode, Decode)]
pub struct BinCache {
    pub(crate) repos: BTreeSet<Repo>,
}

impl BinCache {
    pub fn new(repos: BTreeSet<Repo>) -> BinCache {
        BinCache { repos: repos }
    }

    pub fn load(compressed_bytes: &[u8]) -> Result<BinCache, bincode::error::DecodeError> {
        let config = bincode::config::standard();
        let decompressed = BinIO::decompress(compressed_bytes);

        if decompressed.is_err() {
            return Err(bincode::error::DecodeError::Other("Failed to decompress"));
        }

        let bytes = decompressed.expect("Failed to load decompressed bytes");
        let (cache, _len): (BinCache, usize) = bincode::decode_from_slice(&bytes[..], config)?;
        return Ok(cache);
    }

    pub fn dump(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        let config = bincode::config::standard();
        let mut encoder: ZlibEncoder<Vec<u8>> = ZlibEncoder::new(Vec::new(), Compression::best());
        let io = BinIO::new(&mut encoder);
        bincode::encode_into_writer(self, io, config)?;
        let compressed_bytes = encoder.finish();

        if compressed_bytes.is_err() {
            return Err(bincode::error::EncodeError::Other(
                "Failed to dump final compressed binary",
            ));
        }

        return Ok(compressed_bytes.expect("Failed to load compressed bytes"));
    }
}

// pub struct ExpandedCache {
//     pub(crate) repos: BTreeMap<String, Repo>,
//     pub(crate) projects: BTreeMap<String, ProjectEntry>,
// }
