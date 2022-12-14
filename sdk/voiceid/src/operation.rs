// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateDomain`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_domain`](crate::client::Client::create_domain).
///
/// See [`crate::client::fluent_builders::CreateDomain`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateDomain {
    _private: (),
}
impl CreateDomain {
    /// Creates a new builder-style object to manufacture [`CreateDomainInput`](crate::input::CreateDomainInput).
    pub fn builder() -> crate::input::create_domain_input::Builder {
        crate::input::create_domain_input::Builder::default()
    }
    /// Creates a new `CreateDomain` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateDomain {
    type Output =
        std::result::Result<crate::output::CreateDomainOutput, crate::error::CreateDomainError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_domain_error(response)
        } else {
            crate::operation_deser::parse_create_domain_response(response)
        }
    }
}

/// Operation shape for `DeleteDomain`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_domain`](crate::client::Client::delete_domain).
///
/// See [`crate::client::fluent_builders::DeleteDomain`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteDomain {
    _private: (),
}
impl DeleteDomain {
    /// Creates a new builder-style object to manufacture [`DeleteDomainInput`](crate::input::DeleteDomainInput).
    pub fn builder() -> crate::input::delete_domain_input::Builder {
        crate::input::delete_domain_input::Builder::default()
    }
    /// Creates a new `DeleteDomain` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteDomain {
    type Output =
        std::result::Result<crate::output::DeleteDomainOutput, crate::error::DeleteDomainError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_domain_error(response)
        } else {
            crate::operation_deser::parse_delete_domain_response(response)
        }
    }
}

/// Operation shape for `DeleteFraudster`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_fraudster`](crate::client::Client::delete_fraudster).
///
/// See [`crate::client::fluent_builders::DeleteFraudster`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteFraudster {
    _private: (),
}
impl DeleteFraudster {
    /// Creates a new builder-style object to manufacture [`DeleteFraudsterInput`](crate::input::DeleteFraudsterInput).
    pub fn builder() -> crate::input::delete_fraudster_input::Builder {
        crate::input::delete_fraudster_input::Builder::default()
    }
    /// Creates a new `DeleteFraudster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteFraudster {
    type Output = std::result::Result<
        crate::output::DeleteFraudsterOutput,
        crate::error::DeleteFraudsterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_fraudster_error(response)
        } else {
            crate::operation_deser::parse_delete_fraudster_response(response)
        }
    }
}

/// Operation shape for `DeleteSpeaker`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_speaker`](crate::client::Client::delete_speaker).
///
/// See [`crate::client::fluent_builders::DeleteSpeaker`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteSpeaker {
    _private: (),
}
impl DeleteSpeaker {
    /// Creates a new builder-style object to manufacture [`DeleteSpeakerInput`](crate::input::DeleteSpeakerInput).
    pub fn builder() -> crate::input::delete_speaker_input::Builder {
        crate::input::delete_speaker_input::Builder::default()
    }
    /// Creates a new `DeleteSpeaker` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSpeaker {
    type Output =
        std::result::Result<crate::output::DeleteSpeakerOutput, crate::error::DeleteSpeakerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_speaker_error(response)
        } else {
            crate::operation_deser::parse_delete_speaker_response(response)
        }
    }
}

/// Operation shape for `DescribeDomain`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_domain`](crate::client::Client::describe_domain).
///
/// See [`crate::client::fluent_builders::DescribeDomain`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeDomain {
    _private: (),
}
impl DescribeDomain {
    /// Creates a new builder-style object to manufacture [`DescribeDomainInput`](crate::input::DescribeDomainInput).
    pub fn builder() -> crate::input::describe_domain_input::Builder {
        crate::input::describe_domain_input::Builder::default()
    }
    /// Creates a new `DescribeDomain` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeDomain {
    type Output =
        std::result::Result<crate::output::DescribeDomainOutput, crate::error::DescribeDomainError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_domain_error(response)
        } else {
            crate::operation_deser::parse_describe_domain_response(response)
        }
    }
}

