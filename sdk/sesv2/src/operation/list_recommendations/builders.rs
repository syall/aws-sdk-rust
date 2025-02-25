// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_recommendations::_list_recommendations_output::ListRecommendationsOutputBuilder;

pub use crate::operation::list_recommendations::_list_recommendations_input::ListRecommendationsInputBuilder;

impl ListRecommendationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_recommendations::ListRecommendationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_recommendations::ListRecommendationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_recommendations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListRecommendations`.
///
/// <p>Lists the recommendations present in your Amazon SES account in the current Amazon Web Services Region.</p>
/// <p>You can execute this operation no more than once per second.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListRecommendationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_recommendations::builders::ListRecommendationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_recommendations::ListRecommendationsOutput,
        crate::operation::list_recommendations::ListRecommendationsError,
    > for ListRecommendationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_recommendations::ListRecommendationsOutput,
            crate::operation::list_recommendations::ListRecommendationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListRecommendationsFluentBuilder {
    /// Creates a new `ListRecommendations`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListRecommendations as a reference.
    pub fn as_input(&self) -> &crate::operation::list_recommendations::builders::ListRecommendationsInputBuilder {
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
        crate::operation::list_recommendations::ListRecommendationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_recommendations::ListRecommendationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_recommendations::ListRecommendations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_recommendations::ListRecommendations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_recommendations::ListRecommendationsOutput,
        crate::operation::list_recommendations::ListRecommendationsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_recommendations::paginator::ListRecommendationsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_recommendations::paginator::ListRecommendationsPaginator {
        crate::operation::list_recommendations::paginator::ListRecommendationsPaginator::new(self.handle, self.inner)
    }
    /// Adds a key-value pair to `Filter`.
    ///
    /// To override the contents of this collection use [`set_filter`](Self::set_filter).
    ///
    /// <p>Filters applied when retrieving recommendations. Can eiter be an individual filter, or combinations of <code>STATUS</code> and <code>IMPACT</code> or <code>STATUS</code> and <code>TYPE</code> </p>
    pub fn filter(mut self, k: crate::types::ListRecommendationsFilterKey, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.filter(k, v.into());
        self
    }
    /// <p>Filters applied when retrieving recommendations. Can eiter be an individual filter, or combinations of <code>STATUS</code> and <code>IMPACT</code> or <code>STATUS</code> and <code>TYPE</code> </p>
    pub fn set_filter(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<crate::types::ListRecommendationsFilterKey, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
    /// <p>Filters applied when retrieving recommendations. Can eiter be an individual filter, or combinations of <code>STATUS</code> and <code>IMPACT</code> or <code>STATUS</code> and <code>TYPE</code> </p>
    pub fn get_filter(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<crate::types::ListRecommendationsFilterKey, ::std::string::String>> {
        self.inner.get_filter()
    }
    /// <p>A token returned from a previous call to <code>ListRecommendations</code> to indicate the position in the list of recommendations.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token returned from a previous call to <code>ListRecommendations</code> to indicate the position in the list of recommendations.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token returned from a previous call to <code>ListRecommendations</code> to indicate the position in the list of recommendations.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The number of results to show in a single call to <code>ListRecommendations</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    /// <p>The value you specify has to be at least 1, and can be no more than 100.</p>
    pub fn page_size(mut self, input: i32) -> Self {
        self.inner = self.inner.page_size(input);
        self
    }
    /// <p>The number of results to show in a single call to <code>ListRecommendations</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    /// <p>The value you specify has to be at least 1, and can be no more than 100.</p>
    pub fn set_page_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_page_size(input);
        self
    }
    /// <p>The number of results to show in a single call to <code>ListRecommendations</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    /// <p>The value you specify has to be at least 1, and can be no more than 100.</p>
    pub fn get_page_size(&self) -> &::std::option::Option<i32> {
        self.inner.get_page_size()
    }
}
