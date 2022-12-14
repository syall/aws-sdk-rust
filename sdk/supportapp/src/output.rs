// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateSlackChannelConfigurationOutput {
    /// <p>The team ID in Slack. This ID uniquely identifies a Slack workspace.</p>
    #[doc(hidden)]
    pub team_id: std::option::Option<std::string::String>,
    /// <p>The channel ID in Slack. This ID identifies a channel within a Slack workspace.</p>
    #[doc(hidden)]
    pub channel_id: std::option::Option<std::string::String>,
    /// <p>The name of the Slack channel that you configure for the Amazon Web Services Support App.</p>
    #[doc(hidden)]
    pub channel_name: std::option::Option<std::string::String>,
    /// <p>Whether you want to get notified when a support case is created or reopened.</p>
    #[doc(hidden)]
    pub notify_on_create_or_reopen_case: std::option::Option<bool>,
    /// <p>Whether you want to get notified when a support case has a new correspondence.</p>
    #[doc(hidden)]
    pub notify_on_add_correspondence_to_case: std::option::Option<bool>,
    /// <p>Whether you want to get notified when a support case is resolved.</p>
    #[doc(hidden)]
    pub notify_on_resolve_case: std::option::Option<bool>,
    /// <p>The case severity for a support case that you want to receive notifications.</p>
    #[doc(hidden)]
    pub notify_on_case_severity: std::option::Option<crate::model::NotificationSeverityLevel>,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that you want to use to perform operations on Amazon Web Services. For more information, see <a href="https://docs.aws.amazon.com/awssupport/latest/user/support-app-permissions.html">Managing access to the Amazon Web Services Support App</a> in the <i>Amazon Web Services Support User Guide</i>.</p>
    #[doc(hidden)]
    pub channel_role_arn: std::option::Option<std::string::String>,
}
impl UpdateSlackChannelConfigurationOutput {
    /// <p>The team ID in Slack. This ID uniquely identifies a Slack workspace.</p>
    pub fn team_id(&self) -> std::option::Option<&str> {
        self.team_id.as_deref()
    }
    /// <p>The channel ID in Slack. This ID identifies a channel within a Slack workspace.</p>
    pub fn channel_id(&self) -> std::option::Option<&str> {
        self.channel_id.as_deref()
    }
    /// <p>The name of the Slack channel that you configure for the Amazon Web Services Support App.</p>
    pub fn channel_name(&self) -> std::option::Option<&str> {
        self.channel_name.as_deref()
    }
    /// <p>Whether you want to get notified when a support case is created or reopened.</p>
    pub fn notify_on_create_or_reopen_case(&self) -> std::option::Option<bool> {
        self.notify_on_create_or_reopen_case
    }
    /// <p>Whether you want to get notified when a support case has a new correspondence.</p>
    pub fn notify_on_add_correspondence_to_case(&self) -> std::option::Option<bool> {
        self.notify_on_add_correspondence_to_case
    }
    /// <p>Whether you want to get notified when a support case is resolved.</p>
    pub fn notify_on_resolve_case(&self) -> std::option::Option<bool> {
        self.notify_on_resolve_case
    }
    /// <p>The case severity for a support case that you want to receive notifications.</p>
    pub fn notify_on_case_severity(
        &self,
    ) -> std::option::Option<&crate::model::NotificationSeverityLevel> {
        self.notify_on_case_severity.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role that you want to use to perform operations on Amazon Web Services. For more information, see <a href="https://docs.aws.amazon.com/awssupport/latest/user/support-app-permissions.html">Managing access to the Amazon Web Services Support App</a> in the <i>Amazon Web Services Support User Guide</i>.</p>
    pub fn channel_role_arn(&self) -> std::option::Option<&str> {
        self.channel_role_arn.as_deref()
    }
}
/// See [`UpdateSlackChannelConfigurationOutput`](crate::output::UpdateSlackChannelConfigurationOutput).
pub mod update_slack_channel_configuration_output {