/// Operation shape for `DescribeFraudster`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_fraudster`](crate::client::Client::describe_fraudster).
///
/// See [`crate::client::fluent_builders::DescribeFraudster`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeFraudster {
    _private: (),
}
impl DescribeFraudster {
    /// Creates a new builder-style object to manufacture [`DescribeFraudsterInput`](crate::input::DescribeFraudsterInput).
    pub fn builder() -> crate::input::describe_fraudster_input::Builder {
        crate::input::describe_fraudster_input::Builder::default()
    }
    /// Creates a new `DescribeFraudster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeFraudster {
    type Output = std::result::Result<
        crate::output::DescribeFraudsterOutput,
        crate::error::DescribeFraudsterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_fraudster_error(response)
        } else {
            crate::operation_deser::parse_describe_fraudster_response(response)
        }
    }
}

/// Operation shape for `DescribeFraudsterRegistrationJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_fraudster_registration_job`](crate::client::Client::describe_fraudster_registration_job).
///
/// See [`crate::client::fluent_builders::DescribeFraudsterRegistrationJob`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeFraudsterRegistrationJob {
    _private: (),
}
impl DescribeFraudsterRegistrationJob {
    /// Creates a new builder-style object to manufacture [`DescribeFraudsterRegistrationJobInput`](crate::input::DescribeFraudsterRegistrationJobInput).
    pub fn builder() -> crate::input::describe_fraudster_registration_job_input::Builder {
        crate::input::describe_fraudster_registration_job_input::Builder::default()
    }
    /// Creates a new `DescribeFraudsterRegistrationJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeFraudsterRegistrationJob {
    type Output = std::result::Result<
        crate::output::DescribeFraudsterRegistrationJobOutput,
        crate::error::DescribeFraudsterRegistrationJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_fraudster_registration_job_error(response)
        } else {
            crate::operation_deser::parse_describe_fraudster_registration_job_response(response)
        }
    }
}

/// Operation shape for `DescribeSpeaker`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_speaker`](crate::client::Client::describe_speaker).
///
/// See [`crate::client::fluent_builders::DescribeSpeaker`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeSpeaker {
    _private: (),
}
impl DescribeSpeaker {
    /// Creates a new builder-style object to manufacture [`DescribeSpeakerInput`](crate::input::DescribeSpeakerInput).
    pub fn builder() -> crate::input::describe_speaker_input::Builder {
        crate::input::describe_speaker_input::Builder::default()
    }
    /// Creates a new `DescribeSpeaker` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeSpeaker {
    type Output = std::result::Result<
        crate::output::DescribeSpeakerOutput,
        crate::error::DescribeSpeakerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_speaker_error(response)
        } else {
            crate::operation_deser::parse_describe_speaker_response(response)
        }
    }
}

/// Operation shape for `DescribeSpeakerEnrollmentJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_speaker_enrollment_job`](crate::client::Client::describe_speaker_enrollment_job).
///
/// See [`crate::client::fluent_builders::DescribeSpeakerEnrollmentJob`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeSpeakerEnrollmentJob {
    _private: (),
}
impl DescribeSpeakerEnrollmentJob {
    /// Creates a new builder-style object to manufacture [`DescribeSpeakerEnrollmentJobInput`](crate::input::DescribeSpeakerEnrollmentJobInput).
    pub fn builder() -> crate::input::describe_speaker_enrollment_job_input::Builder {
        crate::input::describe_speaker_enrollment_job_input::Builder::default()
    }
    /// Creates a new `DescribeSpeakerEnrollmentJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeSpeakerEnrollmentJob {
    type Output = std::result::Result<
        crate::output::DescribeSpeakerEnrollmentJobOutput,
        crate::error::DescribeSpeakerEnrollmentJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_speaker_enrollment_job_error(response)
        } else {
            crate::operation_deser::parse_describe_speaker_enrollment_job_response(response)
        }
    }
}

