// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_workflow_step::_get_workflow_step_output::GetWorkflowStepOutputBuilder;

pub use crate::operation::get_workflow_step::_get_workflow_step_input::GetWorkflowStepInputBuilder;

impl GetWorkflowStepInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_workflow_step::GetWorkflowStepOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_workflow_step::GetWorkflowStepError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_workflow_step();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetWorkflowStep`.
///
/// <p>Get a step in the migration workflow.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetWorkflowStepFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_workflow_step::builders::GetWorkflowStepInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_workflow_step::GetWorkflowStepOutput,
        crate::operation::get_workflow_step::GetWorkflowStepError,
    > for GetWorkflowStepFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_workflow_step::GetWorkflowStepOutput,
            crate::operation::get_workflow_step::GetWorkflowStepError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetWorkflowStepFluentBuilder {
    /// Creates a new `GetWorkflowStep`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetWorkflowStep as a reference.
    pub fn as_input(&self) -> &crate::operation::get_workflow_step::builders::GetWorkflowStepInputBuilder {
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
        crate::operation::get_workflow_step::GetWorkflowStepOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_workflow_step::GetWorkflowStepError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_workflow_step::GetWorkflowStep::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_workflow_step::GetWorkflowStep::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_workflow_step::GetWorkflowStepOutput,
        crate::operation::get_workflow_step::GetWorkflowStepError,
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
    /// <p>The ID of the migration workflow.</p>
    pub fn workflow_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workflow_id(input.into());
        self
    }
    /// <p>The ID of the migration workflow.</p>
    pub fn set_workflow_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workflow_id(input);
        self
    }
    /// <p>The ID of the migration workflow.</p>
    pub fn get_workflow_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workflow_id()
    }
    /// <p>desThe ID of the step group.</p>
    pub fn step_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.step_group_id(input.into());
        self
    }
    /// <p>desThe ID of the step group.</p>
    pub fn set_step_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_step_group_id(input);
        self
    }
    /// <p>desThe ID of the step group.</p>
    pub fn get_step_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_step_group_id()
    }
    /// <p>The ID of the step.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the step.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of the step.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
}
