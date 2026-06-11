use manas_core::ManasError;

pub fn compute_crc32(data: &[u8]) -> u32 {
    crc32fast::hash(data)
}

pub fn verify_checksum(data: &[u8]) -> Result<u32, ManasError> {
    if data.len() < 4 {
        return Err(ManasError::CorruptFile {
            path: std::path::PathBuf::new(),
            reason: "file too small to contain checksum".into(),
        });
    }
    let stored_crc = u32::from_le_bytes(data[data.len() - 4..].try_into().unwrap());
    let actual_crc = compute_crc32(&data[..data.len() - 4]);
    if stored_crc != actual_crc {
        return Err(ManasError::ChecksumMismatch);
    }
    Ok(stored_crc)
}
