// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deregister_workspace_directory::_deregister_workspace_directory_output::DeregisterWorkspaceDirectoryOutputBuilder;

pub use crate::operation::deregister_workspace_directory::_deregister_workspace_directory_input::DeregisterWorkspaceDirectoryInputBuilder;

impl DeregisterWorkspaceDirectoryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::deregister_workspace_directory::DeregisterWorkspaceDirectoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deregister_workspace_directory::DeregisterWorkspaceDirectoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.deregister_workspace_directory();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeregisterWorkspaceDirectory`.
///
/// <p>Deregisters the specified directory. This operation is asynchronous and returns before the WorkSpace directory is deregistered. If any WorkSpaces are registered to this directory, you must remove them before you can deregister the directory.</p> <note>
/// <p>Simple AD and AD Connector are made available to you free of charge to use with WorkSpaces. If there are no WorkSpaces being used with your Simple AD or AD Connector directory for 30 consecutive days, this directory will be automatically deregistered for use with Amazon WorkSpaces, and you will be charged for this directory as per the <a href="http://aws.amazon.com/directoryservice/pricing/">Directory Service pricing terms</a>.</p>
/// <p>To delete empty directories, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/delete-workspaces-directory.html"> Delete the Directory for Your WorkSpaces</a>. If you delete your Simple AD or AD Connector directory, you can always create a new one when you want to start using WorkSpaces again.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeregisterWorkspaceDirectoryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::deregister_workspace_directory::builders::DeregisterWorkspaceDirectoryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::deregister_workspace_directory::DeregisterWorkspaceDirectoryOutput,
        crate::operation::deregister_workspace_directory::DeregisterWorkspaceDirectoryError,
    > for DeregisterWorkspaceDirectoryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::deregister_workspace_directory::DeregisterWorkspaceDirectoryOutput,
            crate::operation::deregister_workspace_directory::DeregisterWorkspaceDirectoryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeregisterWorkspaceDirectoryFluentBuilder {
    /// Creates a new `DeregisterWorkspaceDirectory`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeregisterWorkspaceDirectory as a reference.
    pub fn as_input(&self) -> &crate::operation::deregister_workspace_directory::builders::DeregisterWorkspaceDirectoryInputBuilder {
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
        crate::operation::deregister_workspace_directory::DeregisterWorkspaceDirectoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deregister_workspace_directory::DeregisterWorkspaceDirectoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::deregister_workspace_directory::DeregisterWorkspaceDirectory::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::deregister_workspace_directory::DeregisterWorkspaceDirectory::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::deregister_workspace_directory::DeregisterWorkspaceDirectoryOutput,
        crate::operation::deregister_workspace_directory::DeregisterWorkspaceDirectoryError,
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
    /// <p>The identifier of the directory. If any WorkSpaces are registered to this directory, you must remove them before you deregister the directory, or you will receive an OperationNotSupportedException error.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The identifier of the directory. If any WorkSpaces are registered to this directory, you must remove them before you deregister the directory, or you will receive an OperationNotSupportedException error.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>The identifier of the directory. If any WorkSpaces are registered to this directory, you must remove them before you deregister the directory, or you will receive an OperationNotSupportedException error.</p>
    pub fn get_directory_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_id()
    }
}
