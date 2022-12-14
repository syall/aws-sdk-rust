// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DescribeAffectedAccountsForOrganization`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_affected_accounts_for_organization`](crate::client::Client::describe_affected_accounts_for_organization).
///
/// See [`crate::client::fluent_builders::DescribeAffectedAccountsForOrganization`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeAffectedAccountsForOrganization {
    _private: (),
}
impl DescribeAffectedAccountsForOrganization {
    /// Creates a new builder-style object to manufacture [`DescribeAffectedAccountsForOrganizationInput`](crate::input::DescribeAffectedAccountsForOrganizationInput).
    pub fn builder() -> crate::input::describe_affected_accounts_for_organization_input::Builder {
        crate::input::describe_affected_accounts_for_organization_input::Builder::default()
    }
    /// Creates a new `DescribeAffectedAccountsForOrganization` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAffectedAccountsForOrganization {
    type Output = std::result::Result<
        crate::output::DescribeAffectedAccountsForOrganizationOutput,
        crate::error::DescribeAffectedAccountsForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_affected_accounts_for_organization_error(
                response,
            )
        } else {
            crate::operation_deser::parse_describe_affected_accounts_for_organization_response(
                response,
            )
        }
    }
}

/// Operation shape for `DescribeAffectedEntities`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_affected_entities`](crate::client::Client::describe_affected_entities).
///
/// See [`crate::client::fluent_builders::DescribeAffectedEntities`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeAffectedEntities {
    _private: (),
}
impl DescribeAffectedEntities {
    /// Creates a new builder-style object to manufacture [`DescribeAffectedEntitiesInput`](crate::input::DescribeAffectedEntitiesInput).
    pub fn builder() -> crate::input::describe_affected_entities_input::Builder {
        crate::input::describe_affected_entities_input::Builder::default()
    }
    /// Creates a new `DescribeAffectedEntities` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAffectedEntities {
    type Output = std::result::Result<
        crate::output::DescribeAffectedEntitiesOutput,
        crate::error::DescribeAffectedEntitiesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_affected_entities_error(response)
        } else {
            crate::operation_deser::parse_describe_affected_entities_response(response)
        }
    }
}

/// Operation shape for `DescribeAffectedEntitiesForOrganization`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_affected_entities_for_organization`](crate::client::Client::describe_affected_entities_for_organization).
///
/// See [`crate::client::fluent_builders::DescribeAffectedEntitiesForOrganization`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeAffectedEntitiesForOrganization {
    _private: (),
}
impl DescribeAffectedEntitiesForOrganization {
    /// Creates a new builder-style object to manufacture [`DescribeAffectedEntitiesForOrganizationInput`](crate::input::DescribeAffectedEntitiesForOrganizationInput).
    pub fn builder() -> crate::input::describe_affected_entities_for_organization_input::Builder {
        crate::input::describe_affected_entities_for_organization_input::Builder::default()
    }
    /// Creates a new `DescribeAffectedEntitiesForOrganization` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAffectedEntitiesForOrganization {
    type Output = std::result::Result<
        crate::output::DescribeAffectedEntitiesForOrganizationOutput,
        crate::error::DescribeAffectedEntitiesForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_affected_entities_for_organization_error(
                response,
            )
        } else {
            crate::operation_deser::parse_describe_affected_entities_for_organization_response(
                response,
            )
        }
    }
}

/// Operation shape for `DescribeEntityAggregates`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_entity_aggregates`](crate::client::Client::describe_entity_aggregates).
///
/// See [`crate::client::fluent_builders::DescribeEntityAggregates`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEntityAggregates {
    _private: (),
}
impl DescribeEntityAggregates {
    /// Creates a new builder-style object to manufacture [`DescribeEntityAggregatesInput`](crate::input::DescribeEntityAggregatesInput).
    pub fn builder() -> crate::input::describe_entity_aggregates_input::Builder {
        crate::input::describe_entity_aggregates_input::Builder::default()
    }
    /// Creates a new `DescribeEntityAggregates` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEntityAggregates {
    type Output = std::result::Result<
        crate::output::DescribeEntityAggregatesOutput,
        crate::error::DescribeEntityAggregatesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_entity_aggregates_error(response)
        } else {
            crate::operation_deser::parse_describe_entity_aggregates_response(response)
        }
    }
}

