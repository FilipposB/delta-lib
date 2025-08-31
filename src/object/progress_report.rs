use uuid::Uuid;
use crate::object::manifest::Manifest;
use crate::serialization::Serializable;

#[derive(Clone, Debug)]
pub struct ProgressReport {
    id: Uuid,
    chunk_index: u64,
    missed_chunks: Vec<u64>,
}

impl ProgressReport {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn chunk_index(&self) -> u64 {
        self.chunk_index
    }

    pub fn missed_chunks(&self) -> &Vec<u64> {
        &self.missed_chunks
    }

    pub fn new(id: Uuid, chunk_index: u64, missed_chunks: Vec<u64>) -> Self {
        Self { id, chunk_index, missed_chunks }
    }
}

impl Serializable for ProgressReport {
    fn serialize(&self) -> Vec<u8> {
        let mut request_data = self.id.as_bytes().to_vec();

        // Add the chunk index
        request_data.extend_from_slice(&(self.chunk_index).to_be_bytes());

        // Add the number of missed chunks
        request_data.extend_from_slice(&(self.missed_chunks.len() as u64).to_be_bytes());

        // Add the missed chunks themselves
        for chunk in self.missed_chunks.iter() {
            request_data.extend_from_slice(&chunk.to_be_bytes());
        }

        request_data
    }
    
    fn deserialize(bytes: &Vec<u8>) -> Self {
        let uuid_arr: [u8; 16] = bytes[0..16].try_into().unwrap();
        let id = Uuid::from_bytes(uuid_arr);

        // Chunk index (8 bytes)
        let chunk_index = u64::from_be_bytes(bytes[16..24].try_into().unwrap());

        // Missed chunks length (8 bytes)
        let missed_len = u64::from_be_bytes(bytes[24..32].try_into().unwrap()) as usize;

        // Missed chunks (8 bytes each)
        let mut missed_chunks = Vec::with_capacity(missed_len);
        for i in 0..missed_len {
            let start = 32 + i * 8;
            let end = start + 8;
            let chunk = u64::from_be_bytes(bytes[start..end].try_into().unwrap());
            missed_chunks.push(chunk);
        }

        Self {
            id,
            chunk_index,
            missed_chunks,
        }
    }
}