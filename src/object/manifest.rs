use uuid::Uuid;
use crate::serialization::Serializable;

pub struct Manifest {
    id: Uuid,
    name: String,
    chunk_size: u64,
    total_chunks: u64,
    total_bytes: u64,
}

impl Manifest {
    pub fn new(id: Uuid, name: String, chunk_size: u64, total_chunks: u64, total_bytes: u64) -> Self {
        Self { id, name, chunk_size, total_chunks, total_bytes }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn chunk_size(&self) -> u64 {
        self.chunk_size
    }
    
    pub fn payload_size(&self) -> u64 {
        self.chunk_size - 8
    }

    pub fn total_chunks(&self) -> u64 {
        self.total_chunks
    }
    
    pub fn last_payload_bytes(&self) -> u64 {
        (self.total_chunks * self.payload_size()) -  self.total_bytes
    }
}

impl Serializable for Manifest {
    fn serialize(&self) -> Vec<u8> {
        let mut data = self.id.as_bytes().to_vec();

        // Serialize the name length and the name itself
        let name_bytes = self.name.as_bytes();
        let name_len = name_bytes.len() as u64;
        data.extend_from_slice(&name_len.to_be_bytes());
        data.extend_from_slice(name_bytes);

        // Serialize chunk_size and total_chunks
        data.extend_from_slice(&self.chunk_size.to_be_bytes());
        data.extend_from_slice(&self.total_chunks.to_be_bytes());
        data.extend_from_slice(&self.total_bytes.to_be_bytes());

        data
    }

    fn deserialize(bytes: &Vec<u8>) -> Self {
        // Extract the UUID
        let uuid_arr: [u8; 16] = bytes[0..16].try_into().unwrap();
        let id = Uuid::from_bytes(uuid_arr);

        // Extract the name length
        let name_len = u64::from_be_bytes(bytes[16..24].try_into().unwrap()) as usize;

        // Extract the name string
        let name_start = 24;
        let name_end = name_start + name_len;
        let name = String::from_utf8(bytes[name_start..name_end].to_vec()).unwrap();

        // Extract chunk_size and total_chunks
        let chunk_size = u64::from_be_bytes(bytes[name_end..name_end + 8].try_into().unwrap());
        let total_chunks = u64::from_be_bytes(bytes[name_end + 8..name_end + 16].try_into().unwrap());
        let total_bytes = u64::from_be_bytes(bytes[name_end + 16..name_end + 24].try_into().unwrap());

        Self {
            id,
            name,
            chunk_size,
            total_chunks,
            total_bytes
        }
    }
}
