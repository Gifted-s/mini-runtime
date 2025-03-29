use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub identity: Identity,
    pub resource: Resource,
}

#[derive(Debug, Deserialize)]
pub struct Identity {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Resource {
    pub io: IOConfig,
    pub response: ResponseConfig,
    pub worker_thread_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct IOConfig {
    pub io_queue_size: usize,
    pub io_queue_no: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResponseConfig {
    pub response_queue_size: usize,
    pub response_queue_no: u32,
}