// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_sync_resources::_list_sync_resources_output::ListSyncResourcesOutputBuilder;

pub use crate::operation::list_sync_resources::_list_sync_resources_input::ListSyncResourcesInputBuilder;

impl ListSyncResourcesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_sync_resources::ListSyncResourcesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_sync_resources::ListSyncResourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_sync_resources();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListSyncResources`.
///
/// <p>Lists the sync resources.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListSyncResourcesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_sync_resources::builders::ListSyncResourcesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_sync_resources::ListSyncResourcesOutput,
        crate::operation::list_sync_resources::ListSyncResourcesError,
    > for ListSyncResourcesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_sync_resources::ListSyncResourcesOutput,
            crate::operation::list_sync_resources::ListSyncResourcesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListSyncResourcesFluentBuilder {
    /// Creates a new `ListSyncResources`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListSyncResources as a reference.
    pub fn as_input(&self) -> &crate::operation::list_sync_resources::builders::ListSyncResourcesInputBuilder {
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
        crate::operation::list_sync_resources::ListSyncResourcesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_sync_resources::ListSyncResourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_sync_resources::ListSyncResources::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_sync_resources::ListSyncResources::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_sync_resources::ListSyncResourcesOutput,
        crate::operation::list_sync_resources::ListSyncResourcesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_sync_resources::paginator::ListSyncResourcesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_sync_resources::paginator::ListSyncResourcesPaginator {
        crate::operation::list_sync_resources::paginator::ListSyncResourcesPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of the workspace that contains the sync job.</p>
    pub fn workspace_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workspace_id(input.into());
        self
    }
    /// <p>The ID of the workspace that contains the sync job.</p>
    pub fn set_workspace_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workspace_id(input);
        self
    }
    /// <p>The ID of the workspace that contains the sync job.</p>
    pub fn get_workspace_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workspace_id()
    }
    /// <p>The sync source.</p> <note>
    /// <p>Currently the only supported syncSource is <code>SITEWISE </code>.</p>
    /// </note>
    pub fn sync_source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sync_source(input.into());
        self
    }
    /// <p>The sync source.</p> <note>
    /// <p>Currently the only supported syncSource is <code>SITEWISE </code>.</p>
    /// </note>
    pub fn set_sync_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sync_source(input);
        self
    }
    /// <p>The sync source.</p> <note>
    /// <p>Currently the only supported syncSource is <code>SITEWISE </code>.</p>
    /// </note>
    pub fn get_sync_source(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sync_source()
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>A list of objects that filter the request.</p>
    /// <p>The following filter combinations are supported:</p>
    /// <ul>
    /// <li> <p>Filter with state</p> </li>
    /// <li> <p>Filter with ResourceType and ResourceId</p> </li>
    /// <li> <p>Filter with ResourceType and ExternalId</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::SyncResourceFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>A list of objects that filter the request.</p>
    /// <p>The following filter combinations are supported:</p>
    /// <ul>
    /// <li> <p>Filter with state</p> </li>
    /// <li> <p>Filter with ResourceType and ResourceId</p> </li>
    /// <li> <p>Filter with ResourceType and ExternalId</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SyncResourceFilter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>A list of objects that filter the request.</p>
    /// <p>The following filter combinations are supported:</p>
    /// <ul>
    /// <li> <p>Filter with state</p> </li>
    /// <li> <p>Filter with ResourceType and ResourceId</p> </li>
    /// <li> <p>Filter with ResourceType and ExternalId</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SyncResourceFilter>> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of results to return at one time. The default is 50.</p>
    /// <p>Valid Range: Minimum value of 0. Maximum value of 200.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time. The default is 50.</p>
    /// <p>Valid Range: Minimum value of 0. Maximum value of 200.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time. The default is 50.</p>
    /// <p>Valid Range: Minimum value of 0. Maximum value of 200.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The string that specifies the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The string that specifies the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The string that specifies the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
