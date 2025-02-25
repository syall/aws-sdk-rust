// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_game_session::_update_game_session_output::UpdateGameSessionOutputBuilder;

pub use crate::operation::update_game_session::_update_game_session_input::UpdateGameSessionInputBuilder;

impl UpdateGameSessionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_game_session::UpdateGameSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_game_session::UpdateGameSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_game_session();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateGameSession`.
///
/// <p>Updates the mutable properties of a game session. </p>
/// <p>To update a game session, specify the game session ID and the values you want to change. </p>
/// <p>If successful, the updated <code>GameSession</code> object is returned. </p>
/// <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/reference-awssdk.html#reference-awssdk-resources-fleets">All APIs by task</a> </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateGameSessionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_game_session::builders::UpdateGameSessionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_game_session::UpdateGameSessionOutput,
        crate::operation::update_game_session::UpdateGameSessionError,
    > for UpdateGameSessionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_game_session::UpdateGameSessionOutput,
            crate::operation::update_game_session::UpdateGameSessionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateGameSessionFluentBuilder {
    /// Creates a new `UpdateGameSession`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateGameSession as a reference.
    pub fn as_input(&self) -> &crate::operation::update_game_session::builders::UpdateGameSessionInputBuilder {
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
        crate::operation::update_game_session::UpdateGameSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_game_session::UpdateGameSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_game_session::UpdateGameSession::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_game_session::UpdateGameSession::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_game_session::UpdateGameSessionOutput,
        crate::operation::update_game_session::UpdateGameSessionError,
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
    /// <p>A unique identifier for the game session to update. </p>
    pub fn game_session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.game_session_id(input.into());
        self
    }
    /// <p>A unique identifier for the game session to update. </p>
    pub fn set_game_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_game_session_id(input);
        self
    }
    /// <p>A unique identifier for the game session to update. </p>
    pub fn get_game_session_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_game_session_id()
    }
    /// <p>The maximum number of players that can be connected simultaneously to the game session.</p>
    pub fn maximum_player_session_count(mut self, input: i32) -> Self {
        self.inner = self.inner.maximum_player_session_count(input);
        self
    }
    /// <p>The maximum number of players that can be connected simultaneously to the game session.</p>
    pub fn set_maximum_player_session_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_maximum_player_session_count(input);
        self
    }
    /// <p>The maximum number of players that can be connected simultaneously to the game session.</p>
    pub fn get_maximum_player_session_count(&self) -> &::std::option::Option<i32> {
        self.inner.get_maximum_player_session_count()
    }
    /// <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>A policy that determines whether the game session is accepting new players.</p>
    pub fn player_session_creation_policy(mut self, input: crate::types::PlayerSessionCreationPolicy) -> Self {
        self.inner = self.inner.player_session_creation_policy(input);
        self
    }
    /// <p>A policy that determines whether the game session is accepting new players.</p>
    pub fn set_player_session_creation_policy(mut self, input: ::std::option::Option<crate::types::PlayerSessionCreationPolicy>) -> Self {
        self.inner = self.inner.set_player_session_creation_policy(input);
        self
    }
    /// <p>A policy that determines whether the game session is accepting new players.</p>
    pub fn get_player_session_creation_policy(&self) -> &::std::option::Option<crate::types::PlayerSessionCreationPolicy> {
        self.inner.get_player_session_creation_policy()
    }
    /// <p>Game session protection policy to apply to this game session only.</p>
    /// <ul>
    /// <li> <p> <b>NoProtection</b> -- The game session can be terminated during a scale-down event.</p> </li>
    /// <li> <p> <b>FullProtection</b> -- If the game session is in an <code>ACTIVE</code> status, it cannot be terminated during a scale-down event.</p> </li>
    /// </ul>
    pub fn protection_policy(mut self, input: crate::types::ProtectionPolicy) -> Self {
        self.inner = self.inner.protection_policy(input);
        self
    }
    /// <p>Game session protection policy to apply to this game session only.</p>
    /// <ul>
    /// <li> <p> <b>NoProtection</b> -- The game session can be terminated during a scale-down event.</p> </li>
    /// <li> <p> <b>FullProtection</b> -- If the game session is in an <code>ACTIVE</code> status, it cannot be terminated during a scale-down event.</p> </li>
    /// </ul>
    pub fn set_protection_policy(mut self, input: ::std::option::Option<crate::types::ProtectionPolicy>) -> Self {
        self.inner = self.inner.set_protection_policy(input);
        self
    }
    /// <p>Game session protection policy to apply to this game session only.</p>
    /// <ul>
    /// <li> <p> <b>NoProtection</b> -- The game session can be terminated during a scale-down event.</p> </li>
    /// <li> <p> <b>FullProtection</b> -- If the game session is in an <code>ACTIVE</code> status, it cannot be terminated during a scale-down event.</p> </li>
    /// </ul>
    pub fn get_protection_policy(&self) -> &::std::option::Option<crate::types::ProtectionPolicy> {
        self.inner.get_protection_policy()
    }
}
