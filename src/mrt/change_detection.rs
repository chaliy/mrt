use anyhow::Result;
use std::{cmp, io::copy, fs::File, path::{Path, PathBuf}};
use ignore::Walk;

// Large part of this code is borrowed from https://github.com/BLAKE3-team/BLAKE3/blob/master/b3sum/src/main.rs

// Mmap a file, if it looks like a good idea. Return None in cases where we
// know mmap will fail, or if the file is short enough that mmapping isn't
// worth it. However, if we do try to mmap and it fails, return the error.
fn maybe_memmap_file(file: &File) -> Result<Option<memmap2::Mmap>> {
    let metadata = file.metadata()?;
    let file_size = metadata.len();
    Ok(if !metadata.is_file() {
        // Not a real file.
        None
    } else if file_size > isize::max_value() as u64 {
        // Too long to safely map.
        // https://github.com/danburkert/memmap-rs/issues/69
        None
    } else if file_size == 0 {
        // Mapping an empty file currently fails.
        // https://github.com/danburkert/memmap-rs/issues/72
        None
    } else if file_size < 16 * 1024 {
        // Mapping small files is not worth it.
        None
    } else {
        // Explicitly set the length of the memory map, so that filesystem
        // changes can't race to violate the invariants we just checked.
        let map = unsafe {
            memmap2::MmapOptions::new()
                .len(file_size as usize)
                .map(file)?
        };
        Some(map)
    })
}

enum Input {
    Mmap(std::io::Cursor<memmap2::Mmap>),
    File(File)
}

// Open an input file, using mmap if appropriate
fn open(path: &Path) -> Result<Input> {
    let file = File::open(path)?;
    if let Some(mmap) = maybe_memmap_file(&file)? {
        return Ok(Input::Mmap(std::io::Cursor::new(mmap)));
    }
    Ok(Input::File(file))
}

fn hash(input: Input) -> Result<blake3::OutputReader> {
    let mut hasher = blake3::Hasher::new();
    match input {
        // The fast path: If we mmapped the file successfully, hash using
        // multiple threads.
        Input::Mmap(cursor) => {
            hasher.update_rayon(cursor.get_ref());
        }
        Input::File(mut file) => {
            copy(&mut file, &mut hasher)?;
        }
    }
    Ok(hasher.finalize_xof())
}


fn to_hex_string(mut output: blake3::OutputReader) -> Result<String> {
    let mut len = blake3::OUT_LEN as u64;
    let mut block = [0; blake3::guts::BLOCK_LEN];
    let mut hex_str = String::new();
    while len > 0 {
        output.fill(&mut block);
        hex_str.push_str(&hex::encode(&block[..]));
        let take_bytes = cmp::min(len, block.len() as u64);
        len -= take_bytes;
    }
    Ok(hex_str)
}

pub fn read_hashes<P: AsRef<Path>>(path: P) -> Vec<(PathBuf, String)> {

    return Walk::new(path)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .map(|e| {
            let path = e.path();
            if path.is_file() {
                let input = open(path).unwrap();
                let output = hash(input).unwrap();
                return Some((path.to_path_buf(), to_hex_string(output).unwrap()));
            }
            return None;
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();
}