// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_time_series::_list_time_series_output::ListTimeSeriesOutputBuilder;

pub use crate::operation::list_time_series::_list_time_series_input::ListTimeSeriesInputBuilder;

impl ListTimeSeriesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_time_series::ListTimeSeriesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_time_series::ListTimeSeriesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_time_series();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListTimeSeries`.
///
/// <p>Retrieves a paginated list of time series (data streams).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListTimeSeriesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_time_series::builders::ListTimeSeriesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_time_series::ListTimeSeriesOutput,
        crate::operation::list_time_series::ListTimeSeriesError,
    > for ListTimeSeriesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_time_series::ListTimeSeriesOutput,
            crate::operation::list_time_series::ListTimeSeriesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListTimeSeriesFluentBuilder {
    /// Creates a new `ListTimeSeries`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListTimeSeries as a reference.
    pub fn as_input(&self) -> &crate::operation::list_time_series::builders::ListTimeSeriesInputBuilder {
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
        crate::operation::list_time_series::ListTimeSeriesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_time_series::ListTimeSeriesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_time_series::ListTimeSeries::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_time_series::ListTimeSeries::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_time_series::ListTimeSeriesOutput,
        crate::operation::list_time_series::ListTimeSeriesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_time_series::paginator::ListTimeSeriesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_time_series::paginator::ListTimeSeriesPaginator {
        crate::operation::list_time_series::paginator::ListTimeSeriesPaginator::new(self.handle, self.inner)
    }
    /// <p>The token to be used for the next set of paginated results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to be used for the next set of paginated results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token to be used for the next set of paginated results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return for each paginated request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return for each paginated request.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return for each paginated request.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The ID of the asset in which the asset property was created.</p>
    pub fn asset_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.asset_id(input.into());
        self
    }
    /// <p>The ID of the asset in which the asset property was created.</p>
    pub fn set_asset_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_asset_id(input);
        self
    }
    /// <p>The ID of the asset in which the asset property was created.</p>
    pub fn get_asset_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_asset_id()
    }
    /// <p>The alias prefix of the time series.</p>
    pub fn alias_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.alias_prefix(input.into());
        self
    }
    /// <p>The alias prefix of the time series.</p>
    pub fn set_alias_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_alias_prefix(input);
        self
    }
    /// <p>The alias prefix of the time series.</p>
    pub fn get_alias_prefix(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_alias_prefix()
    }
    /// <p>The type of the time series. The time series type can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>ASSOCIATED</code> – The time series is associated with an asset property.</p> </li>
    /// <li> <p> <code>DISASSOCIATED</code> – The time series isn't associated with any asset property.</p> </li>
    /// </ul>
    pub fn time_series_type(mut self, input: crate::types::ListTimeSeriesType) -> Self {
        self.inner = self.inner.time_series_type(input);
        self
    }
    /// <p>The type of the time series. The time series type can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>ASSOCIATED</code> – The time series is associated with an asset property.</p> </li>
    /// <li> <p> <code>DISASSOCIATED</code> – The time series isn't associated with any asset property.</p> </li>
    /// </ul>
    pub fn set_time_series_type(mut self, input: ::std::option::Option<crate::types::ListTimeSeriesType>) -> Self {
        self.inner = self.inner.set_time_series_type(input);
        self
    }
    /// <p>The type of the time series. The time series type can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>ASSOCIATED</code> – The time series is associated with an asset property.</p> </li>
    /// <li> <p> <code>DISASSOCIATED</code> – The time series isn't associated with any asset property.</p> </li>
    /// </ul>
    pub fn get_time_series_type(&self) -> &::std::option::Option<crate::types::ListTimeSeriesType> {
        self.inner.get_time_series_type()
    }
}
