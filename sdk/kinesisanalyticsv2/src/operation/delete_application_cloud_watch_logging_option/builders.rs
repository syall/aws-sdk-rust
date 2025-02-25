// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_application_cloud_watch_logging_option::_delete_application_cloud_watch_logging_option_output::DeleteApplicationCloudWatchLoggingOptionOutputBuilder;

pub use crate::operation::delete_application_cloud_watch_logging_option::_delete_application_cloud_watch_logging_option_input::DeleteApplicationCloudWatchLoggingOptionInputBuilder;

impl DeleteApplicationCloudWatchLoggingOptionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_application_cloud_watch_logging_option();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteApplicationCloudWatchLoggingOption`.
///
/// <p>Deletes an Amazon CloudWatch log stream from an Kinesis Data Analytics application. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteApplicationCloudWatchLoggingOptionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_application_cloud_watch_logging_option::builders::DeleteApplicationCloudWatchLoggingOptionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOptionOutput,
        crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOptionError,
    > for DeleteApplicationCloudWatchLoggingOptionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOptionOutput,
            crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOptionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteApplicationCloudWatchLoggingOptionFluentBuilder {
    /// Creates a new `DeleteApplicationCloudWatchLoggingOption`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteApplicationCloudWatchLoggingOption as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::delete_application_cloud_watch_logging_option::builders::DeleteApplicationCloudWatchLoggingOptionInputBuilder {
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
        crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOption::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOption::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOptionOutput,
        crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOptionError,
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
    /// <p>The application name.</p>
    pub fn application_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The application name.</p>
    pub fn set_application_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>The application name.</p>
    pub fn get_application_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_name()
    }
    /// <p>The version ID of the application. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You can retrieve the application version ID using <code>DescribeApplication</code>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    pub fn current_application_version_id(mut self, input: i64) -> Self {
        self.inner = self.inner.current_application_version_id(input);
        self
    }
    /// <p>The version ID of the application. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You can retrieve the application version ID using <code>DescribeApplication</code>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    pub fn set_current_application_version_id(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_current_application_version_id(input);
        self
    }
    /// <p>The version ID of the application. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You can retrieve the application version ID using <code>DescribeApplication</code>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    pub fn get_current_application_version_id(&self) -> &::std::option::Option<i64> {
        self.inner.get_current_application_version_id()
    }
    /// <p>The <code>CloudWatchLoggingOptionId</code> of the Amazon CloudWatch logging option to delete. You can get the <code>CloudWatchLoggingOptionId</code> by using the <code>DescribeApplication</code> operation. </p>
    pub fn cloud_watch_logging_option_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cloud_watch_logging_option_id(input.into());
        self
    }
    /// <p>The <code>CloudWatchLoggingOptionId</code> of the Amazon CloudWatch logging option to delete. You can get the <code>CloudWatchLoggingOptionId</code> by using the <code>DescribeApplication</code> operation. </p>
    pub fn set_cloud_watch_logging_option_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cloud_watch_logging_option_id(input);
        self
    }
    /// <p>The <code>CloudWatchLoggingOptionId</code> of the Amazon CloudWatch logging option to delete. You can get the <code>CloudWatchLoggingOptionId</code> by using the <code>DescribeApplication</code> operation. </p>
    pub fn get_cloud_watch_logging_option_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cloud_watch_logging_option_id()
    }
    /// <p>A value you use to implement strong concurrency for application updates. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You get the application's current <code>ConditionalToken</code> using <code>DescribeApplication</code>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    pub fn conditional_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.conditional_token(input.into());
        self
    }
    /// <p>A value you use to implement strong concurrency for application updates. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You get the application's current <code>ConditionalToken</code> using <code>DescribeApplication</code>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    pub fn set_conditional_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_conditional_token(input);
        self
    }
    /// <p>A value you use to implement strong concurrency for application updates. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You get the application's current <code>ConditionalToken</code> using <code>DescribeApplication</code>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    pub fn get_conditional_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_conditional_token()
    }
}
