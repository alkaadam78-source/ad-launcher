pub mod curseforge;
pub mod fabric;
pub mod forge;
pub mod modrinth;
pub mod neoforge;
pub mod quilt;

pub use curseforge::{CurseForgeClient, CurseForgeMod};
pub use fabric::FabricClient;
pub use modrinth::{ModrinthClient, ModrinthProject};
