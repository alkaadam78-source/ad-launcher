use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PPeer {
    pub device_name: String,
    pub ip_address: String,
    pub port: u16,
    pub is_authenticated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferPayloadType {
    InstanceProfile,
    ModpackArchive,
    WorldBackup,
    Screenshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferManifest {
    pub transfer_id: String,
    pub payload_name: String,
    pub payload_type: TransferPayloadType,
    pub size_bytes: u64,
    pub sha256_checksum: String,
}

pub struct LocalTransferManager;

impl LocalTransferManager {
    pub const DEFAULT_TRANSFER_PORT: u16 = 47890;

    /// Generates transfer invitation for pairing devices on local Wi-Fi / Ethernet
    pub fn create_pairing_token(device_name: &str) -> String {
        let random_suffix = uuid::Uuid::new_v4().to_string();
        format!("AD-P2P-{}-{}", device_name.replace(' ', "-"), &random_suffix[..8])
    }
}
