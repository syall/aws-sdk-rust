// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_data_share_consumer::_disassociate_data_share_consumer_output::DisassociateDataShareConsumerOutputBuilder;

pub use crate::operation::disassociate_data_share_consumer::_disassociate_data_share_consumer_input::DisassociateDataShareConsumerInputBuilder;

impl DisassociateDataShareConsumerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disassociate_data_share_consumer::DisassociateDataShareConsumerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_data_share_consumer::DisassociateDataShareConsumerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disassociate_data_share_consumer();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisassociateDataShareConsumer`.
///
/// <p>From a datashare consumer account, remove association for the specified datashare. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateDataShareConsumerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_data_share_consumer::builders::DisassociateDataShareConsumerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disassociate_data_share_consumer::DisassociateDataShareConsumerOutput,
        crate::operation::disassociate_data_share_consumer::DisassociateDataShareConsumerError,
    > for DisassociateDataShareConsumerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disassociate_data_share_consumer::DisassociateDataShareConsumerOutput,
            crate::operation::disassociate_data_share_consumer::DisassociateDataShareConsumerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisassociateDataShareConsumerFluentBuilder {
    /// Creates a new `DisassociateDataShareConsumer`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisassociateDataShareConsumer as a reference.
    pub fn as_input(&self) -> &crate::operation::disassociate_data_share_consumer::builders::DisassociateDataShareConsumerInputBuilder {
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
        crate::operation::disassociate_data_share_consumer::DisassociateDataShareConsumerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_data_share_consumer::DisassociateDataShareConsumerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disassociate_data_share_consumer::DisassociateDataShareConsumer::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disassociate_data_share_consumer::DisassociateDataShareConsumer::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disassociate_data_share_consumer::DisassociateDataShareConsumerOutput,
        crate::operation::disassociate_data_share_consumer::DisassociateDataShareConsumerError,
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
    /// <p>The Amazon Resource Name (ARN) of the datashare to remove association for. </p>
    pub fn data_share_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.data_share_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the datashare to remove association for. </p>
    pub fn set_data_share_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_data_share_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the datashare to remove association for. </p>
    pub fn get_data_share_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_data_share_arn()
    }
    /// <p>A value that specifies whether association for the datashare is removed from the entire account.</p>
    pub fn disassociate_entire_account(mut self, input: bool) -> Self {
        self.inner = self.inner.disassociate_entire_account(input);
        self
    }
    /// <p>A value that specifies whether association for the datashare is removed from the entire account.</p>
    pub fn set_disassociate_entire_account(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_disassociate_entire_account(input);
        self
    }
    /// <p>A value that specifies whether association for the datashare is removed from the entire account.</p>
    pub fn get_disassociate_entire_account(&self) -> &::std::option::Option<bool> {
        self.inner.get_disassociate_entire_account()
    }
    /// <p>The Amazon Resource Name (ARN) of the consumer that association for the datashare is removed from.</p>
    pub fn consumer_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.consumer_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the consumer that association for the datashare is removed from.</p>
    pub fn set_consumer_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_consumer_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the consumer that association for the datashare is removed from.</p>
    pub fn get_consumer_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_consumer_arn()
    }
    /// <p>From a datashare consumer account, removes association of a datashare from all the existing and future namespaces in the specified Amazon Web Services Region.</p>
    pub fn consumer_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.consumer_region(input.into());
        self
    }
    /// <p>From a datashare consumer account, removes association of a datashare from all the existing and future namespaces in the specified Amazon Web Services Region.</p>
    pub fn set_consumer_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_consumer_region(input);
        self
    }
    /// <p>From a datashare consumer account, removes association of a datashare from all the existing and future namespaces in the specified Amazon Web Services Region.</p>
    pub fn get_consumer_region(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_consumer_region()
    }
}
