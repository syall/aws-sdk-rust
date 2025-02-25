// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_model::_update_model_output::UpdateModelOutputBuilder;

pub use crate::operation::update_model::_update_model_input::UpdateModelInputBuilder;

impl UpdateModelInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_model::UpdateModelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_model::UpdateModelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_model();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateModel`.
///
/// <p>Updates a model in the account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateModelFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_model::builders::UpdateModelInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_model::UpdateModelOutput,
        crate::operation::update_model::UpdateModelError,
    > for UpdateModelFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_model::UpdateModelOutput,
            crate::operation::update_model::UpdateModelError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateModelFluentBuilder {
    /// Creates a new `UpdateModel`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateModel as a reference.
    pub fn as_input(&self) -> &crate::operation::update_model::builders::UpdateModelInputBuilder {
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
        crate::operation::update_model::UpdateModelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_model::UpdateModelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_model::UpdateModel::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_model::UpdateModel::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_model::UpdateModelOutput,
        crate::operation::update_model::UpdateModelError,
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
    /// <p>The name of the model to update.</p>
    pub fn model_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.model_name(input.into());
        self
    }
    /// <p>The name of the model to update.</p>
    pub fn set_model_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_model_name(input);
        self
    }
    /// <p>The name of the model to update.</p>
    pub fn get_model_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_model_name()
    }
    /// <p>Contains the configuration information for the S3 location being used to hold label data. </p>
    pub fn labels_input_configuration(mut self, input: crate::types::LabelsInputConfiguration) -> Self {
        self.inner = self.inner.labels_input_configuration(input);
        self
    }
    /// <p>Contains the configuration information for the S3 location being used to hold label data. </p>
    pub fn set_labels_input_configuration(mut self, input: ::std::option::Option<crate::types::LabelsInputConfiguration>) -> Self {
        self.inner = self.inner.set_labels_input_configuration(input);
        self
    }
    /// <p>Contains the configuration information for the S3 location being used to hold label data. </p>
    pub fn get_labels_input_configuration(&self) -> &::std::option::Option<crate::types::LabelsInputConfiguration> {
        self.inner.get_labels_input_configuration()
    }
    /// <p>The ARN of the model to update.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The ARN of the model to update.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The ARN of the model to update.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
}
