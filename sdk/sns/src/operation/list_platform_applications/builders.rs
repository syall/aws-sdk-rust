// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_platform_applications::_list_platform_applications_output::ListPlatformApplicationsOutputBuilder;

pub use crate::operation::list_platform_applications::_list_platform_applications_input::ListPlatformApplicationsInputBuilder;

impl ListPlatformApplicationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_platform_applications::ListPlatformApplicationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_platform_applications::ListPlatformApplicationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_platform_applications();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListPlatformApplications`.
///
/// <p>Lists the platform application objects for the supported push notification services, such as APNS and GCM (Firebase Cloud Messaging). The results for <code>ListPlatformApplications</code> are paginated and return a limited list of applications, up to 100. If additional records are available after the first page results, then a NextToken string will be returned. To receive the next page, you call <code>ListPlatformApplications</code> using the NextToken string received from the previous call. When there are no more records to return, <code>NextToken</code> will be null. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
/// <p>This action is throttled at 15 transactions per second (TPS).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListPlatformApplicationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_platform_applications::builders::ListPlatformApplicationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_platform_applications::ListPlatformApplicationsOutput,
        crate::operation::list_platform_applications::ListPlatformApplicationsError,
    > for ListPlatformApplicationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_platform_applications::ListPlatformApplicationsOutput,
            crate::operation::list_platform_applications::ListPlatformApplicationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListPlatformApplicationsFluentBuilder {
    /// Creates a new `ListPlatformApplications`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListPlatformApplications as a reference.
    pub fn as_input(&self) -> &crate::operation::list_platform_applications::builders::ListPlatformApplicationsInputBuilder {
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
        crate::operation::list_platform_applications::ListPlatformApplicationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_platform_applications::ListPlatformApplicationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_platform_applications::ListPlatformApplications::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_platform_applications::ListPlatformApplications::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_platform_applications::ListPlatformApplicationsOutput,
        crate::operation::list_platform_applications::ListPlatformApplicationsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_platform_applications::paginator::ListPlatformApplicationsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_platform_applications::paginator::ListPlatformApplicationsPaginator {
        crate::operation::list_platform_applications::paginator::ListPlatformApplicationsPaginator::new(self.handle, self.inner)
    }
    /// <p>NextToken string is used when calling ListPlatformApplications action to retrieve additional records that are available after the first page results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>NextToken string is used when calling ListPlatformApplications action to retrieve additional records that are available after the first page results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>NextToken string is used when calling ListPlatformApplications action to retrieve additional records that are available after the first page results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
