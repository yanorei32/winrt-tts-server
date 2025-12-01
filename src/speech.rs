use once_cell::sync::OnceCell;
use ssml::Serialize;
use thiserror::Error;
use windows::{
    core::{HRESULT, HSTRING},
    Media::SpeechSynthesis::{SpeechSynthesizer, VoiceGender},
    Storage::Streams::DataReader,
};

use crate::model::{ApiRequest, Voice};

static VOICES: OnceCell<Vec<Voice>> = OnceCell::new();

pub fn init() -> Result<(), HRESULT> {
    let mut voices = vec![];

    for voice in SpeechSynthesizer::AllVoices()? {
        let display_name = voice.DisplayName()?.to_string();
        let id = voice.Id()?.to_string();
        let language = voice.Language()?.to_string();
        let description = voice.Description()?.to_string();
        let gender = match voice.Gender()? {
            VoiceGender::Male => "Male".to_string(),
            VoiceGender::Female => "Female".to_string(),
            _ => panic!(),
        };

        voices.push(Voice {
            display_name,
            id,
            language,
            description,
            gender,
        });
    }

    VOICES.set(voices).unwrap();

    Ok(())
}

pub fn voices() -> &'static [Voice] {
    VOICES.get().unwrap().as_ref()
}

#[derive(Error, Debug)]
pub enum SynthesisError {
    #[error("Failed to lookup voice")]
    LookupVoice,

    #[error("Failed to initialize SpeechSynthesizer: {0}")]
    InitializeSpeechSynthesizer(windows_result::Error),

    #[error("Failed to get options instance: {0}")]
    GetOptionsInstance(windows_result::Error),

    #[error("Failed to set property: {property} {error}")]
    SetProperty {
        property: &'static str,
        error: windows_result::Error,
    },

    #[error("Failed to encode to SSML: {0}")]
    SSMLError(ssml::Error),

    #[error("Failed to create SynthesizeSsmlToStream: {0}")]
    CreateSynthesizeSsmlToStreamAsync(windows_result::Error),

    #[error("Failed to SynthesizeSsmlToStream: {0}")]
    SynthesizeSsmlToStreamAsync(windows_result::Error),

    #[error("Failed to Stream.GetSize: {0}")]
    StreamGetSize(windows_result::Error),

    #[error("Failed to CreateDataReader: {0}")]
    CreateDataReader(windows_result::Error),

    #[error("Failed to create DataReader.LoadAsync: {0}")]
    CreateDataReaderLoadAsync(windows_result::Error),

    #[error("Failed to DataReader.LoadAsync: {0}")]
    DataReaderLoadAsync(windows_result::Error),

    #[error("Failed to DataReader.ReadBytes: {0}")]
    DataReaderReadBytes(windows_result::Error),
}

impl SynthesisError {
    pub fn is_client_error(&self) -> bool {
        match self {
            SynthesisError::LookupVoice => true,
            SynthesisError::InitializeSpeechSynthesizer(_) => false,
            SynthesisError::GetOptionsInstance(_) => false,
            SynthesisError::SetProperty {
                property: _,
                error: _,
            } => true,
            SynthesisError::SSMLError(_) => true,
            SynthesisError::CreateSynthesizeSsmlToStreamAsync(_) => false,
            SynthesisError::SynthesizeSsmlToStreamAsync(_) => true,
            SynthesisError::StreamGetSize(_) => false,
            SynthesisError::CreateDataReader(_) => false,
            SynthesisError::CreateDataReaderLoadAsync(_) => false,
            SynthesisError::DataReaderLoadAsync(_) => false,
            SynthesisError::DataReaderReadBytes(_) => false,
        }
    }
}

pub async fn synthesis(req: &ApiRequest) -> Result<Vec<u8>, SynthesisError> {
    let start_at = std::time::Instant::now();

    let voice = VOICES
        .get()
        .unwrap()
        .iter()
        .find(|voice| voice.id == req.voice_id)
        .ok_or(SynthesisError::LookupVoice)?;

    let synthesizer =
        SpeechSynthesizer::new().map_err(SynthesisError::InitializeSpeechSynthesizer)?;

    let options = synthesizer
        .Options()
        .map_err(SynthesisError::GetOptionsInstance)?;

    options
        .SetAudioVolume(req.audio_volume)
        .map_err(|error| SynthesisError::SetProperty {
            property: "AudioVolume",
            error,
        })?;

    options
        .SetSpeakingRate(req.speaking_rate)
        .map_err(|error| SynthesisError::SetProperty {
            property: "SpeakingRate",
            error,
        })?;

    options
        .SetAudioPitch(req.audio_pitch)
        .map_err(|error| SynthesisError::SetProperty {
            property: "AudioPitch",
            error,
        })?;

    let ssml = ssml::speak(Some(&voice.language), [&req.text])
        .serialize_to_string(&ssml::SerializeOptions::default())
        .map_err(SynthesisError::SSMLError)?;

    let stream = synthesizer
        .SynthesizeSsmlToStreamAsync(&HSTRING::from(&ssml))
        .map_err(SynthesisError::CreateSynthesizeSsmlToStreamAsync)?
        .await
        .map_err(SynthesisError::SynthesizeSsmlToStreamAsync)?;

    let size = stream.Size().map_err(SynthesisError::StreamGetSize)?;
    let reader = DataReader::CreateDataReader(&stream).map_err(SynthesisError::CreateDataReader)?;

    reader
        .LoadAsync(size as u32)
        .map_err(SynthesisError::CreateDataReaderLoadAsync)?
        .await
        .map_err(SynthesisError::DataReaderLoadAsync)?;

    let mut buffer = vec![0u8; size as usize];

    reader
        .ReadBytes(&mut buffer)
        .map_err(SynthesisError::DataReaderReadBytes)?;

    let end_at = std::time::Instant::now();

    tracing::info!("Synthesis: {:?}", end_at - start_at);

    Ok(buffer)
}
