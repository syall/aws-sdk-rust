// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_permissions::_list_permissions_output::ListPermissionsOutputBuilder;

pub use crate::operation::list_permissions::_list_permissions_input::ListPermissionsInputBuilder;

impl ListPermissionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_permissions::ListPermissionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_permissions::ListPermissionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_permissions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListPermissions`.
///
/// <p>Lists the users and groups who have the Grafana <code>Admin</code> and <code>Editor</code> roles in this workspace. If you use this operation without specifying <code>userId</code> or <code>groupId</code>, the operation returns the roles of all users and groups. If you specify a <code>userId</code> or a <code>groupId</code>, only the roles for that user or group are returned. If you do this, you can specify only one <code>userId</code> or one <code>groupId</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListPermissionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_permissions::builders::ListPermissionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_permissions::ListPermissionsOutput,
        crate::operation::list_permissions::ListPermissionsError,
    > for ListPermissionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_permissions::ListPermissionsOutput,
            crate::operation::list_permissions::ListPermissionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListPermissionsFluentBuilder {
    /// Creates a new `ListPermissions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListPermissions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_permissions::builders::ListPermissionsInputBuilder {
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
        crate::operation::list_permissions::ListPermissionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_permissions::ListPermissionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_permissions::ListPermissions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_permissions::ListPermissions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_permissions::ListPermissionsOutput,
        crate::operation::list_permissions::ListPermissionsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_permissions::paginator::ListPermissionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_permissions::paginator::ListPermissionsPaginator {
        crate::operation::list_permissions::paginator::ListPermissionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The maximum number of results to include in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to include in the response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to include in the response.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token to use when requesting the next set of results. You received this token from a previous <code>ListPermissions</code> operation.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to use when requesting the next set of results. You received this token from a previous <code>ListPermissions</code> operation.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token to use when requesting the next set of results. You received this token from a previous <code>ListPermissions</code> operation.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>(Optional) If you specify <code>SSO_USER</code>, then only the permissions of IAM Identity Center users are returned. If you specify <code>SSO_GROUP</code>, only the permissions of IAM Identity Center groups are returned.</p>
    pub fn user_type(mut self, input: crate::types::UserType) -> Self {
        self.inner = self.inner.user_type(input);
        self
    }
    /// <p>(Optional) If you specify <code>SSO_USER</code>, then only the permissions of IAM Identity Center users are returned. If you specify <code>SSO_GROUP</code>, only the permissions of IAM Identity Center groups are returned.</p>
    pub fn set_user_type(mut self, input: ::std::option::Option<crate::types::UserType>) -> Self {
        self.inner = self.inner.set_user_type(input);
        self
    }
    /// <p>(Optional) If you specify <code>SSO_USER</code>, then only the permissions of IAM Identity Center users are returned. If you specify <code>SSO_GROUP</code>, only the permissions of IAM Identity Center groups are returned.</p>
    pub fn get_user_type(&self) -> &::std::option::Option<crate::types::UserType> {
        self.inner.get_user_type()
    }
    /// <p>(Optional) Limits the results to only the user that matches this ID.</p>
    pub fn user_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_id(input.into());
        self
    }
    /// <p>(Optional) Limits the results to only the user that matches this ID.</p>
    pub fn set_user_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_id(input);
        self
    }
    /// <p>(Optional) Limits the results to only the user that matches this ID.</p>
    pub fn get_user_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_id()
    }
    /// <p>(Optional) Limits the results to only the group that matches this ID.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_id(input.into());
        self
    }
    /// <p>(Optional) Limits the results to only the group that matches this ID.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_group_id(input);
        self
    }
    /// <p>(Optional) Limits the results to only the group that matches this ID.</p>
    pub fn get_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_group_id()
    }
    /// <p>The ID of the workspace to list permissions for. This parameter is required.</p>
    pub fn workspace_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workspace_id(input.into());
        self
    }
    /// <p>The ID of the workspace to list permissions for. This parameter is required.</p>
    pub fn set_workspace_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workspace_id(input);
        self
    }
    /// <p>The ID of the workspace to list permissions for. This parameter is required.</p>
    pub fn get_workspace_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workspace_id()
    }
}
