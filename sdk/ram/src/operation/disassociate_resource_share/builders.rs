// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_resource_share::_disassociate_resource_share_output::DisassociateResourceShareOutputBuilder;

pub use crate::operation::disassociate_resource_share::_disassociate_resource_share_input::DisassociateResourceShareInputBuilder;

impl DisassociateResourceShareInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disassociate_resource_share::DisassociateResourceShareOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_resource_share::DisassociateResourceShareError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disassociate_resource_share();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisassociateResourceShare`.
///
/// <p>Removes the specified principals or resources from participating in the specified resource share.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateResourceShareFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_resource_share::builders::DisassociateResourceShareInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disassociate_resource_share::DisassociateResourceShareOutput,
        crate::operation::disassociate_resource_share::DisassociateResourceShareError,
    > for DisassociateResourceShareFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disassociate_resource_share::DisassociateResourceShareOutput,
            crate::operation::disassociate_resource_share::DisassociateResourceShareError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisassociateResourceShareFluentBuilder {
    /// Creates a new `DisassociateResourceShare`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisassociateResourceShare as a reference.
    pub fn as_input(&self) -> &crate::operation::disassociate_resource_share::builders::DisassociateResourceShareInputBuilder {
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
        crate::operation::disassociate_resource_share::DisassociateResourceShareOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_resource_share::DisassociateResourceShareError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disassociate_resource_share::DisassociateResourceShare::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disassociate_resource_share::DisassociateResourceShare::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disassociate_resource_share::DisassociateResourceShareOutput,
        crate::operation::disassociate_resource_share::DisassociateResourceShareError,
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
    /// <p>Specifies <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the resource share that you want to remove resources or principals from.</p>
    pub fn resource_share_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_share_arn(input.into());
        self
    }
    /// <p>Specifies <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the resource share that you want to remove resources or principals from.</p>
    pub fn set_resource_share_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_share_arn(input);
        self
    }
    /// <p>Specifies <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the resource share that you want to remove resources or principals from.</p>
    pub fn get_resource_share_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_share_arn()
    }
    /// Appends an item to `resourceArns`.
    ///
    /// To override the contents of this collection use [`set_resource_arns`](Self::set_resource_arns).
    ///
    /// <p>Specifies a list of <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> for one or more resources that you want to remove from the resource share. After the operation runs, these resources are no longer shared with principals associated with the resource share.</p>
    pub fn resource_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arns(input.into());
        self
    }
    /// <p>Specifies a list of <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> for one or more resources that you want to remove from the resource share. After the operation runs, these resources are no longer shared with principals associated with the resource share.</p>
    pub fn set_resource_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_resource_arns(input);
        self
    }
    /// <p>Specifies a list of <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> for one or more resources that you want to remove from the resource share. After the operation runs, these resources are no longer shared with principals associated with the resource share.</p>
    pub fn get_resource_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_resource_arns()
    }
    /// Appends an item to `principals`.
    ///
    /// To override the contents of this collection use [`set_principals`](Self::set_principals).
    ///
    /// <p>Specifies a list of one or more principals that no longer are to have access to the resources in this resource share.</p>
    /// <p>You can include the following values:</p>
    /// <ul>
    /// <li> <p>An Amazon Web Services account ID, for example: <code>123456789012</code> </p> </li>
    /// <li> <p>An <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an organization in Organizations, for example: <code>organizations::123456789012:organization/o-exampleorgid</code> </p> </li>
    /// <li> <p>An ARN of an organizational unit (OU) in Organizations, for example: <code>organizations::123456789012:ou/o-exampleorgid/ou-examplerootid-exampleouid123</code> </p> </li>
    /// <li> <p>An ARN of an IAM role, for example: <code>iam::123456789012:role/rolename</code> </p> </li>
    /// <li> <p>An ARN of an IAM user, for example: <code>iam::123456789012user/username</code> </p> </li>
    /// </ul> <note>
    /// <p>Not all resource types can be shared with IAM roles and users. For more information, see <a href="https://docs.aws.amazon.com/ram/latest/userguide/permissions.html#permissions-rbp-supported-resource-types">Sharing with IAM roles and users</a> in the <i>Resource Access Manager User Guide</i>.</p>
    /// </note>
    pub fn principals(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.principals(input.into());
        self
    }
    /// <p>Specifies a list of one or more principals that no longer are to have access to the resources in this resource share.</p>
    /// <p>You can include the following values:</p>
    /// <ul>
    /// <li> <p>An Amazon Web Services account ID, for example: <code>123456789012</code> </p> </li>
    /// <li> <p>An <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an organization in Organizations, for example: <code>organizations::123456789012:organization/o-exampleorgid</code> </p> </li>
    /// <li> <p>An ARN of an organizational unit (OU) in Organizations, for example: <code>organizations::123456789012:ou/o-exampleorgid/ou-examplerootid-exampleouid123</code> </p> </li>
    /// <li> <p>An ARN of an IAM role, for example: <code>iam::123456789012:role/rolename</code> </p> </li>
    /// <li> <p>An ARN of an IAM user, for example: <code>iam::123456789012user/username</code> </p> </li>
    /// </ul> <note>
    /// <p>Not all resource types can be shared with IAM roles and users. For more information, see <a href="https://docs.aws.amazon.com/ram/latest/userguide/permissions.html#permissions-rbp-supported-resource-types">Sharing with IAM roles and users</a> in the <i>Resource Access Manager User Guide</i>.</p>
    /// </note>
    pub fn set_principals(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_principals(input);
        self
    }
    /// <p>Specifies a list of one or more principals that no longer are to have access to the resources in this resource share.</p>
    /// <p>You can include the following values:</p>
    /// <ul>
    /// <li> <p>An Amazon Web Services account ID, for example: <code>123456789012</code> </p> </li>
    /// <li> <p>An <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an organization in Organizations, for example: <code>organizations::123456789012:organization/o-exampleorgid</code> </p> </li>
    /// <li> <p>An ARN of an organizational unit (OU) in Organizations, for example: <code>organizations::123456789012:ou/o-exampleorgid/ou-examplerootid-exampleouid123</code> </p> </li>
    /// <li> <p>An ARN of an IAM role, for example: <code>iam::123456789012:role/rolename</code> </p> </li>
    /// <li> <p>An ARN of an IAM user, for example: <code>iam::123456789012user/username</code> </p> </li>
    /// </ul> <note>
    /// <p>Not all resource types can be shared with IAM roles and users. For more information, see <a href="https://docs.aws.amazon.com/ram/latest/userguide/permissions.html#permissions-rbp-supported-resource-types">Sharing with IAM roles and users</a> in the <i>Resource Access Manager User Guide</i>.</p>
    /// </note>
    pub fn get_principals(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_principals()
    }
    /// <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value.</a>.</p>
    /// <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>
    /// <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value.</a>.</p>
    /// <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>
    /// <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value.</a>.</p>
    /// <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>
    /// <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// Appends an item to `sources`.
    ///
    /// To override the contents of this collection use [`set_sources`](Self::set_sources).
    ///
    /// <p>Specifies from which source accounts the service principal no longer has access to the resources in this resource share.</p>
    pub fn sources(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sources(input.into());
        self
    }
    /// <p>Specifies from which source accounts the service principal no longer has access to the resources in this resource share.</p>
    pub fn set_sources(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_sources(input);
        self
    }
    /// <p>Specifies from which source accounts the service principal no longer has access to the resources in this resource share.</p>
    pub fn get_sources(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_sources()
    }
}
