// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_warm_pool_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::delete_warm_pool::DeleteWarmPoolOutput, crate::operation::delete_warm_pool::DeleteWarmPoolError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_warm_pool::DeleteWarmPoolError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_warm_pool::DeleteWarmPoolError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "LimitExceeded" => crate::operation::delete_warm_pool::DeleteWarmPoolError::LimitExceededFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededFaultBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_fault::de_limit_exceeded_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::delete_warm_pool::DeleteWarmPoolError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceContention" => crate::operation::delete_warm_pool::DeleteWarmPoolError::ResourceContentionFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceContentionFaultBuilder::default();
                output = crate::protocol_serde::shape_resource_contention_fault::de_resource_contention_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::delete_warm_pool::DeleteWarmPoolError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceInUse" => crate::operation::delete_warm_pool::DeleteWarmPoolError::ResourceInUseFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceInUseFaultBuilder::default();
                output = crate::protocol_serde::shape_resource_in_use_fault::de_resource_in_use_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::delete_warm_pool::DeleteWarmPoolError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ScalingActivityInProgress" => crate::operation::delete_warm_pool::DeleteWarmPoolError::ScalingActivityInProgressFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ScalingActivityInProgressFaultBuilder::default();
                output = crate::protocol_serde::shape_scaling_activity_in_progress_fault::de_scaling_activity_in_progress_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_warm_pool::DeleteWarmPoolError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_warm_pool::DeleteWarmPoolError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_warm_pool_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::delete_warm_pool::DeleteWarmPoolOutput, crate::operation::delete_warm_pool::DeleteWarmPoolError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_warm_pool::builders::DeleteWarmPoolOutputBuilder::default();
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
