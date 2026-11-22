use serde::{Deserialize, Serialize};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftServerEntry {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub category: String, // "Classical Baroque", "Technical Vanilla", "Fabric Modded", "Custom"
    pub supported_version: String,
    pub description: String,
    pub is_favorite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerPingResult {
    pub is_online: bool,
    pub latency_ms: Option<u64>,
    pub status_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendProfile {
    pub username: String,
    pub uuid: String,
    pub note: String,
    pub last_seen: String,
    pub is_favorite: bool,
}

pub struct MultiplayerManager;

impl MultiplayerManager {
    /// Returns default curated servers (verified community servers & LAN templates)
    pub fn curated_servers() -> Vec<MinecraftServerEntry> {
        vec![
            MinecraftServerEntry {
                id: "server-baroque-gallery".to_string(),
                name: "The Grand Baroque Gallery (Creative Realm)".to_string(),
                address: "gallery.minecraft.ad".to_string(),
                port: 25565,
                category: "Classical Baroque".to_string(),
                supported_version: "1.20.1 / 1.21.4".to_string(),
                description: "Palatial Baroque architecture showcase, Chiaroscuro shader exhibition, and European classical art builds.".to_string(),
                is_favorite: true,
            },
            MinecraftServerEntry {
                id: "server-potato-smp".to_string(),
                name: "Intel HD 620 High-FPS SMP (Fabric 1.20.1)".to_string(),
                address: "smp.adamdevnoob.org".to_string(),
                port: 25565,
                category: "Fabric Modded".to_string(),
                supported_version: "1.20.1 Fabric".to_string(),
                description: "Calibrated for 60+ FPS on integrated GPUs. Pure Sodium + Lithium + FerriteCore synergy.".to_string(),
                is_favorite: true,
            },
            MinecraftServerEntry {
                id: "server-local-lan".to_string(),
                name: "Local Palace LAN (P2P Discovery)".to_string(),
                address: "127.0.0.1".to_string(),
                port: 25565,
                category: "Custom".to_string(),
                supported_version: "Any Active Version".to_string(),
                description: "Local area network game broadcasted by nearby peer explorers.".to_string(),
                is_favorite: false,
            },
        ]
    }

    /// Measures genuine TCP handshake latency to server address
    pub fn ping_server(address: &str, port: u16) -> ServerPingResult {
        let addr_str = format!("{}:{}", address, port);
        let start = Instant::now();

        match addr_str.to_socket_addrs() {
            Ok(mut addrs) => {
                if let Some(socket_addr) = addrs.next() {
                    let timeout = Duration::from_millis(1500);
                    match TcpStream::connect_timeout(&socket_addr, timeout) {
                        Ok(_) => {
                            let elapsed = start.elapsed().as_millis() as u64;
                            ServerPingResult {
                                is_online: true,
                                latency_ms: Some(elapsed),
                                status_message: format!("Online ({} ms)", elapsed),
                            }
                        }
                        Err(e) => ServerPingResult {
                            is_online: false,
                            latency_ms: None,
                            status_message: format!("Unreachable ({})", e.kind()),
                        },
                    }
                } else {
                    ServerPingResult {
                        is_online: false,
                        latency_ms: None,
                        status_message: "Address resolution yielded no socket".to_string(),
                    }
                }
            }
            Err(_) => ServerPingResult {
                is_online: false,
                latency_ms: None,
                status_message: "DNS lookup failed".to_string(),
            },
        }
    }
}
