//! MKV Audio Track Extractor.
//! Ported from mkvAudioExtractor.ts — uses range-based byte fetching
//! to pull raw audio frames without downloading the entire file.

use crate::utils::ebml::*;

#[derive(Debug, Clone)]
pub struct AudioTrack {
    pub track_number: u64,
    pub codec_id: String,
    pub language: String,
}

#[derive(Debug, Clone)]
pub struct Cue {
    pub timecode_ms: f64,
    pub position: u64,
}

#[derive(Debug, Clone)]
pub struct AudioFrame {
    pub timecode_ms: f64,
    pub data: Vec<u8>,
}

/// Parse the Tracks element to locate the first audio track.
/// Returns `None` if no audio track is found in the supplied byte slice.
pub fn parse_audio_track(data: &[u8], tracks_start: usize, tracks_end: usize) -> Option<AudioTrack> {
    let mut pos = tracks_start;
    while pos < tracks_end {
        let entry = find_element(data, pos, tracks_end, ID_TRACK_ENTRY)?;
        let mut track_num = 0u64;
        let mut track_type = 0u64;
        let mut codec = String::new();
        let mut lang = String::new();

        let mut inner = entry.data_offset;
        while inner < entry.end_offset {
            let child = read_element(data, inner)?;
            let size = child.end_offset - child.data_offset;
            match child.id {
                x if x == ID_TRACK_NUMBER => track_num = read_uint(data, child.data_offset, size),
                x if x == ID_TRACK_TYPE => track_type = read_uint(data, child.data_offset, size),
                x if x == ID_CODEC_ID => codec = read_string(data, child.data_offset, size),
                x if x == ID_LANGUAGE => lang = read_string(data, child.data_offset, size),
                _ => {}
            }
            inner = child.end_offset;
        }

        if track_type == 2 {
            return Some(AudioTrack {
                track_number: track_num,
                codec_id: codec,
                language: lang,
            });
        }
        pos = entry.end_offset;
    }
    None
}

/// Parse a single Cluster's SimpleBlock frames within [start_ms, end_ms].
pub fn parse_cluster(
    data: &[u8],
    audio_track_number: u64,
    start_ms: f64,
    end_ms: f64,
    timecode_scale: f64,
) -> Vec<AudioFrame> {
    let mut frames = Vec::new();

    let cluster = match read_element(data, 0) {
        Some(el) if el.id == ID_CLUSTER => el,
        _ => return frames,
    };

    let mut pos = cluster.data_offset;
    let mut cluster_tc = 0u64;

    // Read cluster timecode first
    if let Some(tc_el) = find_element(data, pos, cluster.end_offset, ID_TIME_CODE) {
        let size = tc_el.end_offset - tc_el.data_offset;
        cluster_tc = read_uint(data, tc_el.data_offset, size);
        pos = tc_el.end_offset;
    }

    while pos < cluster.end_offset && pos < data.len() {
        let el = match read_element(data, pos) {
            Some(e) => e,
            None => break,
        };

        if el.id == ID_SIMPLE_BLOCK {
            if let Some(frame) = parse_simple_block(
                data,
                el.data_offset,
                el.end_offset,
                cluster_tc,
                audio_track_number,
                timecode_scale,
            ) {
                if frame.timecode_ms >= start_ms && frame.timecode_ms <= end_ms {
                    frames.push(frame);
                }
            }
        }
        pos = el.end_offset;
    }
    frames
}

/// Parse a SimpleBlock, returning the audio frame if it belongs to the target track.
pub fn parse_simple_block(
    data: &[u8],
    start: usize,
    end: usize,
    cluster_tc: u64,
    target_track: u64,
    timecode_scale: f64,
) -> Option<AudioFrame> {
    let (track_num, vint_len) = read_vint(data, start)?;
    if track_num != target_track {
        return None;
    }
    let tc_pos = start + vint_len;
    if tc_pos + 3 > end {
        return None;
    }
    let timecode = read_int16(data, tc_pos) as f64;
    let frame_start = tc_pos + 3; // skip timecode (2) + flags (1)

    let timecode_ms = (cluster_tc as f64 + timecode) * timecode_scale / 1_000_000.0;

    Some(AudioFrame {
        timecode_ms,
        data: data[frame_start..end].to_vec(),
    })
}

/// Build a sorted cue table from the Cues element byte slice.
pub fn parse_cues(data: &[u8], segment_offset: u64, timecode_scale: f64) -> Vec<Cue> {
    let mut cues = Vec::new();

    let cues_el = match find_element(data, 0, data.len(), ID_CUES) {
        Some(el) => el,
        None => return cues,
    };

    let mut pos = cues_el.data_offset;
    while pos < cues_el.end_offset {
        let point = match find_element(data, pos, cues_el.end_offset, ID_CUE_POINT) {
            Some(p) => p,
            None => break,
        };

        let mut cue_time = 0u64;
        let mut cluster_pos = 0u64;
        let mut inner = point.data_offset;

        while inner < point.end_offset {
            let child = match read_element(data, inner) {
                Some(c) => c,
                None => break,
            };
            let size = child.end_offset - child.data_offset;
            if child.id == ID_CUE_TIME {
                cue_time = read_uint(data, child.data_offset, size);
            } else if child.id == ID_CUE_TRACK_POSITIONS {
                let mut tp = child.data_offset;
                while tp < child.end_offset {
                    let tp_child = match read_element(data, tp) {
                        Some(c) => c,
                        None => break,
                    };
                    if tp_child.id == ID_CUE_CLUSTER_POSITION {
                        let s = tp_child.end_offset - tp_child.data_offset;
                        cluster_pos = read_uint(data, tp_child.data_offset, s);
                    }
                    tp = tp_child.end_offset;
                }
            }
            inner = child.end_offset;
        }

        if cluster_pos > 0 {
            let timecode_ms = (cue_time as f64 * timecode_scale) / 1_000_000.0;
            cues.push(Cue {
                timecode_ms,
                position: segment_offset + cluster_pos,
            });
        }
        pos = point.end_offset;
    }

    cues.sort_by(|a, b| a.timecode_ms.partial_cmp(&b.timecode_ms).unwrap());
    cues
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_block_wrong_track() {
        // vint track=1, timecode=[0,0], flags=0, data=[0xAA]
        let data = [0x81u8, 0x00, 0x00, 0x00, 0xAA];
        // Target track is 2 — should return None
        let result = parse_simple_block(&data, 0, data.len(), 0, 2, 1_000_000.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_simple_block_correct_track() {
        // vint track=1 (0x81), timecode=[0,10], flags=0x00, data=[0xDE, 0xAD]
        let data = [0x81u8, 0x00, 10, 0x00, 0xDE, 0xAD];
        let result = parse_simple_block(&data, 0, data.len(), 0, 1, 1_000_000.0);
        assert!(result.is_some());
        let frame = result.unwrap();
        assert_eq!(frame.data, vec![0xDE, 0xAD]);
        assert!((frame.timecode_ms - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_parse_cues_empty() {
        let cues = parse_cues(&[], 0, 1_000_000.0);
        assert!(cues.is_empty());
    }
}
