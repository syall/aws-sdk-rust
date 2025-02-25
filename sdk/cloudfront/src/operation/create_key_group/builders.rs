// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_key_group::_create_key_group_output::CreateKeyGroupOutputBuilder;

pub use crate::operation::create_key_group::_create_key_group_input::CreateKeyGroupInputBuilder;

impl CreateKeyGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_key_group::CreateKeyGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_key_group::CreateKeyGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_key_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateKeyGroup`.
///
/// <p>Creates a key group that you can use with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">CloudFront signed URLs and signed cookies</a>.</p>
/// <p>To create a key group, you must specify at least one public key for the key group. After you create a key group, you can reference it from one or more cache behaviors. When you reference a key group in a cache behavior, CloudFront requires signed URLs or signed cookies for all requests that match the cache behavior. The URLs or cookies must be signed with a private key whose corresponding public key is in the key group. The signed URL or cookie contains information about which public key CloudFront should use to verify the signature. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving private content</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateKeyGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_key_group::builders::CreateKeyGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_key_group::CreateKeyGroupOutput,
        crate::operation::create_key_group::CreateKeyGroupError,
    > for CreateKeyGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_key_group::CreateKeyGroupOutput,
            crate::operation::create_key_group::CreateKeyGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateKeyGroupFluentBuilder {
    /// Creates a new `CreateKeyGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateKeyGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::create_key_group::builders::CreateKeyGroupInputBuilder {
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
        crate::operation::create_key_group::CreateKeyGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_key_group::CreateKeyGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_key_group::CreateKeyGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_key_group::CreateKeyGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_key_group::CreateKeyGroupOutput,
        crate::operation::create_key_group::CreateKeyGroupError,
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
    /// <p>A key group configuration.</p>
    pub fn key_group_config(mut self, input: crate::types::KeyGroupConfig) -> Self {
        self.inner = self.inner.key_group_config(input);
        self
    }
    /// <p>A key group configuration.</p>
    pub fn set_key_group_config(mut self, input: ::std::option::Option<crate::types::KeyGroupConfig>) -> Self {
        self.inner = self.inner.set_key_group_config(input);
        self
    }
    /// <p>A key group configuration.</p>
    pub fn get_key_group_config(&self) -> &::std::option::Option<crate::types::KeyGroupConfig> {
        self.inner.get_key_group_config()
    }
}