/// Operation shape for `DescribeEventAggregates`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_event_aggregates`](crate::client::Client::describe_event_aggregates).
///
/// See [`crate::client::fluent_builders::DescribeEventAggregates`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEventAggregates {
    _private: (),
}
impl DescribeEventAggregates {
    /// Creates a new builder-style object to manufacture [`DescribeEventAggregatesInput`](crate::input::DescribeEventAggregatesInput).
    pub fn builder() -> crate::input::describe_event_aggregates_input::Builder {
        crate::input::describe_event_aggregates_input::Builder::default()
    }
    /// Creates a new `DescribeEventAggregates` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEventAggregates {
    type Output = std::result::Result<
        crate::output::DescribeEventAggregatesOutput,
        crate::error::DescribeEventAggregatesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_event_aggregates_error(response)
        } else {
            crate::operation_deser::parse_describe_event_aggregates_response(response)
        }
    }
}

/// Operation shape for `DescribeEventDetails`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_event_details`](crate::client::Client::describe_event_details).
///
/// See [`crate::client::fluent_builders::DescribeEventDetails`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEventDetails {
    _private: (),
}
impl DescribeEventDetails {
    /// Creates a new builder-style object to manufacture [`DescribeEventDetailsInput`](crate::input::DescribeEventDetailsInput).
    pub fn builder() -> crate::input::describe_event_details_input::Builder {
        crate::input::describe_event_details_input::Builder::default()
    }
    /// Creates a new `DescribeEventDetails` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEventDetails {
    type Output = std::result::Result<
        crate::output::DescribeEventDetailsOutput,
        crate::error::DescribeEventDetailsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_event_details_error(response)
        } else {
            crate::operation_deser::parse_describe_event_details_response(response)
        }
    }
}

/// Operation shape for `DescribeEventDetailsForOrganization`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_event_details_for_organization`](crate::client::Client::describe_event_details_for_organization).
///
/// See [`crate::client::fluent_builders::DescribeEventDetailsForOrganization`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEventDetailsForOrganization {
    _private: (),
}
impl DescribeEventDetailsForOrganization {
    /// Creates a new builder-style object to manufacture [`DescribeEventDetailsForOrganizationInput`](crate::input::DescribeEventDetailsForOrganizationInput).
    pub fn builder() -> crate::input::describe_event_details_for_organization_input::Builder {
        crate::input::describe_event_details_for_organization_input::Builder::default()
    }
    /// Creates a new `DescribeEventDetailsForOrganization` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEventDetailsForOrganization {
    type Output = std::result::Result<
        crate::output::DescribeEventDetailsForOrganizationOutput,
        crate::error::DescribeEventDetailsForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_event_details_for_organization_error(response)
        } else {
            crate::operation_deser::parse_describe_event_details_for_organization_response(response)
        }
    }
}

/// Operation shape for `DescribeEvents`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_events`](crate::client::Client::describe_events).
///
/// See [`crate::client::fluent_builders::DescribeEvents`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEvents {
    _private: (),
}
impl DescribeEvents {
    /// Creates a new builder-style object to manufacture [`DescribeEventsInput`](crate::input::DescribeEventsInput).
    pub fn builder() -> crate::input::describe_events_input::Builder {
        crate::input::describe_events_input::Builder::default()
    }
    /// Creates a new `DescribeEvents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEvents {
    type Output =
        std::result::Result<crate::output::DescribeEventsOutput, crate::error::DescribeEventsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_events_error(response)
        } else {
            crate::operation_deser::parse_describe_events_response(response)
        }
    }
}

/// Operation shape for `DescribeEventsForOrganization`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_events_for_organization`](crate::client::Client::describe_events_for_organization).
///
/// See [`crate::client::fluent_builders::DescribeEventsForOrganization`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEventsForOrganization {
    _private: (),
}
impl DescribeEventsForOrganization {
    /// Creates a new builder-style object to manufacture [`DescribeEventsForOrganizationInput`](crate::input::DescribeEventsForOrganizationInput).
    pub fn builder() -> crate::input::describe_events_for_organization_input::Builder {
        crate::input::describe_events_for_organization_input::Builder::default()
    }
    /// Creates a new `DescribeEventsForOrganization` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEventsForOrganization {
    type Output = std::result::Result<
        crate::output::DescribeEventsForOrganizationOutput,
        crate::error::DescribeEventsForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_events_for_organization_error(response)
        } else {
            crate::operation_deser::parse_describe_events_for_organization_response(response)
        }
    }
}

