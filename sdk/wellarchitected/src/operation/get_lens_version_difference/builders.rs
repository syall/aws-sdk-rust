// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_lens_version_difference::_get_lens_version_difference_output::GetLensVersionDifferenceOutputBuilder;

pub use crate::operation::get_lens_version_difference::_get_lens_version_difference_input::GetLensVersionDifferenceInputBuilder;

impl GetLensVersionDifferenceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_lens_version_difference::GetLensVersionDifferenceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_lens_version_difference::GetLensVersionDifferenceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_lens_version_difference();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetLensVersionDifference`.
///
/// <p>Get lens version differences.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetLensVersionDifferenceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_lens_version_difference::builders::GetLensVersionDifferenceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_lens_version_difference::GetLensVersionDifferenceOutput,
        crate::operation::get_lens_version_difference::GetLensVersionDifferenceError,
    > for GetLensVersionDifferenceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_lens_version_difference::GetLensVersionDifferenceOutput,
            crate::operation::get_lens_version_difference::GetLensVersionDifferenceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetLensVersionDifferenceFluentBuilder {
    /// Creates a new `GetLensVersionDifference`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetLensVersionDifference as a reference.
    pub fn as_input(&self) -> &crate::operation::get_lens_version_difference::builders::GetLensVersionDifferenceInputBuilder {
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
        crate::operation::get_lens_version_difference::GetLensVersionDifferenceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_lens_version_difference::GetLensVersionDifferenceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_lens_version_difference::GetLensVersionDifference::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_lens_version_difference::GetLensVersionDifference::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_lens_version_difference::GetLensVersionDifferenceOutput,
        crate::operation::get_lens_version_difference::GetLensVersionDifferenceError,
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
    /// <p>The alias of the lens.</p>
    /// <p>For Amazon Web Services official lenses, this is either the lens alias, such as <code>serverless</code>, or the lens ARN, such as <code>arn:aws:wellarchitected:us-east-1::lens/serverless</code>. Note that some operations (such as ExportLens and CreateLensShare) are not permitted on Amazon Web Services official lenses.</p>
    /// <p>For custom lenses, this is the lens ARN, such as <code>arn:aws:wellarchitected:us-west-2:123456789012:lens/0123456789abcdef01234567890abcdef</code>. </p>
    /// <p>Each lens is identified by its <code>LensSummary$LensAlias</code>.</p>
    pub fn lens_alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.lens_alias(input.into());
        self
    }
    /// <p>The alias of the lens.</p>
    /// <p>For Amazon Web Services official lenses, this is either the lens alias, such as <code>serverless</code>, or the lens ARN, such as <code>arn:aws:wellarchitected:us-east-1::lens/serverless</code>. Note that some operations (such as ExportLens and CreateLensShare) are not permitted on Amazon Web Services official lenses.</p>
    /// <p>For custom lenses, this is the lens ARN, such as <code>arn:aws:wellarchitected:us-west-2:123456789012:lens/0123456789abcdef01234567890abcdef</code>. </p>
    /// <p>Each lens is identified by its <code>LensSummary$LensAlias</code>.</p>
    pub fn set_lens_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_lens_alias(input);
        self
    }
    /// <p>The alias of the lens.</p>
    /// <p>For Amazon Web Services official lenses, this is either the lens alias, such as <code>serverless</code>, or the lens ARN, such as <code>arn:aws:wellarchitected:us-east-1::lens/serverless</code>. Note that some operations (such as ExportLens and CreateLensShare) are not permitted on Amazon Web Services official lenses.</p>
    /// <p>For custom lenses, this is the lens ARN, such as <code>arn:aws:wellarchitected:us-west-2:123456789012:lens/0123456789abcdef01234567890abcdef</code>. </p>
    /// <p>Each lens is identified by its <code>LensSummary$LensAlias</code>.</p>
    pub fn get_lens_alias(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_lens_alias()
    }
    /// <p>The base version of the lens.</p>
    pub fn base_lens_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.base_lens_version(input.into());
        self
    }
    /// <p>The base version of the lens.</p>
    pub fn set_base_lens_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_base_lens_version(input);
        self
    }
    /// <p>The base version of the lens.</p>
    pub fn get_base_lens_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_base_lens_version()
    }
    /// <p>The lens version to target a difference for.</p>
    pub fn target_lens_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_lens_version(input.into());
        self
    }
    /// <p>The lens version to target a difference for.</p>
    pub fn set_target_lens_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_lens_version(input);
        self
    }
    /// <p>The lens version to target a difference for.</p>
    pub fn get_target_lens_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_lens_version()
    }
}
