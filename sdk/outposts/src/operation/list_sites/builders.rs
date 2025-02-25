// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_sites::_list_sites_output::ListSitesOutputBuilder;

pub use crate::operation::list_sites::_list_sites_input::ListSitesInputBuilder;

impl ListSitesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_sites::ListSitesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_sites::ListSitesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_sites();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListSites`.
///
/// <p>Lists the Outpost sites for your Amazon Web Services account. Use filters to return specific results.</p>
/// <p>Use filters to return specific results. If you specify multiple filters, the results include only the resources that match all of the specified filters. For a filter where you can specify multiple values, the results include items that match any of the values that you specify for the filter.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListSitesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_sites::builders::ListSitesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::list_sites::ListSitesOutput, crate::operation::list_sites::ListSitesError>
    for ListSitesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::list_sites::ListSitesOutput, crate::operation::list_sites::ListSitesError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListSitesFluentBuilder {
    /// Creates a new `ListSites`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListSites as a reference.
    pub fn as_input(&self) -> &crate::operation::list_sites::builders::ListSitesInputBuilder {
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
        crate::operation::list_sites::ListSitesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_sites::ListSitesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_sites::ListSites::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_sites::ListSites::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_sites::ListSitesOutput,
        crate::operation::list_sites::ListSitesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_sites::paginator::ListSitesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_sites::paginator::ListSitesPaginator {
        crate::operation::list_sites::paginator::ListSitesPaginator::new(self.handle, self.inner)
    }
    /// <p>The pagination token.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The pagination token.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum page size.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum page size.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum page size.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// Appends an item to `OperatingAddressCountryCodeFilter`.
    ///
    /// To override the contents of this collection use [`set_operating_address_country_code_filter`](Self::set_operating_address_country_code_filter).
    ///
    /// <p>Filters the results by country code.</p>
    pub fn operating_address_country_code_filter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.operating_address_country_code_filter(input.into());
        self
    }
    /// <p>Filters the results by country code.</p>
    pub fn set_operating_address_country_code_filter(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_operating_address_country_code_filter(input);
        self
    }
    /// <p>Filters the results by country code.</p>
    pub fn get_operating_address_country_code_filter(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_operating_address_country_code_filter()
    }
    /// Appends an item to `OperatingAddressStateOrRegionFilter`.
    ///
    /// To override the contents of this collection use [`set_operating_address_state_or_region_filter`](Self::set_operating_address_state_or_region_filter).
    ///
    /// <p>Filters the results by state or region.</p>
    pub fn operating_address_state_or_region_filter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.operating_address_state_or_region_filter(input.into());
        self
    }
    /// <p>Filters the results by state or region.</p>
    pub fn set_operating_address_state_or_region_filter(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_operating_address_state_or_region_filter(input);
        self
    }
    /// <p>Filters the results by state or region.</p>
    pub fn get_operating_address_state_or_region_filter(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_operating_address_state_or_region_filter()
    }
    /// Appends an item to `OperatingAddressCityFilter`.
    ///
    /// To override the contents of this collection use [`set_operating_address_city_filter`](Self::set_operating_address_city_filter).
    ///
    /// <p>Filters the results by city.</p>
    pub fn operating_address_city_filter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.operating_address_city_filter(input.into());
        self
    }
    /// <p>Filters the results by city.</p>
    pub fn set_operating_address_city_filter(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_operating_address_city_filter(input);
        self
    }
    /// <p>Filters the results by city.</p>
    pub fn get_operating_address_city_filter(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_operating_address_city_filter()
    }
}
