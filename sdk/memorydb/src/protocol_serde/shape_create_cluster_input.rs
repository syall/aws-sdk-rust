// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_cluster_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_cluster::CreateClusterInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.cluster_name {
        object.key("ClusterName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.node_type {
        object.key("NodeType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.description {
        object.key("Description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.num_shards {
        object.key("NumShards").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.num_replicas_per_shard {
        object.key("NumReplicasPerShard").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.subnet_group_name {
        object.key("SubnetGroupName").string(var_7.as_str());
    }
    if let Some(var_8) = &input.security_group_ids {
        let mut array_9 = object.key("SecurityGroupIds").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.maintenance_window {
        object.key("MaintenanceWindow").string(var_11.as_str());
    }
    if let Some(var_12) = &input.port {
        object.key("Port").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    if let Some(var_13) = &input.sns_topic_arn {
        object.key("SnsTopicArn").string(var_13.as_str());
    }
    if let Some(var_14) = &input.tls_enabled {
        object.key("TLSEnabled").boolean(*var_14);
    }
    if let Some(var_15) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_15.as_str());
    }
    if let Some(var_16) = &input.snapshot_arns {
        let mut array_17 = object.key("SnapshotArns").start_array();
        for item_18 in var_16 {
            {
                array_17.value().string(item_18.as_str());
            }
        }
        array_17.finish();
    }
    if let Some(var_19) = &input.snapshot_name {
        object.key("SnapshotName").string(var_19.as_str());
    }
    if let Some(var_20) = &input.snapshot_retention_limit {
        object.key("SnapshotRetentionLimit").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_20).into()),
        );
    }
    if let Some(var_21) = &input.tags {
        let mut array_22 = object.key("Tags").start_array();
        for item_23 in var_21 {
            {
                #[allow(unused_mut)]
                let mut object_24 = array_22.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_24, item_23)?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    if let Some(var_25) = &input.snapshot_window {
        object.key("SnapshotWindow").string(var_25.as_str());
    }
    if let Some(var_26) = &input.acl_name {
        object.key("ACLName").string(var_26.as_str());
    }
    if let Some(var_27) = &input.engine_version {
        object.key("EngineVersion").string(var_27.as_str());
    }
    if let Some(var_28) = &input.auto_minor_version_upgrade {
        object.key("AutoMinorVersionUpgrade").boolean(*var_28);
    }
    if let Some(var_29) = &input.data_tiering {
        object.key("DataTiering").boolean(*var_29);
    }
    Ok(())
}
