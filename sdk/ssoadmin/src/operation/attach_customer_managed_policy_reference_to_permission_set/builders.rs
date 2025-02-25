// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::attach_customer_managed_policy_reference_to_permission_set::_attach_customer_managed_policy_reference_to_permission_set_output::AttachCustomerManagedPolicyReferenceToPermissionSetOutputBuilder;

pub use crate::operation::attach_customer_managed_policy_reference_to_permission_set::_attach_customer_managed_policy_reference_to_permission_set_input::AttachCustomerManagedPolicyReferenceToPermissionSetInputBuilder;

impl AttachCustomerManagedPolicyReferenceToPermissionSetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.attach_customer_managed_policy_reference_to_permission_set();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AttachCustomerManagedPolicyReferenceToPermissionSet`.
///
/// <p>Attaches the specified customer managed policy to the specified <code>PermissionSet</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AttachCustomerManagedPolicyReferenceToPermissionSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::attach_customer_managed_policy_reference_to_permission_set::builders::AttachCustomerManagedPolicyReferenceToPermissionSetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSetOutput,
        crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSetError,
    > for AttachCustomerManagedPolicyReferenceToPermissionSetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSetOutput,
            crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AttachCustomerManagedPolicyReferenceToPermissionSetFluentBuilder {
    /// Creates a new `AttachCustomerManagedPolicyReferenceToPermissionSet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AttachCustomerManagedPolicyReferenceToPermissionSet as a reference.
    pub fn as_input(&self) -> &crate::operation::attach_customer_managed_policy_reference_to_permission_set::builders::AttachCustomerManagedPolicyReferenceToPermissionSetInputBuilder{
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
        crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSet::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
        crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSetOutput,
        crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSetError,
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
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. </p>
    pub fn instance_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_arn(input.into());
        self
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. </p>
    pub fn set_instance_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_arn(input);
        self
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. </p>
    pub fn get_instance_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_arn()
    }
    /// <p>The ARN of the <code>PermissionSet</code>.</p>
    pub fn permission_set_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.permission_set_arn(input.into());
        self
    }
    /// <p>The ARN of the <code>PermissionSet</code>.</p>
    pub fn set_permission_set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_permission_set_arn(input);
        self
    }
    /// <p>The ARN of the <code>PermissionSet</code>.</p>
    pub fn get_permission_set_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_permission_set_arn()
    }
    /// <p>Specifies the name and path of a customer managed policy. You must have an IAM policy that matches the name and path in each Amazon Web Services account where you want to deploy your permission set.</p>
    pub fn customer_managed_policy_reference(mut self, input: crate::types::CustomerManagedPolicyReference) -> Self {
        self.inner = self.inner.customer_managed_policy_reference(input);
        self
    }
    /// <p>Specifies the name and path of a customer managed policy. You must have an IAM policy that matches the name and path in each Amazon Web Services account where you want to deploy your permission set.</p>
    pub fn set_customer_managed_policy_reference(mut self, input: ::std::option::Option<crate::types::CustomerManagedPolicyReference>) -> Self {
        self.inner = self.inner.set_customer_managed_policy_reference(input);
        self
    }
    /// <p>Specifies the name and path of a customer managed policy. You must have an IAM policy that matches the name and path in each Amazon Web Services account where you want to deploy your permission set.</p>
    pub fn get_customer_managed_policy_reference(&self) -> &::std::option::Option<crate::types::CustomerManagedPolicyReference> {
        self.inner.get_customer_managed_policy_reference()
    }
}
