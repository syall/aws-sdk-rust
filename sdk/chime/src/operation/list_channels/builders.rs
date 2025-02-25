// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_channels::_list_channels_output::ListChannelsOutputBuilder;

pub use crate::operation::list_channels::_list_channels_input::ListChannelsInputBuilder;

impl ListChannelsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_channels::ListChannelsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_channels::ListChannelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_channels();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListChannels`.
///
/// <p>Lists all Channels created under a single Chime App as a paginated list. You can specify filters to narrow results.</p>
/// <p class="title"> <b>Functionality &amp; restrictions</b> </p>
/// <ul>
/// <li> <p>Use privacy = <code>PUBLIC</code> to retrieve all public channels in the account.</p> </li>
/// <li> <p>Only an <code>AppInstanceAdmin</code> can set privacy = <code>PRIVATE</code> to list the private channels in an account.</p> </li>
/// </ul> <note>
/// <p>The <code>x-amz-chime-bearer</code> request header is mandatory. Use the <code>AppInstanceUserArn</code> of the user that makes the API call as the value in the header.</p>
/// </note> <important>
/// <p> <b>This API is is no longer supported and will not be updated.</b> We recommend using the latest version, <a href="https://docs.aws.amazon.com/chime-sdk/latest/APIReference/API_messaging-chime_ListChannels.html">ListChannels</a>, in the Amazon Chime SDK.</p>
/// <p>Using the latest version requires migrating to a dedicated namespace. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/migrate-from-chm-namespace.html">Migrating from the Amazon Chime namespace</a> in the <i>Amazon Chime SDK Developer Guide</i>.</p>
/// </important>
#[deprecated(note = "Replaced by ListChannels in the Amazon Chime SDK Messaging Namespace")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListChannelsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_channels::builders::ListChannelsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_channels::ListChannelsOutput,
        crate::operation::list_channels::ListChannelsError,
    > for ListChannelsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_channels::ListChannelsOutput,
            crate::operation::list_channels::ListChannelsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListChannelsFluentBuilder {
    /// Creates a new `ListChannels`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListChannels as a reference.
    pub fn as_input(&self) -> &crate::operation::list_channels::builders::ListChannelsInputBuilder {
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
        crate::operation::list_channels::ListChannelsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_channels::ListChannelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_channels::ListChannels::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_channels::ListChannels::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_channels::ListChannelsOutput,
        crate::operation::list_channels::ListChannelsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_channels::paginator::ListChannelsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_channels::paginator::ListChannelsPaginator {
        crate::operation::list_channels::paginator::ListChannelsPaginator::new(self.handle, self.inner)
    }
    /// <p>The ARN of the <code>AppInstance</code>.</p>
    pub fn app_instance_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_instance_arn(input.into());
        self
    }
    /// <p>The ARN of the <code>AppInstance</code>.</p>
    pub fn set_app_instance_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_instance_arn(input);
        self
    }
    /// <p>The ARN of the <code>AppInstance</code>.</p>
    pub fn get_app_instance_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_instance_arn()
    }
    /// <p>The privacy setting. <code>PUBLIC</code> retrieves all the public channels. <code>PRIVATE</code> retrieves private channels. Only an <code>AppInstanceAdmin</code> can retrieve private channels. </p>
    pub fn privacy(mut self, input: crate::types::ChannelPrivacy) -> Self {
        self.inner = self.inner.privacy(input);
        self
    }
    /// <p>The privacy setting. <code>PUBLIC</code> retrieves all the public channels. <code>PRIVATE</code> retrieves private channels. Only an <code>AppInstanceAdmin</code> can retrieve private channels. </p>
    pub fn set_privacy(mut self, input: ::std::option::Option<crate::types::ChannelPrivacy>) -> Self {
        self.inner = self.inner.set_privacy(input);
        self
    }
    /// <p>The privacy setting. <code>PUBLIC</code> retrieves all the public channels. <code>PRIVATE</code> retrieves private channels. Only an <code>AppInstanceAdmin</code> can retrieve private channels. </p>
    pub fn get_privacy(&self) -> &::std::option::Option<crate::types::ChannelPrivacy> {
        self.inner.get_privacy()
    }
    /// <p>The maximum number of channels that you want to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of channels that you want to return.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of channels that you want to return.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token passed by previous API calls until all requested channels are returned.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token passed by previous API calls until all requested channels are returned.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token passed by previous API calls until all requested channels are returned.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub fn chime_bearer(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.chime_bearer(input.into());
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub fn set_chime_bearer(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_chime_bearer(input);
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub fn get_chime_bearer(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_chime_bearer()
    }
}
