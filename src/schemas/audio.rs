
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateSpeechRequest {
	/// The text to generate audio for. The maximum length is 4096 characters.
	pub input: String,
	/// Control the voice of your generated audio with additional instructions. Does not work with `tts-1` or `tts-1-hd`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/// One of the available [TTS models](https://platform.openai.com/docs/models#tts): `tts-1`, `tts-1-hd` or `gpt-4o-mini-tts`.
	pub model: CreateSpeechRequestModel,
	/// The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateSpeechRequestResponseFormat>,
	/// The speed of the generated audio. Select a value from `0.25` to `4.0`. `1.0` is the default.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub speed: Option<f64>,
	/// The voice to use when generating the audio. Supported voices are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `onyx`, `nova`, `sage`, `shimmer`, and `verse`. Previews of the voices are available in the [Text to speech guide](https://platform.openai.com/docs/guides/text-to-speech#voice-options).
	pub voice: VoiceIdsShared,
}
/// The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CreateSpeechRequestResponseFormat {
	Mp3,
	Opus,
	Aac,
	Flac,
	Wav,
	Pcm,
}
