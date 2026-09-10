//! Minimal EBML / Matroska byte parser.
//! Ported from ebml.ts — operates on raw byte slices with zero allocation.

// EBML Element IDs
pub const ID_SEGMENT: u32 = 0x18538067;
pub const ID_TRACKS: u32 = 0x1654AE6B;
pub const ID_TRACK_ENTRY: u32 = 0xAE;
pub const ID_TRACK_NUMBER: u32 = 0xD7;
pub const ID_TRACK_TYPE: u32 = 0x83;
pub const ID_CODEC_ID: u32 = 0x86;
pub const ID_LANGUAGE: u32 = 0x22B59C;
pub const ID_CLUSTER: u32 = 0x1F43B675;
pub const ID_SIMPLE_BLOCK: u32 = 0xA3;
pub const ID_TIME_CODE: u32 = 0xE7;
pub const ID_CUES: u32 = 0x1C53BB6B;
pub const ID_CUE_POINT: u32 = 0xBB;
pub const ID_CUE_TIME: u32 = 0xB3;
pub const ID_CUE_TRACK_POSITIONS: u32 = 0xB7;
pub const ID_CUE_CLUSTER_POSITION: u32 = 0xF1;
pub const ID_INFO: u32 = 0x1549A966;
pub const ID_TIMECODE_SCALE: u32 = 0x2AD7B1;

#[derive(Debug, Clone, PartialEq)]
pub struct EbmlElement {
    pub id: u32,
    pub data_offset: usize,
    pub end_offset: usize,
}

/// Read EBML Variable Length Integer from a byte slice at `offset`.
/// Returns `(value, byte_length)` or `None` if the buffer is incomplete.
pub fn read_vint(data: &[u8], offset: usize) -> Option<(u64, usize)> {
    if offset >= data.len() {
        return None;
    }
    let first = data[offset];
    let mut length = 1usize;
    while length <= 8 && (first & (0x80 >> (length - 1))) == 0 {
        length += 1;
    }
    if length > 8 || offset + length > data.len() {
        return None;
    }
    // Mask out the length marker bit
    let mut value = (first & ((1u8 << (8 - length)).wrapping_sub(1))) as u64;
    for i in 1..length {
        value = (value << 8) | data[offset + i] as u64;
    }
    Some((value, length))
}

/// Parse the EBML element at `offset`. Returns `None` if the buffer is incomplete.
pub fn read_element(data: &[u8], offset: usize) -> Option<EbmlElement> {
    let (id_value, id_len) = read_vint(data, offset)?;
    let (size_value, size_len) = read_vint(data, offset + id_len)?;
    let data_offset = offset + id_len + size_len;
    let end_offset = data_offset + size_value as usize;
    if end_offset > data.len() {
        return None;
    }
    Some(EbmlElement {
        id: id_value as u32,
        data_offset,
        end_offset,
    })
}

/// Read a UTF-8 string from `data[offset..offset+length]`, stripping null bytes.
pub fn read_string(data: &[u8], offset: usize, length: usize) -> String {
    data[offset..offset + length]
        .iter()
        .filter(|&&b| b != 0)
        .map(|&b| b as char)
        .collect()
}

/// Read an unsigned big-endian integer of `length` bytes.
pub fn read_uint(data: &[u8], offset: usize, length: usize) -> u64 {
    data[offset..offset + length]
        .iter()
        .fold(0u64, |acc, &b| (acc << 8) | b as u64)
}

/// Read a signed 16-bit big-endian integer.
pub fn read_int16(data: &[u8], offset: usize) -> i16 {
    let val = ((data[offset] as u16) << 8) | data[offset + 1] as u16;
    val as i16
}

/// Scan forward from `start` to `end` looking for the first element with `target_id`.
pub fn find_element(
    data: &[u8],
    start: usize,
    end: usize,
    target_id: u32,
) -> Option<EbmlElement> {
    let mut pos = start;
    while pos < end {
        let el = read_element(data, pos)?;
        if el.id == target_id {
            return Some(el);
        }
        pos = el.end_offset;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_vint_single_byte() {
        // 0x81 = 1000_0001 → length=1, value=1
        let data = [0x81u8];
        let result = read_vint(&data, 0);
        assert_eq!(result, Some((1, 1)));
    }

    #[test]
    fn test_read_vint_two_bytes() {
        // 0x40 0x01 → length=2, value=1
        let data = [0x40u8, 0x01];
        let result = read_vint(&data, 0);
        assert_eq!(result, Some((1, 2)));
    }

    #[test]
    fn test_read_vint_out_of_bounds() {
        let data = [];
        assert_eq!(read_vint(&data, 0), None);
    }

    #[test]
    fn test_read_element_basic() {
        // ID: 0x81 (vint=1, len=1), Size: 0x84 (vint=4, len=1), Data: [0,0,0,0]
        let data = [0x81u8, 0x84, 0x00, 0x00, 0x00, 0x00];
        let el = read_element(&data, 0).unwrap();
        assert_eq!(el.id, 1);
        assert_eq!(el.data_offset, 2);
        assert_eq!(el.end_offset, 6);
    }

    #[test]
    fn test_read_uint() {
        let data = [0x00u8, 0x01, 0xA4, 0xFF];
        assert_eq!(read_uint(&data, 0, 4), 0x0001A4FF);
    }

    #[test]
    fn test_read_string() {
        let data = b"A_AAC\x00\x00";
        assert_eq!(read_string(data, 0, 7), "A_AAC");
    }

    #[test]
    fn test_read_int16_positive() {
        let data = [0x01u8, 0x00];
        assert_eq!(read_int16(&data, 0), 256i16);
    }

    #[test]
    fn test_read_int16_negative() {
        let data = [0x80u8, 0x00];
        assert_eq!(read_int16(&data, 0), i16::MIN);
    }

    #[test]
    fn test_find_element() {
        // Two elements: id=1 size=0, id=2 size=0
        // 0x81 0x80 = id=1, size=0
        // 0x82 0x80 = id=2, size=0
        let data = [0x81u8, 0x80, 0x82, 0x80];
        let el = find_element(&data, 0, data.len(), 2).unwrap();
        assert_eq!(el.id, 2);
        assert_eq!(el.data_offset, 4);
    }
}
