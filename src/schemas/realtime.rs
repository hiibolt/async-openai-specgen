
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Realtime session object configuration.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct RealtimeSessionCreateRequest {
	/// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
	/// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, 
	/// single channel (mono), and little-endian byte order.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_format: Option<RealtimeSessionCreateRequestInputAudioFormat>,
	/// Configuration for input audio noise reduction. This can be set to `null` to turn off.
	/// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
	/// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_noise_reduction: Option<RealtimeSessionCreateRequestInputAudioNoiseReduction>,
	/// Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_transcription: Option<RealtimeSessionCreateRequestInputAudioTranscription>,
	/// The default system instructions (i.e. system message) prepended to model  calls. This field allows the client to guide the model on desired  responses. The model can be instructed on response content and format,  (e.g. "be extremely succinct", "act friendly", "here are examples of good  responses") and on audio behavior (e.g. "talk quickly", "inject emotion  into your voice", "laugh frequently"). The instructions are not guaranteed  to be followed by the model, but they provide guidance to the model on the desired behavior.
	/// 
	/// Note that the server sets default instructions which will be used if this  field is not set and are visible in the `session.created` event at the  start of the session.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/// Maximum number of output tokens for a single assistant response,
	/// inclusive of tool calls. Provide an integer between 1 and 4096 to
	/// limit output tokens, or `inf` for the maximum available tokens for a
	/// given model. Defaults to `inf`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_response_output_tokens: Option<RealtimeSessionCreateRequestMaxResponseOutputTokens>,
	/// The set of modalities the model can respond with. To disable audio,
	/// set this to ["text"].
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<RealtimeSessionCreateRequestItem>>,
	/// The Realtime model used for this session.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<RealtimeSessionCreateRequestModel>,
	/// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
	/// For `pcm16`, output audio is sampled at a rate of 24kHz.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_audio_format: Option<RealtimeSessionCreateRequestOutputAudioFormat>,
	/// Sampling temperature for the model, limited to [0.6, 1.2]. For audio models a temperature of 0.8 is highly recommended for best performance.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,
	/// How the model chooses tools. Options are `auto`, `none`, `required`, or 
	/// specify a function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<String>,
	/// Tools (functions) available to the model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<RealtimeSessionCreateRequestToolsItem>>,
	/// Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
	/// Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
	/// Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub turn_detection: Option<RealtimeSessionCreateRequestTurnDetection>,
	/// The voice the model uses to respond. Voice cannot be changed during the 
	/// session once the model has responded with audio at least once. Current 
	/// voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,
	/// `onyx`, `nova`, `sage`, `shimmer`, and `verse`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voice: Option<VoiceIdsShared>,
}
/// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
/// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, 
/// single channel (mono), and little-endian byte order.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestInputAudioFormat {
	Pcm16,
	#[serde(rename = "g711_ulaw")]
	G711Ulaw,
	#[serde(rename = "g711_alaw")]
	G711Alaw,
}
/// Configuration for input audio noise reduction. This can be set to `null` to turn off.
/// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
/// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct RealtimeSessionCreateRequestInputAudioNoiseReduction {
	/// Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionCreateRequestInputAudioNoiseReductionType>,
}
/// Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestInputAudioNoiseReductionType {
	#[serde(rename = "near_field")]
	NearField,
	#[serde(rename = "far_field")]
	FarField,
}
/// Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct RealtimeSessionCreateRequestInputAudioTranscription {
	/// The language of the input audio. Supplying the input language in
	/// [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format
	/// will improve accuracy and latency.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String>,
	/// The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
	/// An optional text to guide the model's style or continue a previous audio
	/// segment.
	/// For `whisper-1`, the [prompt is a list of keywords](https://platform.openai.com/docs/guides/speech-to-text#prompting).
	/// For `gpt-4o-transcribe` models, the prompt is a free text string, for example "expect words related to technology".
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestItem {
	Text,
	Audio,
}
/// Maximum number of output tokens for a single assistant response,
/// inclusive of tool calls. Provide an integer between 1 and 4096 to
/// limit output tokens, or `inf` for the maximum available tokens for a
/// given model. Defaults to `inf`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestMaxResponseOutputTokens {
	Integer(i64),
}
/// The Realtime model used for this session.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestModel {
	#[serde(rename = "gpt-4o-realtime-preview")]
	Gpt4ORealtimePreview,
	#[serde(rename = "gpt-4o-realtime-preview-2024-10-01")]
	Gpt4ORealtimePreview20241001,
	#[serde(rename = "gpt-4o-realtime-preview-2024-12-17")]
	Gpt4ORealtimePreview20241217,
	#[serde(rename = "gpt-4o-mini-realtime-preview")]
	Gpt4OMiniRealtimePreview,
	#[serde(rename = "gpt-4o-mini-realtime-preview-2024-12-17")]
	Gpt4OMiniRealtimePreview20241217,
}
/// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
/// For `pcm16`, output audio is sampled at a rate of 24kHz.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestOutputAudioFormat {
	Pcm16,
	#[serde(rename = "g711_ulaw")]
	G711Ulaw,
	#[serde(rename = "g711_alaw")]
	G711Alaw,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct RealtimeSessionCreateRequestToolsItem {
	/// The description of the function, including guidance on when and how 
	/// to call it, and guidance about what to tell the user when calling 
	/// (if anything).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The name of the function.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// Parameters of the function in JSON Schema.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<serde_json::Value>,
	/// The type of the tool, i.e. `function`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionCreateRequestToolsItemType>,
}
/// The type of the tool, i.e. `function`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestToolsItemType {
	Function,
}
/// Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
/// Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
/// Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct RealtimeSessionCreateRequestTurnDetection {
	/// Whether or not to automatically generate a response when a VAD stop event occurs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub create_response: Option<bool>,
	/// Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub eagerness: Option<RealtimeSessionCreateRequestTurnDetectionEagerness>,
	/// Whether or not to automatically interrupt any ongoing response with output to the default
	/// conversation (i.e. `conversation` of `auto`) when a VAD start event occurs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interrupt_response: Option<bool>,
	/// Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in 
	/// milliseconds). Defaults to 300ms.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prefix_padding_ms: Option<i64>,
	/// Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults 
	/// to 500ms. With shorter values the model will respond more quickly, 
	/// but may jump in on short pauses from the user.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub silence_duration_ms: Option<i64>,
	/// Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A 
	/// higher threshold will require louder audio to activate the model, and 
	/// thus might perform better in noisy environments.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold: Option<f64>,
	/// Type of turn detection.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionCreateRequestTurnDetectionType>,
}
/// Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestTurnDetectionEagerness {
	Low,
	Medium,
	High,
	Auto,
}
/// Type of turn detection.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeSessionCreateRequestTurnDetectionType {
	#[serde(rename = "server_vad")]
	ServerVad,
	#[serde(rename = "semantic_vad")]
	SemanticVad,
}
/// Realtime transcription session object configuration.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct RealtimeTranscriptionSessionCreateRequest {
	/// The set of items to include in the transcription. Current available items are:
	/// - `item.input_audio_transcription.logprobs`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<Vec<String>>,
	/// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
	/// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, 
	/// single channel (mono), and little-endian byte order.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_format: Option<RealtimeTranscriptionSessionCreateRequestInputAudioFormat>,
	/// Configuration for input audio noise reduction. This can be set to `null` to turn off.
	/// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
	/// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_noise_reduction: Option<RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction>,
	/// Configuration for input audio transcription. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_transcription: Option<RealtimeTranscriptionSessionCreateRequestInputAudioTranscription>,
	/// The set of modalities the model can respond with. To disable audio,
	/// set this to ["text"].
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<RealtimeTranscriptionSessionCreateRequestItem>>,
	/// Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
	/// Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
	/// Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub turn_detection: Option<RealtimeTranscriptionSessionCreateRequestTurnDetection>,
}
/// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
/// For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, 
/// single channel (mono), and little-endian byte order.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioFormat {
	Pcm16,
	#[serde(rename = "g711_ulaw")]
	G711Ulaw,
	#[serde(rename = "g711_alaw")]
	G711Alaw,
}
/// Configuration for input audio noise reduction. This can be set to `null` to turn off.
/// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
/// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction {
	/// Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReductionType>,
}
/// Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReductionType {
	#[serde(rename = "near_field")]
	NearField,
	#[serde(rename = "far_field")]
	FarField,
}
/// Configuration for input audio transcription. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct RealtimeTranscriptionSessionCreateRequestInputAudioTranscription {
	/// The language of the input audio. Supplying the input language in
	/// [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format
	/// will improve accuracy and latency.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String>,
	/// The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<RealtimeTranscriptionSessionCreateRequestInputAudioTranscriptionModel>,
	/// An optional text to guide the model's style or continue a previous audio
	/// segment.
	/// For `whisper-1`, the [prompt is a list of keywords](https://platform.openai.com/docs/guides/speech-to-text#prompting).
	/// For `gpt-4o-transcribe` models, the prompt is a free text string, for example "expect words related to technology".
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<String>,
}
/// The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioTranscriptionModel {
	#[serde(rename = "gpt-4o-transcribe")]
	Gpt4OTranscribe,
	#[serde(rename = "gpt-4o-mini-transcribe")]
	Gpt4OMiniTranscribe,
	#[serde(rename = "whisper-1")]
	Whisper1,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeTranscriptionSessionCreateRequestItem {
	Text,
	Audio,
}
/// Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
/// Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
/// Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct RealtimeTranscriptionSessionCreateRequestTurnDetection {
	/// Whether or not to automatically generate a response when a VAD stop event occurs. Not available for transcription sessions.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub create_response: Option<bool>,
	/// Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub eagerness: Option<RealtimeTranscriptionSessionCreateRequestTurnDetectionEagerness>,
	/// Whether or not to automatically interrupt any ongoing response with output to the default
	/// conversation (i.e. `conversation` of `auto`) when a VAD start event occurs. Not available for transcription sessions.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interrupt_response: Option<bool>,
	/// Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in 
	/// milliseconds). Defaults to 300ms.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prefix_padding_ms: Option<i64>,
	/// Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults 
	/// to 500ms. With shorter values the model will respond more quickly, 
	/// but may jump in on short pauses from the user.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub silence_duration_ms: Option<i64>,
	/// Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A 
	/// higher threshold will require louder audio to activate the model, and 
	/// thus might perform better in noisy environments.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold: Option<f64>,
	/// Type of turn detection.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeTranscriptionSessionCreateRequestTurnDetectionType>,
}
/// Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeTranscriptionSessionCreateRequestTurnDetectionEagerness {
	Low,
	Medium,
	High,
	Auto,
}
/// Type of turn detection.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeTranscriptionSessionCreateRequestTurnDetectionType {
	#[serde(rename = "server_vad")]
	ServerVad,
	#[serde(rename = "semantic_vad")]
	SemanticVad,
}
