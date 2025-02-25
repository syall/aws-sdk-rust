// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_app_instance_admin::_create_app_instance_admin_output::CreateAppInstanceAdminOutputBuilder;

pub use crate::operation::create_app_instance_admin::_create_app_instance_admin_input::CreateAppInstanceAdminInputBuilder;

impl CreateAppInstanceAdminInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_app_instance_admin::CreateAppInstanceAdminOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_app_instance_admin::CreateAppInstanceAdminError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_app_instance_admin();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAppInstanceAdmin`.
///
/// <p>Promotes an <code>AppInstanceUser</code> to an <code>AppInstanceAdmin</code>. The promoted user can perform the following actions. </p> <important>
/// <p> <b>This API is is no longer supported and will not be updated.</b> We recommend using the latest version, <a href="https://docs.aws.amazon.com/chime-sdk/latest/APIReference/API_identity-chime_CreateAppInstanceAdmin.html">CreateAppInstanceAdmin</a>, in the Amazon Chime SDK.</p>
/// <p>Using the latest version requires migrating to a dedicated namespace. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/migrate-from-chm-namespace.html">Migrating from the Amazon Chime namespace</a> in the <i>Amazon Chime SDK Developer Guide</i>.</p>
/// </important>
/// <ul>
/// <li> <p> <code>ChannelModerator</code> actions across all channels in the <code>AppInstance</code>.</p> </li>
/// <li> <p> <code>DeleteChannelMessage</code> actions.</p> </li>
/// </ul>
/// <p>Only an <code>AppInstanceUser</code> can be promoted to an <code>AppInstanceAdmin</code> role.</p>
#[deprecated(note = "Replaced by CreateAppInstanceAdmin in the Amazon Chime SDK Identity Namespace")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAppInstanceAdminFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_app_instance_admin::builders::CreateAppInstanceAdminInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_app_instance_admin::CreateAppInstanceAdminOutput,
        crate::operation::create_app_instance_admin::CreateAppInstanceAdminError,
    > for CreateAppInstanceAdminFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_app_instance_admin::CreateAppInstanceAdminOutput,
            crate::operation::create_app_instance_admin::CreateAppInstanceAdminError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateAppInstanceAdminFluentBuilder {
    /// Creates a new `CreateAppInstanceAdmin`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateAppInstanceAdmin as a reference.
    pub fn as_input(&self) -> &crate::operation::create_app_instance_admin::builders::CreateAppInstanceAdminInputBuilder {
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
        crate::operation::create_app_instance_admin::CreateAppInstanceAdminOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_app_instance_admin::CreateAppInstanceAdminError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_app_instance_admin::CreateAppInstanceAdmin::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_app_instance_admin::CreateAppInstanceAdmin::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_app_instance_admin::CreateAppInstanceAdminOutput,
        crate::operation::create_app_instance_admin::CreateAppInstanceAdminError,
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
    /// <p>The ARN of the administrator of the current <code>AppInstance</code>.</p>
    pub fn app_instance_admin_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_instance_admin_arn(input.into());
        self
    }
    /// <p>The ARN of the administrator of the current <code>AppInstance</code>.</p>
    pub fn set_app_instance_admin_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_instance_admin_arn(input);
        self
    }
    /// <p>The ARN of the administrator of the current <code>AppInstance</code>.</p>
    pub fn get_app_instance_admin_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_instance_admin_arn()
    }
    /// <p>The ARN of the <code>AppInstance</code>.</p>
    pub fn app_instance_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_instance_arn(input.into());
        self
    }
    /// <p>The ARN of the <code>AppInstance</code>.</p>
    pub fn set_app_instance_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_instance_arn(input);
        self
    }
    /// <p>The ARN of the <code>AppInstance</code>.</p>
    pub fn get_app_instance_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_instance_arn()
    }
}
