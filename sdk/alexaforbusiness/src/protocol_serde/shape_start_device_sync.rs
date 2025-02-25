// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_start_device_sync_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::start_device_sync::StartDeviceSyncOutput, crate::operation::start_device_sync::StartDeviceSyncError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::start_device_sync::StartDeviceSyncError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::start_device_sync::StartDeviceSyncError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DeviceNotRegisteredException" => crate::operation::start_device_sync::StartDeviceSyncError::DeviceNotRegisteredException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DeviceNotRegisteredExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_device_not_registered_exception::de_device_not_registered_exception_json_err(_response_body, output)
                        .map_err(crate::operation::start_device_sync::StartDeviceSyncError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::start_device_sync::StartDeviceSyncError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_device_sync_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::start_device_sync::StartDeviceSyncOutput, crate::operation::start_device_sync::StartDeviceSyncError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::start_device_sync::builders::StartDeviceSyncOutputBuilder::default();
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_start_device_sync_input(
    input: &crate::operation::start_device_sync::StartDeviceSyncInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_start_device_sync_input::ser_start_device_sync_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
