// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_application`](crate::client::Client::create_application).
///
/// See [`crate::client::fluent_builders::CreateApplication`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateApplication {
    _private: (),
}
impl CreateApplication {
    /// Creates a new builder-style object to manufacture [`CreateApplicationInput`](crate::input::CreateApplicationInput).
    pub fn builder() -> crate::input::create_application_input::Builder {
        crate::input::create_application_input::Builder::default()
    }
    /// Creates a new `CreateApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateApplication {
    type Output = std::result::Result<
        crate::output::CreateApplicationOutput,
        crate::error::CreateApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_application_error(response)
        } else {
            crate::operation_deser::parse_create_application_response(response)
        }
    }
}

/// Operation shape for `DeleteApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_application`](crate::client::Client::delete_application).
///
/// See [`crate::client::fluent_builders::DeleteApplication`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteApplication {
    _private: (),
}
impl DeleteApplication {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationInput`](crate::input::DeleteApplicationInput).
    pub fn builder() -> crate::input::delete_application_input::Builder {
        crate::input::delete_application_input::Builder::default()
    }
    /// Creates a new `DeleteApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteApplication {
    type Output = std::result::Result<
        crate::output::DeleteApplicationOutput,
        crate::error::DeleteApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_application_error(response)
        } else {
            crate::operation_deser::parse_delete_application_response(response)
        }
    }
}

/// Operation shape for `DescribeApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_application`](crate::client::Client::describe_application).
///
/// See [`crate::client::fluent_builders::DescribeApplication`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeApplication {
    _private: (),
}
impl DescribeApplication {
    /// Creates a new builder-style object to manufacture [`DescribeApplicationInput`](crate::input::DescribeApplicationInput).
    pub fn builder() -> crate::input::describe_application_input::Builder {
        crate::input::describe_application_input::Builder::default()
    }
    /// Creates a new `DescribeApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeApplication {
    type Output = std::result::Result<
        crate::output::DescribeApplicationOutput,
        crate::error::DescribeApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_application_error(response)
        } else {
            crate::operation_deser::parse_describe_application_response(response)
        }
    }
}

/// Operation shape for `ListApplications`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_applications`](crate::client::Client::list_applications).
///
/// See [`crate::client::fluent_builders::ListApplications`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListApplications {
    _private: (),
}
impl ListApplications {
    /// Creates a new builder-style object to manufacture [`ListApplicationsInput`](crate::input::ListApplicationsInput).
    pub fn builder() -> crate::input::list_applications_input::Builder {
        crate::input::list_applications_input::Builder::default()
    }
    /// Creates a new `ListApplications` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListApplications {
    type Output = std::result::Result<
        crate::output::ListApplicationsOutput,
        crate::error::ListApplicationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_applications_error(response)
        } else {
            crate::operation_deser::parse_list_applications_response(response)
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

/// Operation shape for `UpdateApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_application`](crate::client::Client::update_application).
///
/// See [`crate::client::fluent_builders::UpdateApplication`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateApplication {
    _private: (),
}
impl UpdateApplication {
    /// Creates a new builder-style object to manufacture [`UpdateApplicationInput`](crate::input::UpdateApplicationInput).
    pub fn builder() -> crate::input::update_application_input::Builder {
        crate::input::update_application_input::Builder::default()
    }
    /// Creates a new `UpdateApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateApplication {
    type Output = std::result::Result<
        crate::output::UpdateApplicationOutput,
        crate::error::UpdateApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_update_application_error(response)
        } else {
            crate::operation_deser::parse_update_application_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
