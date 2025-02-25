// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_queue_url_input_input(
    input: &crate::operation::get_queue_url::GetQueueUrlInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetQueueUrl", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("QueueName");
    if let Some(var_2) = &input.queue_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("QueueOwnerAWSAccountId");
    if let Some(var_4) = &input.queue_owner_aws_account_id {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
