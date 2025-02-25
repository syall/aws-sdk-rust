// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisableSso`](crate::operation::disable_sso::builders::DisableSsoFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::operation::disable_sso::builders::DisableSsoFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::disable_sso::builders::DisableSsoFluentBuilder::set_directory_id):<br>required: **true**<br><p>The identifier of the directory for which to disable single-sign on.</p><br>
    ///   - [`user_name(impl Into<String>)`](crate::operation::disable_sso::builders::DisableSsoFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::disable_sso::builders::DisableSsoFluentBuilder::set_user_name):<br>required: **false**<br><p>The username of an alternate account to use to disable single-sign on. This is only used for AD Connector directories. This account must have privileges to remove a service principal name.</p>  <p>If the AD Connector service account does not have privileges to remove a service principal name, you can specify an alternate account with the <i>UserName</i> and <i>Password</i> parameters. These credentials are only used to disable single sign-on and are not stored by the service. The AD Connector service account is not changed.</p><br>
    ///   - [`password(impl Into<String>)`](crate::operation::disable_sso::builders::DisableSsoFluentBuilder::password) / [`set_password(Option<String>)`](crate::operation::disable_sso::builders::DisableSsoFluentBuilder::set_password):<br>required: **false**<br><p>The password of an alternate account to use to disable single-sign on. This is only used for AD Connector directories. For more information, see the <i>UserName</i> parameter.</p><br>
    /// - On success, responds with [`DisableSsoOutput`](crate::operation::disable_sso::DisableSsoOutput)
    /// - On failure, responds with [`SdkError<DisableSsoError>`](crate::operation::disable_sso::DisableSsoError)
    pub fn disable_sso(&self) -> crate::operation::disable_sso::builders::DisableSsoFluentBuilder {
        crate::operation::disable_sso::builders::DisableSsoFluentBuilder::new(self.handle.clone())
    }
}
