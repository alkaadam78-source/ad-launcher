use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Indonesian,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl Language {
    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "id" | "in" | "indonesia" | "indonesian" => Language::Indonesian,
            _ => Language::English,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Indonesian => "id",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English (US)",
            Language::Indonesian => "Bahasa Indonesia",
        }
    }
}

pub fn t<'a>(key: &'a str, lang: Language) -> &'a str {
    match lang {
        Language::English => translate_en(key),
        Language::Indonesian => translate_id(key),
    }
}

fn translate_en<'a>(key: &'a str) -> &'a str {
    match key {
        "app.title" => "AD Launcher — The Art of Minecraft (Dark Baroque Edition)",
        "app.subtitle" => "THE ART OF MINECRAFT",
        "crafted.by" => "Crafted by AdamDevnoob",
        "nav.home" => "🏛  Home",
        "nav.instances" => "📜  Library",
        "nav.performance" => "⚖  Performance",
        "nav.shader_center" => "🕯  Shader Center",
        "nav.shader_lab" => "🧪  Shader Lab",
        "nav.storage" => "🗝  Storage Intel",
        "nav.mods" => "🧩  Mods",
        "nav.downloads" => "📥  Downloads",
        "nav.settings" => "⚙  Settings",
        "nav.account" => "👤  Account",
        "nav.about" => "⚜  About",
        "nav.multiplayer" => "🌐  Multiplayer & Social",
        "nav.fullscreen" => "⛶ Fullscreen (F11)",
        "nav.windowed" => "🗗 Windowed (F11)",
        "home.hero.title" => "COURT OF CLASSICAL MINECRAFT",
        "home.hero.subtitle" => "Caravaggio Chiaroscuro • Multi-Threaded Mesa OpenGL • Zero-Stutter Fabric Architecture",
        "home.play" => "⚔  LAUNCH MINECRAFT",
        "home.status.ready" => "Ready to launch",
        "settings.title" => "ROYAL PREFERENCES & ENGINE CONFIGURATION",
        "settings.save" => "💾  Save Preferences",
        "settings.language" => "Interface Language:",
        "settings.export_diagnostics" => "📋 Export Sanitized Diagnostic Report",
        "account.title" => "COURT REGISTRY & IDENTITY",
        "account.demo_btn" => "🎮 Enable Demo Mode (Safe Explorer Simulation)",
        _ => key,
    }
}

fn translate_id<'a>(key: &'a str) -> &'a str {
    match key {
        "app.title" => "AD Launcher — Seni Minecraft (Edisi Dark Baroque)",
        "app.subtitle" => "SENI MINECRAFT",
        "crafted.by" => "Karya AdamDevnoob",
        "nav.home" => "🏛  Beranda",
        "nav.instances" => "📜  Pustaka",
        "nav.performance" => "⚖  Performa",
        "nav.shader_center" => "🕯  Pusat Shader",
        "nav.shader_lab" => "🧪  Lab Shader",
        "nav.storage" => "🗝  Intel Penyimpanan",
        "nav.mods" => "🧩  Modifikasi",
        "nav.downloads" => "📥  Unduhan",
        "nav.settings" => "⚙  Pengaturan",
        "nav.account" => "👤  Akun",
        "nav.about" => "⚜  Tentang",
        "nav.multiplayer" => "🌐  Multiplayer & Sosial",
        "nav.fullscreen" => "⛶ Layar Penuh (F11)",
        "nav.windowed" => "🗗 Jendela (F11)",
        "home.hero.title" => "ISTANA MINECRAFT KLASIK",
        "home.hero.subtitle" => "Chiaroscuro Caravaggio • Mesa OpenGL Multi-Threaded • Arsitektur Fabric Anti-Lag",
        "home.play" => "⚔  LUNCURKAN MINECRAFT",
        "home.status.ready" => "Siap diluncurkan",
        "settings.title" => "PREFERENSI KERAJAAN & KONFIGURASI ENGINE",
        "settings.save" => "💾  Simpan Preferensi",
        "settings.language" => "Bahasa Antarmuka:",
        "settings.export_diagnostics" => "📋 Ekspor Laporan Diagnostik Bersih",
        "account.title" => "REGISTRASI & IDENTITAS KESULTANAN",
        "account.demo_btn" => "🎮 Aktifkan Mode Demo (Simulasi Penjelajah Aman)",
        _ => key,
    }
}
