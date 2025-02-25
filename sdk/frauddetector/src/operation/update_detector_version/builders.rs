// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_detector_version::_update_detector_version_output::UpdateDetectorVersionOutputBuilder;

pub use crate::operation::update_detector_version::_update_detector_version_input::UpdateDetectorVersionInputBuilder;

impl UpdateDetectorVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_detector_version::UpdateDetectorVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_detector_version::UpdateDetectorVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_detector_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateDetectorVersion`.
///
/// <p> Updates a detector version. The detector version attributes that you can update include models, external model endpoints, rules, rule execution mode, and description. You can only update a <code>DRAFT</code> detector version.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDetectorVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_detector_version::builders::UpdateDetectorVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_detector_version::UpdateDetectorVersionOutput,
        crate::operation::update_detector_version::UpdateDetectorVersionError,
    > for UpdateDetectorVersionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_detector_version::UpdateDetectorVersionOutput,
            crate::operation::update_detector_version::UpdateDetectorVersionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateDetectorVersionFluentBuilder {
    /// Creates a new `UpdateDetectorVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateDetectorVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::update_detector_version::builders::UpdateDetectorVersionInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_detector_version::UpdateDetectorVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_detector_version::UpdateDetectorVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_detector_version::UpdateDetectorVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_detector_version::UpdateDetectorVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_detector_version::UpdateDetectorVersionOutput,
        crate::operation::update_detector_version::UpdateDetectorVersionError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The parent detector ID for the detector version you want to update.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The parent detector ID for the detector version you want to update.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// <p>The parent detector ID for the detector version you want to update.</p>
    pub fn get_detector_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_detector_id()
    }
    /// <p>The detector version ID. </p>
    pub fn detector_version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_version_id(input.into());
        self
    }
    /// <p>The detector version ID. </p>
    pub fn set_detector_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_version_id(input);
        self
    }
    /// <p>The detector version ID. </p>
    pub fn get_detector_version_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_detector_version_id()
    }
    /// Appends an item to `externalModelEndpoints`.
    ///
    /// To override the contents of this collection use [`set_external_model_endpoints`](Self::set_external_model_endpoints).
    ///
    /// <p>The Amazon SageMaker model endpoints to include in the detector version.</p>
    pub fn external_model_endpoints(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.external_model_endpoints(input.into());
        self
    }
    /// <p>The Amazon SageMaker model endpoints to include in the detector version.</p>
    pub fn set_external_model_endpoints(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_external_model_endpoints(input);
        self
    }
    /// <p>The Amazon SageMaker model endpoints to include in the detector version.</p>
    pub fn get_external_model_endpoints(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_external_model_endpoints()
    }
    /// Appends an item to `rules`.
    ///
    /// To override the contents of this collection use [`set_rules`](Self::set_rules).
    ///
    /// <p>The rules to include in the detector version.</p>
    pub fn rules(mut self, input: crate::types::Rule) -> Self {
        self.inner = self.inner.rules(input);
        self
    }
    /// <p>The rules to include in the detector version.</p>
    pub fn set_rules(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Rule>>) -> Self {
        self.inner = self.inner.set_rules(input);
        self
    }
    /// <p>The rules to include in the detector version.</p>
    pub fn get_rules(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Rule>> {
        self.inner.get_rules()
    }
    /// <p>The detector version description. </p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The detector version description. </p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The detector version description. </p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// Appends an item to `modelVersions`.
    ///
    /// To override the contents of this collection use [`set_model_versions`](Self::set_model_versions).
    ///
    /// <p>The model versions to include in the detector version.</p>
    pub fn model_versions(mut self, input: crate::types::ModelVersion) -> Self {
        self.inner = self.inner.model_versions(input);
        self
    }
    /// <p>The model versions to include in the detector version.</p>
    pub fn set_model_versions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ModelVersion>>) -> Self {
        self.inner = self.inner.set_model_versions(input);
        self
    }
    /// <p>The model versions to include in the detector version.</p>
    pub fn get_model_versions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ModelVersion>> {
        self.inner.get_model_versions()
    }
    /// <p>The rule execution mode to add to the detector.</p>
    /// <p>If you specify <code>FIRST_MATCHED</code>, Amazon Fraud Detector evaluates rules sequentially, first to last, stopping at the first matched rule. Amazon Fraud dectector then provides the outcomes for that single rule.</p>
    /// <p>If you specifiy <code>ALL_MATCHED</code>, Amazon Fraud Detector evaluates all rules and returns the outcomes for all matched rules. You can define and edit the rule mode at the detector version level, when it is in draft status.</p>
    /// <p>The default behavior is <code>FIRST_MATCHED</code>.</p>
    pub fn rule_execution_mode(mut self, input: crate::types::RuleExecutionMode) -> Self {
        self.inner = self.inner.rule_execution_mode(input);
        self
    }
    /// <p>The rule execution mode to add to the detector.</p>
    /// <p>If you specify <code>FIRST_MATCHED</code>, Amazon Fraud Detector evaluates rules sequentially, first to last, stopping at the first matched rule. Amazon Fraud dectector then provides the outcomes for that single rule.</p>
    /// <p>If you specifiy <code>ALL_MATCHED</code>, Amazon Fraud Detector evaluates all rules and returns the outcomes for all matched rules. You can define and edit the rule mode at the detector version level, when it is in draft status.</p>
    /// <p>The default behavior is <code>FIRST_MATCHED</code>.</p>
    pub fn set_rule_execution_mode(mut self, input: ::std::option::Option<crate::types::RuleExecutionMode>) -> Self {
        self.inner = self.inner.set_rule_execution_mode(input);
        self
    }
    /// <p>The rule execution mode to add to the detector.</p>
    /// <p>If you specify <code>FIRST_MATCHED</code>, Amazon Fraud Detector evaluates rules sequentially, first to last, stopping at the first matched rule. Amazon Fraud dectector then provides the outcomes for that single rule.</p>
    /// <p>If you specifiy <code>ALL_MATCHED</code>, Amazon Fraud Detector evaluates all rules and returns the outcomes for all matched rules. You can define and edit the rule mode at the detector version level, when it is in draft status.</p>
    /// <p>The default behavior is <code>FIRST_MATCHED</code>.</p>
    pub fn get_rule_execution_mode(&self) -> &::std::option::Option<crate::types::RuleExecutionMode> {
        self.inner.get_rule_execution_mode()
    }
}
