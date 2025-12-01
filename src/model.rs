use serde::{Deserialize, Serialize};

fn default_audio_volume() -> f64 {
    1.0
}

fn default_speaking_rate() -> f64 {
    1.0
}

fn default_audio_pitch() -> f64 {
    1.0
}

#[derive(Deserialize)]
pub struct ApiRequest {
    pub text: String,
    pub voice_id: String,

    #[serde(default = "default_audio_volume")]
    pub audio_volume: f64,

    #[serde(default = "default_speaking_rate")]
    pub speaking_rate: f64,

    #[serde(default = "default_audio_pitch")]
    pub audio_pitch: f64,
}

#[derive(Serialize, Debug)]
pub struct Voice {
    pub display_name: String,
    pub id: String,
    pub language: String,
    pub description: String,
    pub gender: String,
}
