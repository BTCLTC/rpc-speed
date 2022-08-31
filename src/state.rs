use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpc {
    pub name: String,
    pub rpc: String,
    pub success_count: u64,
    pub failed_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResult {
    pub number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse {
    id: u64,
    jsonrpc: String,
    pub result: RpcResult,
}

#[derive(Debug)]
pub struct TableData {
    pub name: String,
    pub success_count: u64,
    pub failed_count: u64,
    pub success_rate: f64,
    pub response_time: i64,
    pub block_number: i64,
}
