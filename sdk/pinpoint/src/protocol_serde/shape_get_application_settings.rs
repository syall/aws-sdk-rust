// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_application_settings_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_application_settings::GetApplicationSettingsOutput,
    crate::operation::get_application_settings::GetApplicationSettingsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_application_settings::GetApplicationSettingsError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::get_application_settings::GetApplicationSettingsError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::operation::get_application_settings::GetApplicationSettingsError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BadRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_application_settings::GetApplicationSettingsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ForbiddenException" => crate::operation::get_application_settings::GetApplicationSettingsError::ForbiddenException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ForbiddenExceptionBuilder::default();
                output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_application_settings::GetApplicationSettingsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServerErrorException" => crate::operation::get_application_settings::GetApplicationSettingsError::InternalServerErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerErrorExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_internal_server_error_exception::de_internal_server_error_exception_json_err(_response_body, output)
                        .map_err(crate::operation::get_application_settings::GetApplicationSettingsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "MethodNotAllowedException" => crate::operation::get_application_settings::GetApplicationSettingsError::MethodNotAllowedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::MethodNotAllowedExceptionBuilder::default();
                output = crate::protocol_serde::shape_method_not_allowed_exception::de_method_not_allowed_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_application_settings::GetApplicationSettingsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NotFoundException" => crate::operation::get_application_settings::GetApplicationSettingsError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_application_settings::GetApplicationSettingsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "PayloadTooLargeException" => crate::operation::get_application_settings::GetApplicationSettingsError::PayloadTooLargeException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::PayloadTooLargeExceptionBuilder::default();
                output = crate::protocol_serde::shape_payload_too_large_exception::de_payload_too_large_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_application_settings::GetApplicationSettingsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::get_application_settings::GetApplicationSettingsError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_application_settings::GetApplicationSettingsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_application_settings::GetApplicationSettingsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_application_settings_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_application_settings::GetApplicationSettingsOutput,
    crate::operation::get_application_settings::GetApplicationSettingsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_application_settings::builders::GetApplicationSettingsOutputBuilder::default();
        output = output.set_application_settings_resource(
            crate::protocol_serde::shape_get_application_settings_output::de_application_settings_resource_payload(_response_body)?,
        );
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::get_application_settings_output_correct_errors(output).build()
    })
}
