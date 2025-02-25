// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_luna_client::_create_luna_client_output::CreateLunaClientOutputBuilder;

pub use crate::operation::create_luna_client::_create_luna_client_input::CreateLunaClientInputBuilder;

impl CreateLunaClientInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_luna_client::CreateLunaClientOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_luna_client::CreateLunaClientError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_luna_client();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateLunaClient`.
///
/// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="https://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="https://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p>
/// <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p>
/// <p>Creates an HSM client.</p>
#[deprecated(note = "This API is deprecated.")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateLunaClientFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_luna_client::builders::CreateLunaClientInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_luna_client::CreateLunaClientOutput,
        crate::operation::create_luna_client::CreateLunaClientError,
    > for CreateLunaClientFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_luna_client::CreateLunaClientOutput,
            crate::operation::create_luna_client::CreateLunaClientError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateLunaClientFluentBuilder {
    /// Creates a new `CreateLunaClient`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateLunaClient as a reference.
    pub fn as_input(&self) -> &crate::operation::create_luna_client::builders::CreateLunaClientInputBuilder {
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
        crate::operation::create_luna_client::CreateLunaClientOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_luna_client::CreateLunaClientError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_luna_client::CreateLunaClient::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_luna_client::CreateLunaClient::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_luna_client::CreateLunaClientOutput,
        crate::operation::create_luna_client::CreateLunaClientError,
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
    /// <p>The label for the client.</p>
    pub fn label(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.label(input.into());
        self
    }
    /// <p>The label for the client.</p>
    pub fn set_label(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_label(input);
        self
    }
    /// <p>The label for the client.</p>
    pub fn get_label(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_label()
    }
    /// <p>The contents of a Base64-Encoded X.509 v3 certificate to be installed on the HSMs used by this client.</p>
    pub fn certificate(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.certificate(input.into());
        self
    }
    /// <p>The contents of a Base64-Encoded X.509 v3 certificate to be installed on the HSMs used by this client.</p>
    pub fn set_certificate(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_certificate(input);
        self
    }
    /// <p>The contents of a Base64-Encoded X.509 v3 certificate to be installed on the HSMs used by this client.</p>
    pub fn get_certificate(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_certificate()
    }
}
