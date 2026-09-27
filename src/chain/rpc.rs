use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntryInfo {
    pub key: String,
    pub live_until_ledger_seq: Option<u32>, // None if archived
    pub is_archived: bool,
}

#[derive(Clone)]
pub struct SorobanRpcClient {
    pub rpc_url: String,
    mock_entries: Arc<Mutex<HashMap<String, LedgerEntryInfo>>>,
}

impl SorobanRpcClient {
    pub fn new(rpc_url: String) -> Self {
        Self {
            rpc_url,
            mock_entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert_mock_entry(&self, key: &str, live_until: Option<u32>, is_archived: bool) {
        let mut entries = self.mock_entries.lock().unwrap();
        entries.insert(
            key.to_string(),
            LedgerEntryInfo {
                key: key.to_string(),
                live_until_ledger_seq: live_until,
                is_archived,
            },
        );
    }

    pub async fn get_latest_ledger(&self) -> Result<u32, String> {
        Ok(1_000_000)
    }

    pub async fn get_ledger_entries(&self, keys: &[String]) -> Result<Vec<LedgerEntryInfo>, String> {
        let entries = self.mock_entries.lock().unwrap();
        let mut result = Vec::new();
        for k in keys {
            if let Some(entry) = entries.get(k) {
                result.push(entry.clone());
            } else {
                result.push(LedgerEntryInfo {
                    key: k.clone(),
                    live_until_ledger_seq: Some(1_500_000),
                    is_archived: false,
                });
            }
        }
        Ok(result)
    }
}