/// Operation shape for `EvaluateSession`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`evaluate_session`](crate::client::Client::evaluate_session).
///
/// See [`crate::client::fluent_builders::EvaluateSession`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct EvaluateSession {
    _private: (),
}
impl EvaluateSession {
    /// Creates a new builder-style object to manufacture [`EvaluateSessionInput`](crate::input::EvaluateSessionInput).
    pub fn builder() -> crate::input::evaluate_session_input::Builder {
        crate::input::evaluate_session_input::Builder::default()
    }
    /// Creates a new `EvaluateSession` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for EvaluateSession {
    type Output = std::result::Result<
        crate::output::EvaluateSessionOutput,
        crate::error::EvaluateSessionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_evaluate_session_error(response)
        } else {
            crate::operation_deser::parse_evaluate_session_response(response)
        }
    }
}

/// Operation shape for `ListDomains`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_domains`](crate::client::Client::list_domains).
///
/// See [`crate::client::fluent_builders::ListDomains`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListDomains {
    _private: (),
}
impl ListDomains {
    /// Creates a new builder-style object to manufacture [`ListDomainsInput`](crate::input::ListDomainsInput).
    pub fn builder() -> crate::input::list_domains_input::Builder {
        crate::input::list_domains_input::Builder::default()
    }
    /// Creates a new `ListDomains` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDomains {
    type Output =
        std::result::Result<crate::output::ListDomainsOutput, crate::error::ListDomainsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_domains_error(response)
        } else {
            crate::operation_deser::parse_list_domains_response(response)
        }
    }
}

/// Operation shape for `ListFraudsterRegistrationJobs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_fraudster_registration_jobs`](crate::client::Client::list_fraudster_registration_jobs).
///
/// See [`crate::client::fluent_builders::ListFraudsterRegistrationJobs`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListFraudsterRegistrationJobs {
    _private: (),
}
impl ListFraudsterRegistrationJobs {
    /// Creates a new builder-style object to manufacture [`ListFraudsterRegistrationJobsInput`](crate::input::ListFraudsterRegistrationJobsInput).
    pub fn builder() -> crate::input::list_fraudster_registration_jobs_input::Builder {
        crate::input::list_fraudster_registration_jobs_input::Builder::default()
    }
    /// Creates a new `ListFraudsterRegistrationJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListFraudsterRegistrationJobs {
    type Output = std::result::Result<
        crate::output::ListFraudsterRegistrationJobsOutput,
        crate::error::ListFraudsterRegistrationJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_fraudster_registration_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_fraudster_registration_jobs_response(response)
        }
    }
}

/// Operation shape for `ListSpeakerEnrollmentJobs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_speaker_enrollment_jobs`](crate::client::Client::list_speaker_enrollment_jobs).
///
/// See [`crate::client::fluent_builders::ListSpeakerEnrollmentJobs`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListSpeakerEnrollmentJobs {
    _private: (),
}
impl ListSpeakerEnrollmentJobs {
    /// Creates a new builder-style object to manufacture [`ListSpeakerEnrollmentJobsInput`](crate::input::ListSpeakerEnrollmentJobsInput).
    pub fn builder() -> crate::input::list_speaker_enrollment_jobs_input::Builder {
        crate::input::list_speaker_enrollment_jobs_input::Builder::default()
    }
    /// Creates a new `ListSpeakerEnrollmentJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSpeakerEnrollmentJobs {
    type Output = std::result::Result<
        crate::output::ListSpeakerEnrollmentJobsOutput,
        crate::error::ListSpeakerEnrollmentJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_speaker_enrollment_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_speaker_enrollment_jobs_response(response)
        }
    }
}

