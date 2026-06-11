use std::io::Read;
use manas_core::{ManasError, Network, Neuron};
use crate::format;
use crate::integrity;

pub fn read_from_path(path: &std::path::Path) -> Result<Network, ManasError> {
    let mut file = std::fs::File::open(path)
        .map_err(|e| ManasError::FileReadError {
            path: path.to_path_buf(),
            source: e,
        })?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .map_err(|e| ManasError::FileReadError {
            path: path.to_path_buf(),
            source: e,
        })?;

    read_from_bytes(&data)
}

pub fn read_from_bytes(data: &[u8]) -> Result<Network, ManasError> {
    integrity::verify_checksum(data)?;

    let header = format::read_header(data)
        .ok_or_else(|| ManasError::CorruptFile {
            path: std::path::PathBuf::new(),
            reason: "invalid or missing file header".into(),
        })?;

    let mut layers = Vec::with_capacity(header.total_layers as usize);

    let mut offset = format::HEADER_SIZE;

    let vocab_count = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
    offset += 4;
    for _ in 0..vocab_count {
        let token_len = data[offset] as usize;
        offset += 1 + token_len + 4 + 256;
    }

    let layer_index_size = header.total_layers as usize * 16;
    offset += layer_index_size;

    for _ in 0..header.total_layers {
        let layer = format::read_layer_block(data, &mut offset)
            .ok_or_else(|| ManasError::CorruptFile {
                path: std::path::PathBuf::new(),
                reason: format!("failed to read layer block at offset {}", offset),
            })?;
        layers.push(layer);
    }

    Ok(Network {
        layers,
        total_neurons: header.total_neurons,
        created_at: header.created_at,
        version: header.version,
    })
}

pub fn read_archived_neurons(data: &[u8]) -> Result<Vec<Neuron>, ManasError> {
    let header = format::read_header(data)
        .ok_or_else(|| ManasError::CorruptFile {
            path: std::path::PathBuf::new(),
            reason: "invalid header".into(),
        })?;

    if header.flags & 1 == 0 {
        return Ok(Vec::new());
    }

    let mut offset = format::HEADER_SIZE;

    let vocab_count = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
    offset += 4;
    for _ in 0..vocab_count {
        let token_len = data[offset] as usize;
        offset += 1 + token_len + 4 + 256;
    }

    let layer_index_size = header.total_layers as usize * 16;
    offset += layer_index_size;

    for _ in 0..header.total_layers {
        let _ = format::read_layer_block(data, &mut offset)
            .ok_or_else(|| ManasError::CorruptFile {
                path: std::path::PathBuf::new(),
                reason: format!("failed to read layer block at offset {}", offset),
            })?;
    }

    let archive_layer = format::read_layer_block(data, &mut offset)
        .ok_or_else(|| ManasError::CorruptFile {
            path: std::path::PathBuf::new(),
            reason: "failed to read archive block".into(),
        })?;

    Ok(archive_layer.neurons)
}
