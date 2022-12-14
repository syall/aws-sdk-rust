// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `NotificationSeverityLevel`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let notificationseveritylevel = unimplemented!();
/// match notificationseveritylevel {
///     NotificationSeverityLevel::All => { /* ... */ },
///     NotificationSeverityLevel::High => { /* ... */ },
///     NotificationSeverityLevel::None => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `notificationseveritylevel` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `NotificationSeverityLevel::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `NotificationSeverityLevel::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `NotificationSeverityLevel::NewFeature` is defined.
/// Specifically, when `notificationseveritylevel` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `NotificationSeverityLevel::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum NotificationSeverityLevel {
    #[allow(missing_docs)] // documentation missing in model
    All,
    #[allow(missing_docs)] // documentation missing in model
    High,
    #[allow(missing_docs)] // documentation missing in model
    None,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for NotificationSeverityLevel {
    fn from(s: &str) -> Self {
        match s {
            "all" => NotificationSeverityLevel::All,
            "high" => NotificationSeverityLevel::High,
            "none" => NotificationSeverityLevel::None,
            other => NotificationSeverityLevel::Unknown(crate::types::UnknownVariantValue(
                other.to_owned(),
            )),
        }
    }
}
impl std::str::FromStr for NotificationSeverityLevel {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(NotificationSeverityLevel::from(s))
    }
}
impl NotificationSeverityLevel {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            NotificationSeverityLevel::All => "all",
            NotificationSeverityLevel::High => "high",
            NotificationSeverityLevel::None => "none",
            NotificationSeverityLevel::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["all", "high", "none"]
    }
}
impl AsRef<str> for NotificationSeverityLevel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The configuration for a Slack workspace that you added to an Amazon Web Services account.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct SlackWorkspaceConfiguration {
    /// <p>The team ID in Slack. This ID uniquely identifies a Slack workspace.</p>
    #[doc(hidden)]
    pub team_id: std::option::Option<std::string::String>,
}
impl SlackWorkspaceConfiguration {
    /// <p>The team ID in Slack. This ID uniquely identifies a Slack workspace.</p>
    pub fn team_id(&self) -> std::option::Option<&str> {
        self.team_id.as_deref()
    }
}
/// See [`SlackWorkspaceConfiguration`](crate::model::SlackWorkspaceConfiguration).
pub mod slack_workspace_configuration {

    /// A builder for [`SlackWorkspaceConfiguration`](crate::model::SlackWorkspaceConfiguration).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) team_id: std::option::Option<std::string::String>,
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
        /// Consumes the builder and constructs a [`SlackWorkspaceConfiguration`](crate::model::SlackWorkspaceConfiguration).
        pub fn build(self) -> crate::model::SlackWorkspaceConfiguration {
            crate::model::SlackWorkspaceConfiguration {
                team_id: self.team_id,
            }
        }
    }
}
impl SlackWorkspaceConfiguration {
    /// Creates a new builder-style object to manufacture [`SlackWorkspaceConfiguration`](crate::model::SlackWorkspaceConfiguration).
    pub fn builder() -> crate::model::slack_workspace_configuration::Builder {
        crate::model::slack_workspace_configuration::Builder::default()
    }
}

/// <p>The configuration for a Slack channel that you added to an Amazon Web Services account.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct SlackChannelConfiguration {
    /// <p>The team ID in Slack. This ID uniquely identifies a Slack workspace.</p>
    #[doc(hidden)]
    pub team_id: std::option::Option<std::string::String>,
    /// <p>The channel ID in Slack. This ID identifies a channel within a Slack workspace.</p>
    #[doc(hidden)]
    pub channel_id: std::option::Option<std::string::String>,
    /// <p>The name of the Slack channel that you configured with the Amazon Web Services Support App.</p>
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
impl SlackChannelConfiguration {
    /// <p>The team ID in Slack. This ID uniquely identifies a Slack workspace.</p>
    pub fn team_id(&self) -> std::option::Option<&str> {
        self.team_id.as_deref()
    }
    /// <p>The channel ID in Slack. This ID identifies a channel within a Slack workspace.</p>
    pub fn channel_id(&self) -> std::option::Option<&str> {
        self.channel_id.as_deref()
    }
    /// <p>The name of the Slack channel that you configured with the Amazon Web Services Support App.</p>
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
/// See [`SlackChannelConfiguration`](crate::model::SlackChannelConfiguration).
pub mod slack_channel_configuration {

    /// A builder for [`SlackChannelConfiguration`](crate::model::SlackChannelConfiguration).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
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
        /// <p>The name of the Slack channel that you configured with the Amazon Web Services Support App.</p>
        pub fn channel_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.channel_name = Some(input.into());
            self
        }
        /// <p>The name of the Slack channel that you configured with the Amazon Web Services Support App.</p>
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
        /// Consumes the builder and constructs a [`SlackChannelConfiguration`](crate::model::SlackChannelConfiguration).
        pub fn build(self) -> crate::model::SlackChannelConfiguration {
            crate::model::SlackChannelConfiguration {
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
impl SlackChannelConfiguration {
    /// Creates a new builder-style object to manufacture [`SlackChannelConfiguration`](crate::model::SlackChannelConfiguration).
    pub fn builder() -> crate::model::slack_channel_configuration::Builder {
        crate::model::slack_channel_configuration::Builder::default()
    }
}
