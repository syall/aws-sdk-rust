// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_storage_configuration_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_storage_configuration::PutStorageConfigurationInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.disassociated_data_storage {
        object.key("disassociatedDataStorage").string(var_1.as_str());
    }
    if let Some(var_2) = &input.multi_layer_storage {
        #[allow(unused_mut)]
        let mut object_3 = object.key("multiLayerStorage").start_object();
        crate::protocol_serde::shape_multi_layer_storage::ser_multi_layer_storage(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.retention_period {
        #[allow(unused_mut)]
        let mut object_5 = object.key("retentionPeriod").start_object();
        crate::protocol_serde::shape_retention_period::ser_retention_period(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.storage_type {
        object.key("storageType").string(var_6.as_str());
    }
    Ok(())
}
