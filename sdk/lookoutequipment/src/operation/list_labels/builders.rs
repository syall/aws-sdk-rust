// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_labels::_list_labels_output::ListLabelsOutputBuilder;

pub use crate::operation::list_labels::_list_labels_input::ListLabelsInputBuilder;

impl ListLabelsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_labels::ListLabelsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_labels::ListLabelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_labels();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListLabels`.
///
/// <p> Provides a list of labels. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListLabelsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_labels::builders::ListLabelsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_labels::ListLabelsOutput,
        crate::operation::list_labels::ListLabelsError,
    > for ListLabelsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_labels::ListLabelsOutput,
            crate::operation::list_labels::ListLabelsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListLabelsFluentBuilder {
    /// Creates a new `ListLabels`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListLabels as a reference.
    pub fn as_input(&self) -> &crate::operation::list_labels::builders::ListLabelsInputBuilder {
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
        crate::operation::list_labels::ListLabelsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_labels::ListLabelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_labels::ListLabels::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_labels::ListLabels::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_labels::ListLabelsOutput,
        crate::operation::list_labels::ListLabelsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_labels::paginator::ListLabelsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_labels::paginator::ListLabelsPaginator {
        crate::operation::list_labels::paginator::ListLabelsPaginator::new(self.handle, self.inner)
    }
    /// <p> Retruns the name of the label group. </p>
    pub fn label_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.label_group_name(input.into());
        self
    }
    /// <p> Retruns the name of the label group. </p>
    pub fn set_label_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_label_group_name(input);
        self
    }
    /// <p> Retruns the name of the label group. </p>
    pub fn get_label_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_label_group_name()
    }
    /// <p> Returns all the labels with a end time equal to or later than the start time given. </p>
    pub fn interval_start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.interval_start_time(input);
        self
    }
    /// <p> Returns all the labels with a end time equal to or later than the start time given. </p>
    pub fn set_interval_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_interval_start_time(input);
        self
    }
    /// <p> Returns all the labels with a end time equal to or later than the start time given. </p>
    pub fn get_interval_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_interval_start_time()
    }
    /// <p> Returns all labels with a start time earlier than the end time given. </p>
    pub fn interval_end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.interval_end_time(input);
        self
    }
    /// <p> Returns all labels with a start time earlier than the end time given. </p>
    pub fn set_interval_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_interval_end_time(input);
        self
    }
    /// <p> Returns all labels with a start time earlier than the end time given. </p>
    pub fn get_interval_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_interval_end_time()
    }
    /// <p> Returns labels with a particular fault code. </p>
    pub fn fault_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.fault_code(input.into());
        self
    }
    /// <p> Returns labels with a particular fault code. </p>
    pub fn set_fault_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_fault_code(input);
        self
    }
    /// <p> Returns labels with a particular fault code. </p>
    pub fn get_fault_code(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_fault_code()
    }
    /// <p> Lists the labels that pertain to a particular piece of equipment. </p>
    pub fn equipment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.equipment(input.into());
        self
    }
    /// <p> Lists the labels that pertain to a particular piece of equipment. </p>
    pub fn set_equipment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_equipment(input);
        self
    }
    /// <p> Lists the labels that pertain to a particular piece of equipment. </p>
    pub fn get_equipment(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_equipment()
    }
    /// <p> An opaque pagination token indicating where to continue the listing of label groups. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p> An opaque pagination token indicating where to continue the listing of label groups. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p> An opaque pagination token indicating where to continue the listing of label groups. </p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p> Specifies the maximum number of labels to list. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p> Specifies the maximum number of labels to list. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p> Specifies the maximum number of labels to list. </p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