/// Operation shape for `DescribeEventTypes`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_event_types`](crate::client::Client::describe_event_types).
///
/// See [`crate::client::fluent_builders::DescribeEventTypes`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEventTypes {
    _private: (),
}
impl DescribeEventTypes {
    /// Creates a new builder-style object to manufacture [`DescribeEventTypesInput`](crate::input::DescribeEventTypesInput).
    pub fn builder() -> crate::input::describe_event_types_input::Builder {
        crate::input::describe_event_types_input::Builder::default()
    }
    /// Creates a new `DescribeEventTypes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEventTypes {
    type Output = std::result::Result<
        crate::output::DescribeEventTypesOutput,
        crate::error::DescribeEventTypesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_event_types_error(response)
        } else {
            crate::operation_deser::parse_describe_event_types_response(response)
        }
    }
}

/// Operation shape for `DescribeHealthServiceStatusForOrganization`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_health_service_status_for_organization`](crate::client::Client::describe_health_service_status_for_organization).
///
/// See [`crate::client::fluent_builders::DescribeHealthServiceStatusForOrganization`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeHealthServiceStatusForOrganization {
    _private: (),
}
impl DescribeHealthServiceStatusForOrganization {
    /// Creates a new builder-style object to manufacture [`DescribeHealthServiceStatusForOrganizationInput`](crate::input::DescribeHealthServiceStatusForOrganizationInput).
    pub fn builder() -> crate::input::describe_health_service_status_for_organization_input::Builder
    {
        crate::input::describe_health_service_status_for_organization_input::Builder::default()
    }
    /// Creates a new `DescribeHealthServiceStatusForOrganization` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeHealthServiceStatusForOrganization {
    type Output = std::result::Result<
        crate::output::DescribeHealthServiceStatusForOrganizationOutput,
        crate::error::DescribeHealthServiceStatusForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_health_service_status_for_organization_error(
                response,
            )
        } else {
            crate::operation_deser::parse_describe_health_service_status_for_organization_response(
                response,
            )
        }
    }
}

/// Operation shape for `DisableHealthServiceAccessForOrganization`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`disable_health_service_access_for_organization`](crate::client::Client::disable_health_service_access_for_organization).
///
/// See [`crate::client::fluent_builders::DisableHealthServiceAccessForOrganization`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisableHealthServiceAccessForOrganization {
    _private: (),
}
impl DisableHealthServiceAccessForOrganization {
    /// Creates a new builder-style object to manufacture [`DisableHealthServiceAccessForOrganizationInput`](crate::input::DisableHealthServiceAccessForOrganizationInput).
    pub fn builder() -> crate::input::disable_health_service_access_for_organization_input::Builder
    {
        crate::input::disable_health_service_access_for_organization_input::Builder::default()
    }
    /// Creates a new `DisableHealthServiceAccessForOrganization` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisableHealthServiceAccessForOrganization {
    type Output = std::result::Result<
        crate::output::DisableHealthServiceAccessForOrganizationOutput,
        crate::error::DisableHealthServiceAccessForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disable_health_service_access_for_organization_error(
                response,
            )
        } else {
            crate::operation_deser::parse_disable_health_service_access_for_organization_response(
                response,
            )
        }
    }
}

/// Operation shape for `EnableHealthServiceAccessForOrganization`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`enable_health_service_access_for_organization`](crate::client::Client::enable_health_service_access_for_organization).
///
/// See [`crate::client::fluent_builders::EnableHealthServiceAccessForOrganization`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct EnableHealthServiceAccessForOrganization {
    _private: (),
}
impl EnableHealthServiceAccessForOrganization {
    /// Creates a new builder-style object to manufacture [`EnableHealthServiceAccessForOrganizationInput`](crate::input::EnableHealthServiceAccessForOrganizationInput).
    pub fn builder() -> crate::input::enable_health_service_access_for_organization_input::Builder {
        crate::input::enable_health_service_access_for_organization_input::Builder::default()
    }
    /// Creates a new `EnableHealthServiceAccessForOrganization` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for EnableHealthServiceAccessForOrganization {
    type Output = std::result::Result<
        crate::output::EnableHealthServiceAccessForOrganizationOutput,
        crate::error::EnableHealthServiceAccessForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_enable_health_service_access_for_organization_error(
                response,
            )
        } else {
            crate::operation_deser::parse_enable_health_service_access_for_organization_response(
                response,
            )
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
