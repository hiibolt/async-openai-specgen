
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateFineTuningCheckpointPermissionRequest {
	/// The project identifiers to grant access to.
	pub project_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateFineTuningJobRequest {
	/// The hyperparameters used for the fine-tuning job.
	/// This value is now deprecated in favor of `method`, and should be passed in under the `method` parameter.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hyperparameters: Option<CreateFineTuningJobRequestHyperparameters>,
	/// A list of integrations to enable for your fine-tuning job.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub integrations: Option<Vec<CreateFineTuningJobRequestIntegrationsItem>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub method: Option<FineTuneMethod>,
	/// The name of the model to fine-tune. You can select one of the
	/// [supported models](https://platform.openai.com/docs/guides/fine-tuning#which-models-can-be-fine-tuned).
	pub model: CreateFineTuningJobRequestModel,
	/// The seed controls the reproducibility of the job. Passing in the same seed and job parameters should produce the same results, but may differ in rare cases.
	/// If a seed is not specified, one will be generated for you.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub seed: Option<i64>,
	/// A string of up to 64 characters that will be added to your fine-tuned model name.
	/// 
	/// For example, a `suffix` of "custom-model-name" would produce a model name like `ft:gpt-4o-mini:openai:custom-model-name:7p4lURel`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub suffix: Option<String>,
	/// The ID of an uploaded file that contains training data.
	/// 
	/// See [upload file](https://platform.openai.com/docs/api-reference/files/create) for how to upload a file.
	/// 
	/// Your dataset must be formatted as a JSONL file. Additionally, you must upload your file with the purpose `fine-tune`.
	/// 
	/// The contents of the file should differ depending on if the model uses the [chat](https://platform.openai.com/docs/api-reference/fine-tuning/chat-input), [completions](https://platform.openai.com/docs/api-reference/fine-tuning/completions-input) format, or if the fine-tuning method uses the [preference](https://platform.openai.com/docs/api-reference/fine-tuning/preference-input) format.
	/// 
	/// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning) for more details.
	pub training_file: String,
	/// The ID of an uploaded file that contains validation data.
	/// 
	/// If you provide this file, the data is used to generate validation
	/// metrics periodically during fine-tuning. These metrics can be viewed in
	/// the fine-tuning results file.
	/// The same data should not be present in both train and validation files.
	/// 
	/// Your dataset must be formatted as a JSONL file. You must upload your file with the purpose `fine-tune`.
	/// 
	/// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning) for more details.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub validation_file: Option<String>,
}
/// The hyperparameters used for the fine-tuning job.
/// This value is now deprecated in favor of `method`, and should be passed in under the `method` parameter.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CreateFineTuningJobRequestHyperparameters {
	/// Number of examples in each batch. A larger batch size means that model parameters
	/// are updated less frequently, but with lower variance.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_size: Option<CreateFineTuningJobRequestHyperparametersBatchSize>,
	/// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid
	/// overfitting.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub learning_rate_multiplier: Option<CreateFineTuningJobRequestHyperparametersLearningRateMultiplier>,
	/// The number of epochs to train the model for. An epoch refers to one full cycle
	/// through the training dataset.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n_epochs: Option<CreateFineTuningJobRequestHyperparametersNEpochs>,
}
/// Number of examples in each batch. A larger batch size means that model parameters
/// are updated less frequently, but with lower variance.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestHyperparametersBatchSize {
	Auto(String),
	Integer(i64),
}
/// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid
/// overfitting.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestHyperparametersLearningRateMultiplier {
	Auto(String),
	Number(f64),
}
/// The number of epochs to train the model for. An epoch refers to one full cycle
/// through the training dataset.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestHyperparametersNEpochs {
	Auto(String),
	Integer(i64),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateFineTuningJobRequestIntegrationsItem {
	/// The type of integration to enable. Currently, only "wandb" (Weights and Biases) is supported.
	pub r#type: CreateFineTuningJobRequestIntegrationsItemType,
	/// The settings for your integration with Weights and Biases. This payload specifies the project that
	/// metrics will be sent to. Optionally, you can set an explicit display name for your run, add tags
	/// to your run, and set a default entity (team, username, etc) to be associated with your run.
	pub wandb: CreateFineTuningJobRequestIntegrationsItemWandb,
}
/// The type of integration to enable. Currently, only "wandb" (Weights and Biases) is supported.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestIntegrationsItemType {
	Wandb(String),
}
/// The settings for your integration with Weights and Biases. This payload specifies the project that
/// metrics will be sent to. Optionally, you can set an explicit display name for your run, add tags
/// to your run, and set a default entity (team, username, etc) to be associated with your run.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateFineTuningJobRequestIntegrationsItemWandb {
	/// The entity to use for the run. This allows you to set the team or username of the WandB user that you would
	/// like associated with the run. If not set, the default entity for the registered WandB API key is used.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity: Option<String>,
	/// A display name to set for the run. If not set, we will use the Job ID as the name.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// The name of the project that the new run will be created under.
	pub project: String,
	/// A list of tags to be attached to the newly created run. These tags are passed through directly to WandB. Some
	/// default tags are generated by OpenAI: "openai/finetune", "openai/{base-model}", "openai/{ftjob-abcdef}".
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<String>>,
}
/// Configuration for the DPO fine-tuning method.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct FineTuneDPOMethod {
	/// The hyperparameters used for the fine-tuning job.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hyperparameters: Option<FineTuneDPOMethodHyperparameters>,
}
/// The hyperparameters used for the fine-tuning job.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct FineTuneDPOMethodHyperparameters {
	/// Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_size: Option<FineTuneDPOMethodHyperparametersBatchSize>,
	/// The beta value for the DPO method. A higher beta value will increase the weight of the penalty between the policy and reference model.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub beta: Option<FineTuneDPOMethodHyperparametersBeta>,
	/// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub learning_rate_multiplier: Option<FineTuneDPOMethodHyperparametersLearningRateMultiplier>,
	/// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n_epochs: Option<FineTuneDPOMethodHyperparametersNEpochs>,
}
/// Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneDPOMethodHyperparametersBatchSize {
	Auto(String),
	Integer(i64),
}
/// The beta value for the DPO method. A higher beta value will increase the weight of the penalty between the policy and reference model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneDPOMethodHyperparametersBeta {
	Auto(String),
	Number(f64),
}
/// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneDPOMethodHyperparametersLearningRateMultiplier {
	Auto(String),
	Number(f64),
}
/// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneDPOMethodHyperparametersNEpochs {
	Auto(String),
	Integer(i64),
}
/// The method used for fine-tuning.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct FineTuneMethod {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dpo: Option<FineTuneDPOMethod>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supervised: Option<FineTuneSupervisedMethod>,
	/// The type of method. Is either `supervised` or `dpo`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<FineTuneMethodType>,
}
/// The type of method. Is either `supervised` or `dpo`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneMethodType {
	Supervised,
	Dpo,
}
/// Configuration for the supervised fine-tuning method.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct FineTuneSupervisedMethod {
	/// The hyperparameters used for the fine-tuning job.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hyperparameters: Option<FineTuneSupervisedMethodHyperparameters>,
}
/// The hyperparameters used for the fine-tuning job.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct FineTuneSupervisedMethodHyperparameters {
	/// Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_size: Option<FineTuneSupervisedMethodHyperparametersBatchSize>,
	/// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub learning_rate_multiplier: Option<FineTuneSupervisedMethodHyperparametersLearningRateMultiplier>,
	/// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n_epochs: Option<FineTuneSupervisedMethodHyperparametersNEpochs>,
}
/// Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneSupervisedMethodHyperparametersBatchSize {
	Auto(String),
	Integer(i64),
}
/// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneSupervisedMethodHyperparametersLearningRateMultiplier {
	Auto(String),
	Number(f64),
}
/// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum FineTuneSupervisedMethodHyperparametersNEpochs {
	Auto(String),
	Integer(i64),
}
