// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_outbound_voice_contact_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_outbound_voice_contact::StartOutboundVoiceContactInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.answer_machine_detection_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("AnswerMachineDetectionConfig").start_object();
        crate::protocol_serde::shape_answer_machine_detection_config::ser_answer_machine_detection_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.attributes {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Attributes").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    if let Some(var_7) = &input.campaign_id {
        object.key("CampaignId").string(var_7.as_str());
    }
    if let Some(var_8) = &input.client_token {
        object.key("ClientToken").string(var_8.as_str());
    }
    if let Some(var_9) = &input.contact_flow_id {
        object.key("ContactFlowId").string(var_9.as_str());
    }
    if let Some(var_10) = &input.destination_phone_number {
        object.key("DestinationPhoneNumber").string(var_10.as_str());
    }
    if let Some(var_11) = &input.instance_id {
        object.key("InstanceId").string(var_11.as_str());
    }
    if let Some(var_12) = &input.queue_id {
        object.key("QueueId").string(var_12.as_str());
    }
    if let Some(var_13) = &input.source_phone_number {
        object.key("SourcePhoneNumber").string(var_13.as_str());
    }
    if let Some(var_14) = &input.traffic_type {
        object.key("TrafficType").string(var_14.as_str());
    }
    Ok(())
}
