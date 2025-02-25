// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::search_available_phone_numbers::_search_available_phone_numbers_output::SearchAvailablePhoneNumbersOutputBuilder;

pub use crate::operation::search_available_phone_numbers::_search_available_phone_numbers_input::SearchAvailablePhoneNumbersInputBuilder;

impl SearchAvailablePhoneNumbersInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.search_available_phone_numbers();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SearchAvailablePhoneNumbers`.
///
/// <p>Searches for available phone numbers that you can claim to your Amazon Connect instance or traffic distribution group. If the provided <code>TargetArn</code> is a traffic distribution group, you can call this API in both Amazon Web Services Regions associated with the traffic distribution group.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SearchAvailablePhoneNumbersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersOutput,
        crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersError,
    > for SearchAvailablePhoneNumbersFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersOutput,
            crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SearchAvailablePhoneNumbersFluentBuilder {
    /// Creates a new `SearchAvailablePhoneNumbers`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SearchAvailablePhoneNumbers as a reference.
    pub fn as_input(&self) -> &crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersInputBuilder {
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
        crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbers::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbers::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersOutput,
        crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::search_available_phone_numbers::paginator::SearchAvailablePhoneNumbersPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::search_available_phone_numbers::paginator::SearchAvailablePhoneNumbersPaginator {
        crate::operation::search_available_phone_numbers::paginator::SearchAvailablePhoneNumbersPaginator::new(self.handle, self.inner)
    }
    /// <p>The Amazon Resource Name (ARN) for Amazon Connect instances or traffic distribution groups that phone number inbound traffic is routed through. You must enter <code>InstanceId</code> or <code>TargetArn</code>. </p>
    pub fn target_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for Amazon Connect instances or traffic distribution groups that phone number inbound traffic is routed through. You must enter <code>InstanceId</code> or <code>TargetArn</code>. </p>
    pub fn set_target_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) for Amazon Connect instances or traffic distribution groups that phone number inbound traffic is routed through. You must enter <code>InstanceId</code> or <code>TargetArn</code>. </p>
    pub fn get_target_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_arn()
    }
    /// <p>The identifier of the Amazon Connect instance that phone numbers are claimed to. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance. You must enter <code>InstanceId</code> or <code>TargetArn</code>. </p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance that phone numbers are claimed to. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance. You must enter <code>InstanceId</code> or <code>TargetArn</code>. </p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The identifier of the Amazon Connect instance that phone numbers are claimed to. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance. You must enter <code>InstanceId</code> or <code>TargetArn</code>. </p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>The ISO country code.</p>
    pub fn phone_number_country_code(mut self, input: crate::types::PhoneNumberCountryCode) -> Self {
        self.inner = self.inner.phone_number_country_code(input);
        self
    }
    /// <p>The ISO country code.</p>
    pub fn set_phone_number_country_code(mut self, input: ::std::option::Option<crate::types::PhoneNumberCountryCode>) -> Self {
        self.inner = self.inner.set_phone_number_country_code(input);
        self
    }
    /// <p>The ISO country code.</p>
    pub fn get_phone_number_country_code(&self) -> &::std::option::Option<crate::types::PhoneNumberCountryCode> {
        self.inner.get_phone_number_country_code()
    }
    /// <p>The type of phone number.</p>
    pub fn phone_number_type(mut self, input: crate::types::PhoneNumberType) -> Self {
        self.inner = self.inner.phone_number_type(input);
        self
    }
    /// <p>The type of phone number.</p>
    pub fn set_phone_number_type(mut self, input: ::std::option::Option<crate::types::PhoneNumberType>) -> Self {
        self.inner = self.inner.set_phone_number_type(input);
        self
    }
    /// <p>The type of phone number.</p>
    pub fn get_phone_number_type(&self) -> &::std::option::Option<crate::types::PhoneNumberType> {
        self.inner.get_phone_number_type()
    }
    /// <p>The prefix of the phone number. If provided, it must contain <code>+</code> as part of the country code.</p>
    pub fn phone_number_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.phone_number_prefix(input.into());
        self
    }
    /// <p>The prefix of the phone number. If provided, it must contain <code>+</code> as part of the country code.</p>
    pub fn set_phone_number_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_phone_number_prefix(input);
        self
    }
    /// <p>The prefix of the phone number. If provided, it must contain <code>+</code> as part of the country code.</p>
    pub fn get_phone_number_prefix(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_phone_number_prefix()
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
