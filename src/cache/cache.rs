use std::any::Any;
use std::collections::{BTreeSet, HashMap};
use std::io::{Read, Write};
use std::{fs, io};

use bincode::{enc::write::Writer, Decode, Encode};
use flate2::bufread::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::date::{Epoch, EpochType};
use crate::reposcrape::Repo;

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

pub trait Update<T> {
    fn update(&mut self, other: &T);
}

#[derive(Eq, PartialEq, Encode, Decode, Default)]
pub struct Cachable<T> {
    pub data: T,
    pub days_to_update: u32,
    pub last_update: EpochType,
}

impl<T: Clone> Cachable<T> {
    pub fn is_outdated(&self) -> bool {
        let local = Epoch::get_local();
        let millis = (self.days_to_update * 24 * 60 * 60 * 100).into();
        (self.last_update < local) && (local - self.last_update > millis)
    }
}

impl Update<BTreeSet<Repo>> for Cachable<BTreeSet<Repo>> {
    fn update(&mut self, other: &BTreeSet<Repo>) {
        // TODO: Test if extending on incoming repos changes anything or if persistance depends on Ord impl
        let mut other = other.clone();
        other.extend(self.data.clone());
        self.data = other;
        self.last_update = Epoch::get_local();
    }
}

impl Update<HashMap<String, String>> for Cachable<HashMap<String, String>> {
    fn update(&mut self, other: &HashMap<String, String>) {
        let mut other = other.to_owned();
        other.extend(self.data.clone());
        self.data = other;
    }
}

#[derive(Eq, PartialEq, Encode, Decode, Default)]
pub struct Cache {
    pub repos: Cachable<BTreeSet<Repo>>,
    pub colors: Cachable<HashMap<String, String>>,
}

impl Cache {
    pub fn new(
        repos: Option<Cachable<BTreeSet<Repo>>>,
        colors: Option<Cachable<HashMap<String, String>>>,
    ) -> Cache {
        Cache {
            repos: repos.unwrap_or_default(),
            colors: colors.unwrap_or_default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.repos.data.len() == 0
    }

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
}
