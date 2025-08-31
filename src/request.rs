use std::io;
use crate::object::manifest::Manifest;
use crate::object::progress_report::ProgressReport;
use crate::request::RequestData::{DownloadResource, SendManifest};
use crate::request::RequestState::{COMPLETED, ERROR, PENDING};
use crate::serialization::Serializable;

pub enum RequestState {
    PENDING,
    COMPLETED(RequestData),
    ERROR(String)
}
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum RequestType {
    DownloadResource,
    SendManifest
}



impl RequestType {
    fn get_request_data(&self, data: &Vec<u8>) -> Option<RequestData> {

        match self {
            RequestType::DownloadResource => {
               Some(DownloadResource(ProgressReport::deserialize(data)))
            },
            RequestType::SendManifest => {
                Some(SendManifest(Manifest::deserialize(data)))
            }
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(RequestType::DownloadResource),
            1 => Some(RequestType::SendManifest),
            _ => None,
        }
    }

    pub fn known_length(&self) -> Option<u64> {
        match self {
            _ => None,
        }
    }

}

pub enum RequestData {
    DownloadResource(ProgressReport),
    SendManifest(Manifest),
}

#[derive(Clone, Debug)]
pub struct Request {
    id: u8,
    request_type: RequestType,
    len: Option<u64>,
    data: Vec<u8>,
}

impl Request {
    
    pub fn from_u8(value: u8) -> Option<Self> {
        RequestType::from_u8(value).and_then(|request_type| Some(Request::new(request_type)))
    }

    pub fn new(request_type: RequestType) -> Self {
        Self { id: request_type as u8, request_type, len: request_type.known_length(), data: Vec::new() }
    }
    
    pub fn pending_bytes(&self) -> usize {

        self.len.unwrap() as usize - self.data.len()
    }

    pub fn add(&mut self, data: &Vec<u8>) -> RequestState {
        self.data.extend_from_slice(data);
        if self.pending_bytes() > 0 {
            return PENDING;
        }

        match self.request_type.get_request_data(&self.data) {
            Some(request_data) => {COMPLETED(request_data)}
            None => {ERROR("Failed to add data to request data!".to_string())}
        }
    }

    pub fn get_length(&self) -> Option<usize> {
        self.len.map(|len| {len as usize})
    }

    pub fn init_length(&mut self, length: u64) -> usize {
        if self.len.is_none() {
            self.len = Some(length);
        }
        self.len.unwrap() as usize
    }

    pub fn request_type(&self) -> RequestType {
        self.request_type
    }
}

pub fn prepare_package(request_type: RequestType, serializable: Box<dyn Serializable>) -> Vec<u8> {
    let mut data = Vec::new();
    data.push(request_type as u8);
    
    let serialized = serializable.serialize();
    
    match  request_type.known_length() {
        None => {
            data.extend_from_slice(&serialized.len().to_be_bytes());
        }
        Some(len) => {
            data.extend_from_slice(&len.to_be_bytes());
        }
    }
    
    data.extend_from_slice(&serializable.serialize());
    data
}