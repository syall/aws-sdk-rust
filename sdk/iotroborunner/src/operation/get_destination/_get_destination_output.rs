// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetDestinationOutput {
    /// Destination ARN.
    pub arn: ::std::string::String,
    /// Filters access by the destination's identifier
    pub id: ::std::string::String,
    /// Human friendly name of the resource.
    pub name: ::std::string::String,
    /// Site ARN.
    pub site: ::std::string::String,
    /// Timestamp at which the resource was created.
    pub created_at: ::aws_smithy_types::DateTime,
    /// Timestamp at which the resource was last updated.
    pub updated_at: ::aws_smithy_types::DateTime,
    /// State of the destination.
    pub state: crate::types::DestinationState,
    /// JSON document containing additional fixed properties regarding the destination
    pub additional_fixed_properties: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetDestinationOutput {
    /// Destination ARN.
    pub fn arn(&self) -> &str {
        use std::ops::Deref;
        self.arn.deref()
    }
    /// Filters access by the destination's identifier
    pub fn id(&self) -> &str {
        use std::ops::Deref;
        self.id.deref()
    }
    /// Human friendly name of the resource.
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    /// Site ARN.
    pub fn site(&self) -> &str {
        use std::ops::Deref;
        self.site.deref()
    }
    /// Timestamp at which the resource was created.
    pub fn created_at(&self) -> &::aws_smithy_types::DateTime {
        &self.created_at
    }
    /// Timestamp at which the resource was last updated.
    pub fn updated_at(&self) -> &::aws_smithy_types::DateTime {
        &self.updated_at
    }
    /// State of the destination.
    pub fn state(&self) -> &crate::types::DestinationState {
        &self.state
    }
    /// JSON document containing additional fixed properties regarding the destination
    pub fn additional_fixed_properties(&self) -> ::std::option::Option<&str> {
        self.additional_fixed_properties.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetDestinationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetDestinationOutput {
    /// Creates a new builder-style object to manufacture [`GetDestinationOutput`](crate::operation::get_destination::GetDestinationOutput).
    pub fn builder() -> crate::operation::get_destination::builders::GetDestinationOutputBuilder {
        crate::operation::get_destination::builders::GetDestinationOutputBuilder::default()
    }
}

/// A builder for [`GetDestinationOutput`](crate::operation::get_destination::GetDestinationOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetDestinationOutputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) site: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) state: ::std::option::Option<crate::types::DestinationState>,
    pub(crate) additional_fixed_properties: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetDestinationOutputBuilder {
    /// Destination ARN.
    /// This field is required.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// Destination ARN.
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Destination ARN.
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// Filters access by the destination's identifier
    /// This field is required.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// Filters access by the destination's identifier
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Filters access by the destination's identifier
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// Human friendly name of the resource.
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// Human friendly name of the resource.
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Human friendly name of the resource.
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Site ARN.
    /// This field is required.
    pub fn site(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.site = ::std::option::Option::Some(input.into());
        self
    }
    /// Site ARN.
    pub fn set_site(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.site = input;
        self
    }
    /// Site ARN.
    pub fn get_site(&self) -> &::std::option::Option<::std::string::String> {
        &self.site
    }
    /// Timestamp at which the resource was created.
    /// This field is required.
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// Timestamp at which the resource was created.
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// Timestamp at which the resource was created.
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    /// Timestamp at which the resource was last updated.
    /// This field is required.
    pub fn updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated_at = ::std::option::Option::Some(input);
        self
    }
    /// Timestamp at which the resource was last updated.
    pub fn set_updated_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.updated_at = input;
        self
    }
    /// Timestamp at which the resource was last updated.
    pub fn get_updated_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.updated_at
    }
    /// State of the destination.
    /// This field is required.
    pub fn state(mut self, input: crate::types::DestinationState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// State of the destination.
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::DestinationState>) -> Self {
        self.state = input;
        self
    }
    /// State of the destination.
    pub fn get_state(&self) -> &::std::option::Option<crate::types::DestinationState> {
        &self.state
    }
    /// JSON document containing additional fixed properties regarding the destination
    pub fn additional_fixed_properties(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.additional_fixed_properties = ::std::option::Option::Some(input.into());
        self
    }
    /// JSON document containing additional fixed properties regarding the destination
    pub fn set_additional_fixed_properties(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.additional_fixed_properties = input;
        self
    }
    /// JSON document containing additional fixed properties regarding the destination
    pub fn get_additional_fixed_properties(&self) -> &::std::option::Option<::std::string::String> {
        &self.additional_fixed_properties
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetDestinationOutput`](crate::operation::get_destination::GetDestinationOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`arn`](crate::operation::get_destination::builders::GetDestinationOutputBuilder::arn)
    /// - [`id`](crate::operation::get_destination::builders::GetDestinationOutputBuilder::id)
    /// - [`name`](crate::operation::get_destination::builders::GetDestinationOutputBuilder::name)
    /// - [`site`](crate::operation::get_destination::builders::GetDestinationOutputBuilder::site)
    /// - [`created_at`](crate::operation::get_destination::builders::GetDestinationOutputBuilder::created_at)
    /// - [`updated_at`](crate::operation::get_destination::builders::GetDestinationOutputBuilder::updated_at)
    /// - [`state`](crate::operation::get_destination::builders::GetDestinationOutputBuilder::state)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_destination::GetDestinationOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_destination::GetDestinationOutput {
            arn: self.arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "arn",
                    "arn was not specified but it is required when building GetDestinationOutput",
                )
            })?,
            id: self.id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "id",
                    "id was not specified but it is required when building GetDestinationOutput",
                )
            })?,
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building GetDestinationOutput",
                )
            })?,
            site: self.site.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "site",
                    "site was not specified but it is required when building GetDestinationOutput",
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "created_at",
                    "created_at was not specified but it is required when building GetDestinationOutput",
                )
            })?,
            updated_at: self.updated_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "updated_at",
                    "updated_at was not specified but it is required when building GetDestinationOutput",
                )
            })?,
            state: self.state.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "state",
                    "state was not specified but it is required when building GetDestinationOutput",
                )
            })?,
            additional_fixed_properties: self.additional_fixed_properties,
            _request_id: self._request_id,
        })
    }
}