    /// A builder for [`UpdateSlackChannelConfigurationOutput`](crate::output::UpdateSlackChannelConfigurationOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) team_id: std::option::Option<std::string::String>,
        pub(crate) channel_id: std::option::Option<std::string::String>,
        pub(crate) channel_name: std::option::Option<std::string::String>,
        pub(crate) notify_on_create_or_reopen_case: std::option::Option<bool>,
        pub(crate) notify_on_add_correspondence_to_case: std::option::Option<bool>,
        pub(crate) notify_on_resolve_case: std::option::Option<bool>,
        pub(crate) notify_on_case_severity:
            std::option::Option<crate::model::NotificationSeverityLevel>,
        pub(crate) channel_role_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The team ID in Slack. This ID uniquely identifies a Slack workspace.</p>
        pub fn team_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.team_id = Some(input.into());
            self
        }
        /// <p>The team ID in Slack. This ID uniquely identifies a Slack workspace.</p>
        pub fn set_team_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.team_id = input;
            self
        }
        /// <p>The channel ID in Slack. This ID identifies a channel within a Slack workspace.</p>
        pub fn channel_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.channel_id = Some(input.into());
            self
        }
        /// <p>The channel ID in Slack. This ID identifies a channel within a Slack workspace.</p>
        pub fn set_channel_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.channel_id = input;
            self
        }
        /// <p>The name of the Slack channel that you configure for the Amazon Web Services Support App.</p>
        pub fn channel_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.channel_name = Some(input.into());
            self
        }
        /// <p>The name of the Slack channel that you configure for the Amazon Web Services Support App.</p>
        pub fn set_channel_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.channel_name = input;
            self
        }
        /// <p>Whether you want to get notified when a support case is created or reopened.</p>
        pub fn notify_on_create_or_reopen_case(mut self, input: bool) -> Self {
            self.notify_on_create_or_reopen_case = Some(input);
            self
        }
        /// <p>Whether you want to get notified when a support case is created or reopened.</p>
        pub fn set_notify_on_create_or_reopen_case(
            mut self,
            input: std::option::Option<bool>,
        ) -> Self {
            self.notify_on_create_or_reopen_case = input;
            self
        }
        /// <p>Whether you want to get notified when a support case has a new correspondence.</p>
        pub fn notify_on_add_correspondence_to_case(mut self, input: bool) -> Self {
            self.notify_on_add_correspondence_to_case = Some(input);
            self
        }
        /// <p>Whether you want to get notified when a support case has a new correspondence.</p>
        pub fn set_notify_on_add_correspondence_to_case(
            mut self,
            input: std::option::Option<bool>,
        ) -> Self {
            self.notify_on_add_correspondence_to_case = input;
            self
        }
        /// <p>Whether you want to get notified when a support case is resolved.</p>
        pub fn notify_on_resolve_case(mut self, input: bool) -> Self {
            self.notify_on_resolve_case = Some(input);
            self
        }
        /// <p>Whether you want to get notified when a support case is resolved.</p>
        pub fn set_notify_on_resolve_case(mut self, input: std::option::Option<bool>) -> Self {
            self.notify_on_resolve_case = input;
            self
        }
        /// <p>The case severity for a support case that you want to receive notifications.</p>
        pub fn notify_on_case_severity(
            mut self,
            input: crate::model::NotificationSeverityLevel,
        ) -> Self {
            self.notify_on_case_severity = Some(input);
            self
        }
        /// <p>The case severity for a support case that you want to receive notifications.</p>
        pub fn set_notify_on_case_severity(
            mut self,
            input: std::option::Option<crate::model::NotificationSeverityLevel>,
        ) -> Self {
            self.notify_on_case_severity = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of an IAM role that you want to use to perform operations on Amazon Web Services. For more information, see <a href="https://docs.aws.amazon.com/awssupport/latest/user/support-app-permissions.html">Managing access to the Amazon Web Services Support App</a> in the <i>Amazon Web Services Support User Guide</i>.</p>
        pub fn channel_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.channel_role_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of an IAM role that you want to use to perform operations on Amazon Web Services. For more information, see <a href="https://docs.aws.amazon.com/awssupport/latest/user/support-app-permissions.html">Managing access to the Amazon Web Services Support App</a> in the <i>Amazon Web Services Support User Guide</i>.</p>
        pub fn set_channel_role_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.channel_role_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`UpdateSlackChannelConfigurationOutput`](crate::output::UpdateSlackChannelConfigurationOutput).
        pub fn build(self) -> crate::output::UpdateSlackChannelConfigurationOutput {
            crate::output::UpdateSlackChannelConfigurationOutput {
                team_id: self.team_id,
                channel_id: self.channel_id,
                channel_name: self.channel_name,
                notify_on_create_or_reopen_case: self.notify_on_create_or_reopen_case,
                notify_on_add_correspondence_to_case: self.notify_on_add_correspondence_to_case,
                notify_on_resolve_case: self.notify_on_resolve_case,
                notify_on_case_severity: self.notify_on_case_severity,
                channel_role_arn: self.channel_role_arn,
            }
        }
    }
}
impl UpdateSlackChannelConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`UpdateSlackChannelConfigurationOutput`](crate::output::UpdateSlackChannelConfigurationOutput).
    pub fn builder() -> crate::output::update_slack_channel_configuration_output::Builder {
        crate::output::update_slack_channel_configuration_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PutAccountAliasOutput {}
/// See [`PutAccountAliasOutput`](crate::output::PutAccountAliasOutput).
pub mod put_account_alias_output {

    /// A builder for [`PutAccountAliasOutput`](crate::output::PutAccountAliasOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`PutAccountAliasOutput`](crate::output::PutAccountAliasOutput).
        pub fn build(self) -> crate::output::PutAccountAliasOutput {
            crate::output::PutAccountAliasOutput {}
        }
    }
}
impl PutAccountAliasOutput {
    /// Creates a new builder-style object to manufacture [`PutAccountAliasOutput`](crate::output::PutAccountAliasOutput).
    pub fn builder() -> crate::output::put_account_alias_output::Builder {
        crate::output::put_account_alias_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListSlackWorkspaceConfigurationsOutput {
    /// <p>The point where pagination should resume when the response returns only partial results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The configurations for a Slack workspace.</p>
    #[doc(hidden)]
    pub slack_workspace_configurations:
        std::option::Option<std::vec::Vec<crate::model::SlackWorkspaceConfiguration>>,
}
impl ListSlackWorkspaceConfigurationsOutput {
    /// <p>The point where pagination should resume when the response returns only partial results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The configurations for a Slack workspace.</p>
    pub fn slack_workspace_configurations(
        &self,
    ) -> std::option::Option<&[crate::model::SlackWorkspaceConfiguration]> {
        self.slack_workspace_configurations.as_deref()
    }
}
/// See [`ListSlackWorkspaceConfigurationsOutput`](crate::output::ListSlackWorkspaceConfigurationsOutput).
pub mod list_slack_workspace_configurations_output {

    /// A builder for [`ListSlackWorkspaceConfigurationsOutput`](crate::output::ListSlackWorkspaceConfigurationsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) slack_workspace_configurations:
            std::option::Option<std::vec::Vec<crate::model::SlackWorkspaceConfiguration>>,
    }
    impl Builder {
        /// <p>The point where pagination should resume when the response returns only partial results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The point where pagination should resume when the response returns only partial results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Appends an item to `slack_workspace_configurations`.
        ///
        /// To override the contents of this collection use [`set_slack_workspace_configurations`](Self::set_slack_workspace_configurations).
        ///
        /// <p>The configurations for a Slack workspace.</p>
        pub fn slack_workspace_configurations(
            mut self,
            input: crate::model::SlackWorkspaceConfiguration,
        ) -> Self {
            let mut v = self.slack_workspace_configurations.unwrap_or_default();
            v.push(input);
            self.slack_workspace_configurations = Some(v);
            self
        }
        /// <p>The configurations for a Slack workspace.</p>
        pub fn set_slack_workspace_configurations(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::SlackWorkspaceConfiguration>>,
        ) -> Self {
            self.slack_workspace_configurations = input;
            self
        }
        /// Consumes the builder and constructs a [`ListSlackWorkspaceConfigurationsOutput`](crate::output::ListSlackWorkspaceConfigurationsOutput).
        pub fn build(self) -> crate::output::ListSlackWorkspaceConfigurationsOutput {
            crate::output::ListSlackWorkspaceConfigurationsOutput {
                next_token: self.next_token,
                slack_workspace_configurations: self.slack_workspace_configurations,
            }
        }
    }
}
impl ListSlackWorkspaceConfigurationsOutput {
    /// Creates a new builder-style object to manufacture [`ListSlackWorkspaceConfigurationsOutput`](crate::output::ListSlackWorkspaceConfigurationsOutput).
    pub fn builder() -> crate::output::list_slack_workspace_configurations_output::Builder {
        crate::output::list_slack_workspace_configurations_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListSlackChannelConfigurationsOutput {
    /// <p>The point where pagination should resume when the response returns only partial results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The configurations for a Slack channel.</p>
    #[doc(hidden)]
    pub slack_channel_configurations:
        std::option::Option<std::vec::Vec<crate::model::SlackChannelConfiguration>>,
}
impl ListSlackChannelConfigurationsOutput {
    /// <p>The point where pagination should resume when the response returns only partial results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The configurations for a Slack channel.</p>
    pub fn slack_channel_configurations(
        &self,
    ) -> std::option::Option<&[crate::model::SlackChannelConfiguration]> {
        self.slack_channel_configurations.as_deref()
    }
}
/// See [`ListSlackChannelConfigurationsOutput`](crate::output::ListSlackChannelConfigurationsOutput).
pub mod list_slack_channel_configurations_output {

    /// A builder for [`ListSlackChannelConfigurationsOutput`](crate::output::ListSlackChannelConfigurationsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) slack_channel_configurations:
            std::option::Option<std::vec::Vec<crate::model::SlackChannelConfiguration>>,
    }
    impl Builder {
        /// <p>The point where pagination should resume when the response returns only partial results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The point where pagination should resume when the response returns only partial results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Appends an item to `slack_channel_configurations`.
        ///
        /// To override the contents of this collection use [`set_slack_channel_configurations`](Self::set_slack_channel_configurations).
        ///
        /// <p>The configurations for a Slack channel.</p>
        pub fn slack_channel_configurations(
            mut self,
            input: crate::model::SlackChannelConfiguration,
        ) -> Self {
            let mut v = self.slack_channel_configurations.unwrap_or_default();
            v.push(input);
            self.slack_channel_configurations = Some(v);
            self
        }
        /// <p>The configurations for a Slack channel.</p>
        pub fn set_slack_channel_configurations(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::SlackChannelConfiguration>>,
        ) -> Self {
            self.slack_channel_configurations = input;
            self
        }
        /// Consumes the builder and constructs a [`ListSlackChannelConfigurationsOutput`](crate::output::ListSlackChannelConfigurationsOutput).
        pub fn build(self) -> crate::output::ListSlackChannelConfigurationsOutput {
            crate::output::ListSlackChannelConfigurationsOutput {
                next_token: self.next_token,
                slack_channel_configurations: self.slack_channel_configurations,
            }
        }
    }
}
impl ListSlackChannelConfigurationsOutput {
    /// Creates a new builder-style object to manufacture [`ListSlackChannelConfigurationsOutput`](crate::output::ListSlackChannelConfigurationsOutput).
    pub fn builder() -> crate::output::list_slack_channel_configurations_output::Builder {
        crate::output::list_slack_channel_configurations_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetAccountAliasOutput {
    /// <p>An alias or short name for an Amazon Web Services account.</p>
    #[doc(hidden)]
    pub account_alias: std::option::Option<std::string::String>,
}
impl GetAccountAliasOutput {
    /// <p>An alias or short name for an Amazon Web Services account.</p>
    pub fn account_alias(&self) -> std::option::Option<&str> {
        self.account_alias.as_deref()
    }
}
/// See [`GetAccountAliasOutput`](crate::output::GetAccountAliasOutput).
pub mod get_account_alias_output {

    /// A builder for [`GetAccountAliasOutput`](crate::output::GetAccountAliasOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) account_alias: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>An alias or short name for an Amazon Web Services account.</p>
        pub fn account_alias(mut self, input: impl Into<std::string::String>) -> Self {
            self.account_alias = Some(input.into());
            self
        }
        /// <p>An alias or short name for an Amazon Web Services account.</p>
        pub fn set_account_alias(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.account_alias = input;
            self
        }
        /// Consumes the builder and constructs a [`GetAccountAliasOutput`](crate::output::GetAccountAliasOutput).
        pub fn build(self) -> crate::output::GetAccountAliasOutput {
            crate::output::GetAccountAliasOutput {
                account_alias: self.account_alias,
            }
        }
    }
}
impl GetAccountAliasOutput {
    /// Creates a new builder-style object to manufacture [`GetAccountAliasOutput`](crate::output::GetAccountAliasOutput).
    pub fn builder() -> crate::output::get_account_alias_output::Builder {
        crate::output::get_account_alias_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteSlackWorkspaceConfigurationOutput {}
/// See [`DeleteSlackWorkspaceConfigurationOutput`](crate::output::DeleteSlackWorkspaceConfigurationOutput).
pub mod delete_slack_workspace_configuration_output {

    /// A builder for [`DeleteSlackWorkspaceConfigurationOutput`](crate::output::DeleteSlackWorkspaceConfigurationOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteSlackWorkspaceConfigurationOutput`](crate::output::DeleteSlackWorkspaceConfigurationOutput).
        pub fn build(self) -> crate::output::DeleteSlackWorkspaceConfigurationOutput {
            crate::output::DeleteSlackWorkspaceConfigurationOutput {}
        }
    }
}
impl DeleteSlackWorkspaceConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`DeleteSlackWorkspaceConfigurationOutput`](crate::output::DeleteSlackWorkspaceConfigurationOutput).
    pub fn builder() -> crate::output::delete_slack_workspace_configuration_output::Builder {
        crate::output::delete_slack_workspace_configuration_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteSlackChannelConfigurationOutput {}
/// See [`DeleteSlackChannelConfigurationOutput`](crate::output::DeleteSlackChannelConfigurationOutput).
pub mod delete_slack_channel_configuration_output {

    /// A builder for [`DeleteSlackChannelConfigurationOutput`](crate::output::DeleteSlackChannelConfigurationOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteSlackChannelConfigurationOutput`](crate::output::DeleteSlackChannelConfigurationOutput).
        pub fn build(self) -> crate::output::DeleteSlackChannelConfigurationOutput {
            crate::output::DeleteSlackChannelConfigurationOutput {}
        }
    }
}
impl DeleteSlackChannelConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`DeleteSlackChannelConfigurationOutput`](crate::output::DeleteSlackChannelConfigurationOutput).
    pub fn builder() -> crate::output::delete_slack_channel_configuration_output::Builder {
        crate::output::delete_slack_channel_configuration_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteAccountAliasOutput {}
/// See [`DeleteAccountAliasOutput`](crate::output::DeleteAccountAliasOutput).
pub mod delete_account_alias_output {

    /// A builder for [`DeleteAccountAliasOutput`](crate::output::DeleteAccountAliasOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteAccountAliasOutput`](crate::output::DeleteAccountAliasOutput).
        pub fn build(self) -> crate::output::DeleteAccountAliasOutput {
            crate::output::DeleteAccountAliasOutput {}
        }
    }
}
impl DeleteAccountAliasOutput {
    /// Creates a new builder-style object to manufacture [`DeleteAccountAliasOutput`](crate::output::DeleteAccountAliasOutput).
    pub fn builder() -> crate::output::delete_account_alias_output::Builder {
        crate::output::delete_account_alias_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateSlackChannelConfigurationOutput {}
/// See [`CreateSlackChannelConfigurationOutput`](crate::output::CreateSlackChannelConfigurationOutput).
pub mod create_slack_channel_configuration_output {

    /// A builder for [`CreateSlackChannelConfigurationOutput`](crate::output::CreateSlackChannelConfigurationOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`CreateSlackChannelConfigurationOutput`](crate::output::CreateSlackChannelConfigurationOutput).
        pub fn build(self) -> crate::output::CreateSlackChannelConfigurationOutput {
            crate::output::CreateSlackChannelConfigurationOutput {}
        }
    }
}
impl CreateSlackChannelConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`CreateSlackChannelConfigurationOutput`](crate::output::CreateSlackChannelConfigurationOutput).
    pub fn builder() -> crate::output::create_slack_channel_configuration_output::Builder {
        crate::output::create_slack_channel_configuration_output::Builder::default()
    }
}
