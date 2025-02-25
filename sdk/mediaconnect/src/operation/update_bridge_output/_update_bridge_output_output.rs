// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateBridgeOutputOutput {
    /// The Amazon Resource Number (ARN) of the bridge.
    pub bridge_arn: ::std::option::Option<::std::string::String>,
    /// The output that you updated.
    pub output: ::std::option::Option<crate::types::BridgeOutput>,
    _request_id: Option<String>,
}
impl UpdateBridgeOutputOutput {
    /// The Amazon Resource Number (ARN) of the bridge.
    pub fn bridge_arn(&self) -> ::std::option::Option<&str> {
        self.bridge_arn.as_deref()
    }
    /// The output that you updated.
    pub fn output(&self) -> ::std::option::Option<&crate::types::BridgeOutput> {
        self.output.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for UpdateBridgeOutputOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateBridgeOutputOutput {
    /// Creates a new builder-style object to manufacture [`UpdateBridgeOutputOutput`](crate::operation::update_bridge_output::UpdateBridgeOutputOutput).
    pub fn builder() -> crate::operation::update_bridge_output::builders::UpdateBridgeOutputOutputBuilder {
        crate::operation::update_bridge_output::builders::UpdateBridgeOutputOutputBuilder::default()
    }
}

/// A builder for [`UpdateBridgeOutputOutput`](crate::operation::update_bridge_output::UpdateBridgeOutputOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateBridgeOutputOutputBuilder {
    pub(crate) bridge_arn: ::std::option::Option<::std::string::String>,
    pub(crate) output: ::std::option::Option<crate::types::BridgeOutput>,
    _request_id: Option<String>,
}
impl UpdateBridgeOutputOutputBuilder {
    /// The Amazon Resource Number (ARN) of the bridge.
    pub fn bridge_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bridge_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// The Amazon Resource Number (ARN) of the bridge.
    pub fn set_bridge_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bridge_arn = input;
        self
    }
    /// The Amazon Resource Number (ARN) of the bridge.
    pub fn get_bridge_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.bridge_arn
    }
    /// The output that you updated.
    pub fn output(mut self, input: crate::types::BridgeOutput) -> Self {
        self.output = ::std::option::Option::Some(input);
        self
    }
    /// The output that you updated.
    pub fn set_output(mut self, input: ::std::option::Option<crate::types::BridgeOutput>) -> Self {
        self.output = input;
        self
    }
    /// The output that you updated.
    pub fn get_output(&self) -> &::std::option::Option<crate::types::BridgeOutput> {
        &self.output
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateBridgeOutputOutput`](crate::operation::update_bridge_output::UpdateBridgeOutputOutput).
    pub fn build(self) -> crate::operation::update_bridge_output::UpdateBridgeOutputOutput {
        crate::operation::update_bridge_output::UpdateBridgeOutputOutput {
            bridge_arn: self.bridge_arn,
            output: self.output,
            _request_id: self._request_id,
        }
    }
}
