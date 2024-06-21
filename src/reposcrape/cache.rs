use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::{fs, io};

use bincode::{enc::write::Writer, Decode, Encode};
use flate2::bufread::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use super::date::EpochType;
// use super::expand::ExpandedCache;
use super::repo::Repo;
use super::Epoch;

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

#[derive(Eq, PartialEq, Encode, Decode, Default)]
pub struct Cache {
    pub repos: BTreeSet<Repo>,
    pub last_update: EpochType,
}

impl Cache {
    pub fn new(repos: BTreeSet<Repo>) -> Cache {
        Cache {
            repos: repos,
            last_update: Epoch::get_local(),
        }
    }

    // pub fn append(&mut self, other: &mut BTreeSet<Repo>) {
    //     self.repos.append(other);
    // }

    pub fn is_empty(&self) -> bool {
        self.repos.len() == 0
    }

    // pub fn expand(&self) -> ExpandedCache {
    //     ExpandedCache::new(self)
    // }

    pub fn _load(compressed_bytes: &[u8]) -> Result<Cache, bincode::error::DecodeError> {
        let config = bincode::config::standard();
        let bytes = match BinIO::decompress(compressed_bytes) {
            Ok(d) => d,
            Err(_) => return Err(bincode::error::DecodeError::Other("Failed to decompress")),
        };
        let (cache, _len): (Cache, usize) = bincode::decode_from_slice(&bytes[..], config)?;
        return Ok(cache);
    }

    pub fn _dump(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
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

    pub fn save(&self, cache_file: &str) -> io::Result<()> {
        let bin = match self._dump() {
            Ok(b) => b,
            Err(_) => {
                return Err(std::io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "bin dump failed",
                ))
            }
        };
        fs::write(cache_file, bin)
    }

    pub fn load(cache_file: &str) -> Cache {
        let mut file = match fs::File::open(cache_file) {
            Ok(file) => file,
            Err(_) => return Cache::default(),
        };

        let mut buf = Vec::new();
        if file.read_to_end(&mut buf).is_err() {
            return Cache::default();
        }

        match Cache::_load(&buf) {
            Ok(cache) => cache,
            Err(_) => Cache::default(),
        }
    }

    pub fn update(&mut self, repos: &BTreeSet<Repo>) {
        // TODO: Test if extending on incoming repos changes anything or if persistance depends on Ord impl
        let mut repos = repos.clone();
        repos.extend(self.repos.clone());
        self.repos = repos;
        self.last_update = Epoch::get_local();
    }
}
