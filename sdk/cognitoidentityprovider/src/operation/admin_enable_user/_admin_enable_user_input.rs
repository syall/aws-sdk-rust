// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the request that enables the user as an administrator.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct AdminEnableUserInput {
    /// <p>The user pool ID for the user pool where you want to enable the user.</p>
    pub user_pool_id: ::std::option::Option<::std::string::String>,
    /// <p>The user name of the user you want to enable.</p>
    pub username: ::std::option::Option<::std::string::String>,
}
impl AdminEnableUserInput {
    /// <p>The user pool ID for the user pool where you want to enable the user.</p>
    pub fn user_pool_id(&self) -> ::std::option::Option<&str> {
        self.user_pool_id.as_deref()
    }
    /// <p>The user name of the user you want to enable.</p>
    pub fn username(&self) -> ::std::option::Option<&str> {
        self.username.as_deref()
    }
}
impl ::std::fmt::Debug for AdminEnableUserInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AdminEnableUserInput");
        formatter.field("user_pool_id", &self.user_pool_id);
        formatter.field("username", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl AdminEnableUserInput {
    /// Creates a new builder-style object to manufacture [`AdminEnableUserInput`](crate::operation::admin_enable_user::AdminEnableUserInput).
    pub fn builder() -> crate::operation::admin_enable_user::builders::AdminEnableUserInputBuilder {
        crate::operation::admin_enable_user::builders::AdminEnableUserInputBuilder::default()
    }
}

/// A builder for [`AdminEnableUserInput`](crate::operation::admin_enable_user::AdminEnableUserInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct AdminEnableUserInputBuilder {
    pub(crate) user_pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) username: ::std::option::Option<::std::string::String>,
}
impl AdminEnableUserInputBuilder {
    /// <p>The user pool ID for the user pool where you want to enable the user.</p>
    /// This field is required.
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user pool ID for the user pool where you want to enable the user.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_pool_id = input;
        self
    }
    /// <p>The user pool ID for the user pool where you want to enable the user.</p>
    pub fn get_user_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.user_pool_id
    }
    /// <p>The user name of the user you want to enable.</p>
    /// This field is required.
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.username = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user name of the user you want to enable.</p>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.username = input;
        self
    }
    /// <p>The user name of the user you want to enable.</p>
    pub fn get_username(&self) -> &::std::option::Option<::std::string::String> {
        &self.username
    }
    /// Consumes the builder and constructs a [`AdminEnableUserInput`](crate::operation::admin_enable_user::AdminEnableUserInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::admin_enable_user::AdminEnableUserInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::admin_enable_user::AdminEnableUserInput {
            user_pool_id: self.user_pool_id,
            username: self.username,
        })
    }
}
impl ::std::fmt::Debug for AdminEnableUserInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AdminEnableUserInputBuilder");
        formatter.field("user_pool_id", &self.user_pool_id);
        formatter.field("username", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
