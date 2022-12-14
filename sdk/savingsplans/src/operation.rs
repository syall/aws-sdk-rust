// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateSavingsPlan`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_savings_plan`](crate::client::Client::create_savings_plan).
///
/// See [`crate::client::fluent_builders::CreateSavingsPlan`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateSavingsPlan {
    _private: (),
}
impl CreateSavingsPlan {
    /// Creates a new builder-style object to manufacture [`CreateSavingsPlanInput`](crate::input::CreateSavingsPlanInput).
    pub fn builder() -> crate::input::create_savings_plan_input::Builder {
        crate::input::create_savings_plan_input::Builder::default()
    }
    /// Creates a new `CreateSavingsPlan` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateSavingsPlan {
    type Output = std::result::Result<
        crate::output::CreateSavingsPlanOutput,
        crate::error::CreateSavingsPlanError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_savings_plan_error(response)
        } else {
            crate::operation_deser::parse_create_savings_plan_response(response)
        }
    }
}

/// Operation shape for `DeleteQueuedSavingsPlan`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_queued_savings_plan`](crate::client::Client::delete_queued_savings_plan).
///
/// See [`crate::client::fluent_builders::DeleteQueuedSavingsPlan`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteQueuedSavingsPlan {
    _private: (),
}
impl DeleteQueuedSavingsPlan {
    /// Creates a new builder-style object to manufacture [`DeleteQueuedSavingsPlanInput`](crate::input::DeleteQueuedSavingsPlanInput).
    pub fn builder() -> crate::input::delete_queued_savings_plan_input::Builder {
        crate::input::delete_queued_savings_plan_input::Builder::default()
    }
    /// Creates a new `DeleteQueuedSavingsPlan` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteQueuedSavingsPlan {
    type Output = std::result::Result<
        crate::output::DeleteQueuedSavingsPlanOutput,
        crate::error::DeleteQueuedSavingsPlanError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_queued_savings_plan_error(response)
        } else {
            crate::operation_deser::parse_delete_queued_savings_plan_response(response)
        }
    }
}

/// Operation shape for `DescribeSavingsPlanRates`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_savings_plan_rates`](crate::client::Client::describe_savings_plan_rates).
///
/// See [`crate::client::fluent_builders::DescribeSavingsPlanRates`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeSavingsPlanRates {
    _private: (),
}
impl DescribeSavingsPlanRates {
    /// Creates a new builder-style object to manufacture [`DescribeSavingsPlanRatesInput`](crate::input::DescribeSavingsPlanRatesInput).
    pub fn builder() -> crate::input::describe_savings_plan_rates_input::Builder {
        crate::input::describe_savings_plan_rates_input::Builder::default()
    }
    /// Creates a new `DescribeSavingsPlanRates` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeSavingsPlanRates {
    type Output = std::result::Result<
        crate::output::DescribeSavingsPlanRatesOutput,
        crate::error::DescribeSavingsPlanRatesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_savings_plan_rates_error(response)
        } else {
            crate::operation_deser::parse_describe_savings_plan_rates_response(response)
        }
    }
}

/// Operation shape for `DescribeSavingsPlans`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_savings_plans`](crate::client::Client::describe_savings_plans).
///
/// See [`crate::client::fluent_builders::DescribeSavingsPlans`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeSavingsPlans {
    _private: (),
}
impl DescribeSavingsPlans {
    /// Creates a new builder-style object to manufacture [`DescribeSavingsPlansInput`](crate::input::DescribeSavingsPlansInput).
    pub fn builder() -> crate::input::describe_savings_plans_input::Builder {
        crate::input::describe_savings_plans_input::Builder::default()
    }
    /// Creates a new `DescribeSavingsPlans` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeSavingsPlans {
    type Output = std::result::Result<
        crate::output::DescribeSavingsPlansOutput,
        crate::error::DescribeSavingsPlansError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_savings_plans_error(response)
        } else {
            crate::operation_deser::parse_describe_savings_plans_response(response)
        }
    }
}

/// Operation shape for `DescribeSavingsPlansOfferingRates`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_savings_plans_offering_rates`](crate::client::Client::describe_savings_plans_offering_rates).
///
/// See [`crate::client::fluent_builders::DescribeSavingsPlansOfferingRates`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeSavingsPlansOfferingRates {
    _private: (),
}
impl DescribeSavingsPlansOfferingRates {
    /// Creates a new builder-style object to manufacture [`DescribeSavingsPlansOfferingRatesInput`](crate::input::DescribeSavingsPlansOfferingRatesInput).
    pub fn builder() -> crate::input::describe_savings_plans_offering_rates_input::Builder {
        crate::input::describe_savings_plans_offering_rates_input::Builder::default()
    }
    /// Creates a new `DescribeSavingsPlansOfferingRates` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeSavingsPlansOfferingRates {
    type Output = std::result::Result<
        crate::output::DescribeSavingsPlansOfferingRatesOutput,
        crate::error::DescribeSavingsPlansOfferingRatesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_savings_plans_offering_rates_error(response)
        } else {
            crate::operation_deser::parse_describe_savings_plans_offering_rates_response(response)
        }
    }
}

/// Operation shape for `DescribeSavingsPlansOfferings`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_savings_plans_offerings`](crate::client::Client::describe_savings_plans_offerings).
///
/// See [`crate::client::fluent_builders::DescribeSavingsPlansOfferings`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeSavingsPlansOfferings {
    _private: (),
}
impl DescribeSavingsPlansOfferings {
    /// Creates a new builder-style object to manufacture [`DescribeSavingsPlansOfferingsInput`](crate::input::DescribeSavingsPlansOfferingsInput).
    pub fn builder() -> crate::input::describe_savings_plans_offerings_input::Builder {
        crate::input::describe_savings_plans_offerings_input::Builder::default()
    }
    /// Creates a new `DescribeSavingsPlansOfferings` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeSavingsPlansOfferings {
    type Output = std::result::Result<
        crate::output::DescribeSavingsPlansOfferingsOutput,
        crate::error::DescribeSavingsPlansOfferingsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_savings_plans_offerings_error(response)
        } else {
            crate::operation_deser::parse_describe_savings_plans_offerings_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
