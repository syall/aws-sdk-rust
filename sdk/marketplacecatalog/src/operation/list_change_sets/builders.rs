// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_change_sets::_list_change_sets_output::ListChangeSetsOutputBuilder;

pub use crate::operation::list_change_sets::_list_change_sets_input::ListChangeSetsInputBuilder;

impl ListChangeSetsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_change_sets::ListChangeSetsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_change_sets::ListChangeSetsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_change_sets();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListChangeSets`.
///
/// <p>Returns the list of change sets owned by the account being used to make the call. You can filter this list by providing any combination of <code>entityId</code>, <code>ChangeSetName</code>, and status. If you provide more than one filter, the API operation applies a logical AND between the filters.</p>
/// <p>You can describe a change during the 60-day request history retention period for API calls.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListChangeSetsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_change_sets::builders::ListChangeSetsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_change_sets::ListChangeSetsOutput,
        crate::operation::list_change_sets::ListChangeSetsError,
    > for ListChangeSetsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_change_sets::ListChangeSetsOutput,
            crate::operation::list_change_sets::ListChangeSetsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListChangeSetsFluentBuilder {
    /// Creates a new `ListChangeSets`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListChangeSets as a reference.
    pub fn as_input(&self) -> &crate::operation::list_change_sets::builders::ListChangeSetsInputBuilder {
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
        crate::operation::list_change_sets::ListChangeSetsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_change_sets::ListChangeSetsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_change_sets::ListChangeSets::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_change_sets::ListChangeSets::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_change_sets::ListChangeSetsOutput,
        crate::operation::list_change_sets::ListChangeSetsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_change_sets::paginator::ListChangeSetsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_change_sets::paginator::ListChangeSetsPaginator {
        crate::operation::list_change_sets::paginator::ListChangeSetsPaginator::new(self.handle, self.inner)
    }
    /// <p>The catalog related to the request. Fixed value: <code>AWSMarketplace</code> </p>
    pub fn catalog(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.catalog(input.into());
        self
    }
    /// <p>The catalog related to the request. Fixed value: <code>AWSMarketplace</code> </p>
    pub fn set_catalog(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_catalog(input);
        self
    }
    /// <p>The catalog related to the request. Fixed value: <code>AWSMarketplace</code> </p>
    pub fn get_catalog(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_catalog()
    }
    /// Appends an item to `FilterList`.
    ///
    /// To override the contents of this collection use [`set_filter_list`](Self::set_filter_list).
    ///
    /// <p>An array of filter objects.</p>
    pub fn filter_list(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filter_list(input);
        self
    }
    /// <p>An array of filter objects.</p>
    pub fn set_filter_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filter_list(input);
        self
    }
    /// <p>An array of filter objects.</p>
    pub fn get_filter_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filter_list()
    }
    /// <p>An object that contains two attributes, <code>SortBy</code> and <code>SortOrder</code>.</p>
    pub fn sort(mut self, input: crate::types::Sort) -> Self {
        self.inner = self.inner.sort(input);
        self
    }
    /// <p>An object that contains two attributes, <code>SortBy</code> and <code>SortOrder</code>.</p>
    pub fn set_sort(mut self, input: ::std::option::Option<crate::types::Sort>) -> Self {
        self.inner = self.inner.set_sort(input);
        self
    }
    /// <p>An object that contains two attributes, <code>SortBy</code> and <code>SortOrder</code>.</p>
    pub fn get_sort(&self) -> &::std::option::Option<crate::types::Sort> {
        self.inner.get_sort()
    }
    /// <p>The maximum number of results returned by a single call. This value must be provided in the next call to retrieve the next set of results. By default, this value is 20.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results returned by a single call. This value must be provided in the next call to retrieve the next set of results. By default, this value is 20.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results returned by a single call. This value must be provided in the next call to retrieve the next set of results. By default, this value is 20.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
