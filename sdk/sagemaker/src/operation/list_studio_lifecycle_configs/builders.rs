// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_studio_lifecycle_configs::_list_studio_lifecycle_configs_output::ListStudioLifecycleConfigsOutputBuilder;

pub use crate::operation::list_studio_lifecycle_configs::_list_studio_lifecycle_configs_input::ListStudioLifecycleConfigsInputBuilder;

impl ListStudioLifecycleConfigsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_studio_lifecycle_configs();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListStudioLifecycleConfigs`.
///
/// <p>Lists the Studio Lifecycle Configurations in your Amazon Web Services Account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListStudioLifecycleConfigsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_studio_lifecycle_configs::builders::ListStudioLifecycleConfigsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsOutput,
        crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsError,
    > for ListStudioLifecycleConfigsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsOutput,
            crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListStudioLifecycleConfigsFluentBuilder {
    /// Creates a new `ListStudioLifecycleConfigs`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListStudioLifecycleConfigs as a reference.
    pub fn as_input(&self) -> &crate::operation::list_studio_lifecycle_configs::builders::ListStudioLifecycleConfigsInputBuilder {
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
        crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigs::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigs::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsOutput,
        crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_studio_lifecycle_configs::paginator::ListStudioLifecycleConfigsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_studio_lifecycle_configs::paginator::ListStudioLifecycleConfigsPaginator {
        crate::operation::list_studio_lifecycle_configs::paginator::ListStudioLifecycleConfigsPaginator::new(self.handle, self.inner)
    }
    /// <p>The total number of items to return in the response. If the total number of items available is more than the value specified, a <code>NextToken</code> is provided in the response. To resume pagination, provide the <code>NextToken</code> value in the as part of a subsequent call. The default value is 10.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The total number of items to return in the response. If the total number of items available is more than the value specified, a <code>NextToken</code> is provided in the response. To resume pagination, provide the <code>NextToken</code> value in the as part of a subsequent call. The default value is 10.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The total number of items to return in the response. If the total number of items available is more than the value specified, a <code>NextToken</code> is provided in the response. To resume pagination, provide the <code>NextToken</code> value in the as part of a subsequent call. The default value is 10.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>If the previous call to ListStudioLifecycleConfigs didn't return the full set of Lifecycle Configurations, the call returns a token for getting the next set of Lifecycle Configurations.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the previous call to ListStudioLifecycleConfigs didn't return the full set of Lifecycle Configurations, the call returns a token for getting the next set of Lifecycle Configurations.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If the previous call to ListStudioLifecycleConfigs didn't return the full set of Lifecycle Configurations, the call returns a token for getting the next set of Lifecycle Configurations.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>A string in the Lifecycle Configuration name. This filter returns only Lifecycle Configurations whose name contains the specified string.</p>
    pub fn name_contains(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name_contains(input.into());
        self
    }
    /// <p>A string in the Lifecycle Configuration name. This filter returns only Lifecycle Configurations whose name contains the specified string.</p>
    pub fn set_name_contains(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name_contains(input);
        self
    }
    /// <p>A string in the Lifecycle Configuration name. This filter returns only Lifecycle Configurations whose name contains the specified string.</p>
    pub fn get_name_contains(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name_contains()
    }
    /// <p>A parameter to search for the App Type to which the Lifecycle Configuration is attached.</p>
    pub fn app_type_equals(mut self, input: crate::types::StudioLifecycleConfigAppType) -> Self {
        self.inner = self.inner.app_type_equals(input);
        self
    }
    /// <p>A parameter to search for the App Type to which the Lifecycle Configuration is attached.</p>
    pub fn set_app_type_equals(mut self, input: ::std::option::Option<crate::types::StudioLifecycleConfigAppType>) -> Self {
        self.inner = self.inner.set_app_type_equals(input);
        self
    }
    /// <p>A parameter to search for the App Type to which the Lifecycle Configuration is attached.</p>
    pub fn get_app_type_equals(&self) -> &::std::option::Option<crate::types::StudioLifecycleConfigAppType> {
        self.inner.get_app_type_equals()
    }
    /// <p>A filter that returns only Lifecycle Configurations created on or before the specified time.</p>
    pub fn creation_time_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_before(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations created on or before the specified time.</p>
    pub fn set_creation_time_before(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_creation_time_before(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations created on or before the specified time.</p>
    pub fn get_creation_time_before(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_creation_time_before()
    }
    /// <p>A filter that returns only Lifecycle Configurations created on or after the specified time.</p>
    pub fn creation_time_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_after(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations created on or after the specified time.</p>
    pub fn set_creation_time_after(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_creation_time_after(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations created on or after the specified time.</p>
    pub fn get_creation_time_after(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_creation_time_after()
    }
    /// <p>A filter that returns only Lifecycle Configurations modified before the specified time.</p>
    pub fn modified_time_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.modified_time_before(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations modified before the specified time.</p>
    pub fn set_modified_time_before(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_modified_time_before(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations modified before the specified time.</p>
    pub fn get_modified_time_before(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_modified_time_before()
    }
    /// <p>A filter that returns only Lifecycle Configurations modified after the specified time.</p>
    pub fn modified_time_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.modified_time_after(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations modified after the specified time.</p>
    pub fn set_modified_time_after(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_modified_time_after(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations modified after the specified time.</p>
    pub fn get_modified_time_after(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_modified_time_after()
    }
    /// <p>The property used to sort results. The default value is CreationTime.</p>
    pub fn sort_by(mut self, input: crate::types::StudioLifecycleConfigSortKey) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>The property used to sort results. The default value is CreationTime.</p>
    pub fn set_sort_by(mut self, input: ::std::option::Option<crate::types::StudioLifecycleConfigSortKey>) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>The property used to sort results. The default value is CreationTime.</p>
    pub fn get_sort_by(&self) -> &::std::option::Option<crate::types::StudioLifecycleConfigSortKey> {
        self.inner.get_sort_by()
    }
    /// <p>The sort order. The default value is Descending.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrder) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>The sort order. The default value is Descending.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::SortOrder>) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
    /// <p>The sort order. The default value is Descending.</p>
    pub fn get_sort_order(&self) -> &::std::option::Option<crate::types::SortOrder> {
        self.inner.get_sort_order()
    }
}
