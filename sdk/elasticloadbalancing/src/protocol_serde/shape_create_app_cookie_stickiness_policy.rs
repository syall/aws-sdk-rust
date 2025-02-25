// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_app_cookie_stickiness_policy_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyOutput,
    crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "LoadBalancerNotFound" => {
            crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyError::AccessPointNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessPointNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_point_not_found_exception::de_access_point_not_found_exception_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "DuplicatePolicyName" => {
            crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyError::DuplicatePolicyNameException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DuplicatePolicyNameExceptionBuilder::default();
                    output = crate::protocol_serde::shape_duplicate_policy_name_exception::de_duplicate_policy_name_exception_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidConfigurationRequest" => {
            crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyError::InvalidConfigurationRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidConfigurationRequestExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_invalid_configuration_request_exception::de_invalid_configuration_request_exception_xml_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TooManyPolicies" => crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyError::TooManyPoliciesException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TooManyPoliciesExceptionBuilder::default();
                output = crate::protocol_serde::shape_too_many_policies_exception::de_too_many_policies_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_app_cookie_stickiness_policy_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyOutput,
    crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_app_cookie_stickiness_policy::builders::CreateAppCookieStickinessPolicyOutputBuilder::default();
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