/// Operation shape for `ListSpeakers`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_speakers`](crate::client::Client::list_speakers).
///
/// See [`crate::client::fluent_builders::ListSpeakers`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListSpeakers {
    _private: (),
}
impl ListSpeakers {
    /// Creates a new builder-style object to manufacture [`ListSpeakersInput`](crate::input::ListSpeakersInput).
    pub fn builder() -> crate::input::list_speakers_input::Builder {
        crate::input::list_speakers_input::Builder::default()
    }
    /// Creates a new `ListSpeakers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSpeakers {
    type Output =
        std::result::Result<crate::output::ListSpeakersOutput, crate::error::ListSpeakersError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_speakers_error(response)
        } else {
            crate::operation_deser::parse_list_speakers_response(response)
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

/// Operation shape for `OptOutSpeaker`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`opt_out_speaker`](crate::client::Client::opt_out_speaker).
///
/// See [`crate::client::fluent_builders::OptOutSpeaker`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct OptOutSpeaker {
    _private: (),
}
impl OptOutSpeaker {
    /// Creates a new builder-style object to manufacture [`OptOutSpeakerInput`](crate::input::OptOutSpeakerInput).
    pub fn builder() -> crate::input::opt_out_speaker_input::Builder {
        crate::input::opt_out_speaker_input::Builder::default()
    }
    /// Creates a new `OptOutSpeaker` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for OptOutSpeaker {
    type Output =
        std::result::Result<crate::output::OptOutSpeakerOutput, crate::error::OptOutSpeakerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_opt_out_speaker_error(response)
        } else {
            crate::operation_deser::parse_opt_out_speaker_response(response)
        }
    }
}

/// Operation shape for `StartFraudsterRegistrationJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_fraudster_registration_job`](crate::client::Client::start_fraudster_registration_job).
///
/// See [`crate::client::fluent_builders::StartFraudsterRegistrationJob`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartFraudsterRegistrationJob {
    _private: (),
}
impl StartFraudsterRegistrationJob {
    /// Creates a new builder-style object to manufacture [`StartFraudsterRegistrationJobInput`](crate::input::StartFraudsterRegistrationJobInput).
    pub fn builder() -> crate::input::start_fraudster_registration_job_input::Builder {
        crate::input::start_fraudster_registration_job_input::Builder::default()
    }
    /// Creates a new `StartFraudsterRegistrationJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartFraudsterRegistrationJob {
    type Output = std::result::Result<
        crate::output::StartFraudsterRegistrationJobOutput,
        crate::error::StartFraudsterRegistrationJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_fraudster_registration_job_error(response)
        } else {
            crate::operation_deser::parse_start_fraudster_registration_job_response(response)
        }
    }
}

/// Operation shape for `StartSpeakerEnrollmentJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_speaker_enrollment_job`](crate::client::Client::start_speaker_enrollment_job).
///
/// See [`crate::client::fluent_builders::StartSpeakerEnrollmentJob`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartSpeakerEnrollmentJob {
    _private: (),
}
impl StartSpeakerEnrollmentJob {
    /// Creates a new builder-style object to manufacture [`StartSpeakerEnrollmentJobInput`](crate::input::StartSpeakerEnrollmentJobInput).
    pub fn builder() -> crate::input::start_speaker_enrollment_job_input::Builder {
        crate::input::start_speaker_enrollment_job_input::Builder::default()
    }
    /// Creates a new `StartSpeakerEnrollmentJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartSpeakerEnrollmentJob {
    type Output = std::result::Result<
        crate::output::StartSpeakerEnrollmentJobOutput,
        crate::error::StartSpeakerEnrollmentJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_speaker_enrollment_job_error(response)
        } else {
            crate::operation_deser::parse_start_speaker_enrollment_job_response(response)
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

/// Operation shape for `UpdateDomain`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_domain`](crate::client::Client::update_domain).
///
/// See [`crate::client::fluent_builders::UpdateDomain`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateDomain {
    _private: (),
}
impl UpdateDomain {
    /// Creates a new builder-style object to manufacture [`UpdateDomainInput`](crate::input::UpdateDomainInput).
    pub fn builder() -> crate::input::update_domain_input::Builder {
        crate::input::update_domain_input::Builder::default()
    }
    /// Creates a new `UpdateDomain` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateDomain {
    type Output =
        std::result::Result<crate::output::UpdateDomainOutput, crate::error::UpdateDomainError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_domain_error(response)
        } else {
            crate::operation_deser::parse_update_domain_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
