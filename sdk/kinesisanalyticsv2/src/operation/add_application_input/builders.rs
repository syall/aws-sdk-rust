// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_application_input::_add_application_input_output::AddApplicationInputOutputBuilder;

pub use crate::operation::add_application_input::_add_application_input_input::AddApplicationInputInputBuilder;

impl AddApplicationInputInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::add_application_input::AddApplicationInputOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_application_input::AddApplicationInputError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.add_application_input();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AddApplicationInput`.
///
/// <p> Adds a streaming source to your SQL-based Kinesis Data Analytics application. </p>
/// <p>You can add a streaming source when you create an application, or you can use this operation to add a streaming source after you create an application. For more information, see <code>CreateApplication</code>.</p>
/// <p>Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <code>DescribeApplication</code> operation to find the current application version. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AddApplicationInputFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::add_application_input::builders::AddApplicationInputInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::add_application_input::AddApplicationInputOutput,
        crate::operation::add_application_input::AddApplicationInputError,
    > for AddApplicationInputFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::add_application_input::AddApplicationInputOutput,
            crate::operation::add_application_input::AddApplicationInputError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AddApplicationInputFluentBuilder {
    /// Creates a new `AddApplicationInput`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AddApplicationInput as a reference.
    pub fn as_input(&self) -> &crate::operation::add_application_input::builders::AddApplicationInputInputBuilder {
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
        crate::operation::add_application_input::AddApplicationInputOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_application_input::AddApplicationInputError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::add_application_input::AddApplicationInput::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::add_application_input::AddApplicationInput::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::add_application_input::AddApplicationInputOutput,
        crate::operation::add_application_input::AddApplicationInputError,
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
    /// <p>The name of your existing application to which you want to add the streaming source.</p>
    pub fn application_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The name of your existing application to which you want to add the streaming source.</p>
    pub fn set_application_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>The name of your existing application to which you want to add the streaming source.</p>
    pub fn get_application_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_name()
    }
    /// <p>The current version of your application. You must provide the <code>ApplicationVersionID</code> or the <code>ConditionalToken</code>.You can use the <code>DescribeApplication</code> operation to find the current application version.</p>
    pub fn current_application_version_id(mut self, input: i64) -> Self {
        self.inner = self.inner.current_application_version_id(input);
        self
    }
    /// <p>The current version of your application. You must provide the <code>ApplicationVersionID</code> or the <code>ConditionalToken</code>.You can use the <code>DescribeApplication</code> operation to find the current application version.</p>
    pub fn set_current_application_version_id(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_current_application_version_id(input);
        self
    }
    /// <p>The current version of your application. You must provide the <code>ApplicationVersionID</code> or the <code>ConditionalToken</code>.You can use the <code>DescribeApplication</code> operation to find the current application version.</p>
    pub fn get_current_application_version_id(&self) -> &::std::option::Option<i64> {
        self.inner.get_current_application_version_id()
    }
    /// <p>The <code>Input</code> to add.</p>
    pub fn input(mut self, input: crate::types::Input) -> Self {
        self.inner = self.inner.input(input);
        self
    }
    /// <p>The <code>Input</code> to add.</p>
    pub fn set_input(mut self, input: ::std::option::Option<crate::types::Input>) -> Self {
        self.inner = self.inner.set_input(input);
        self
    }
    /// <p>The <code>Input</code> to add.</p>
    pub fn get_input(&self) -> &::std::option::Option<crate::types::Input> {
        self.inner.get_input()
    }
}
