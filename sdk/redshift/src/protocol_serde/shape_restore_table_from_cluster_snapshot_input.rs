// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_restore_table_from_cluster_snapshot_input_input(
    input: &crate::operation::restore_table_from_cluster_snapshot::RestoreTableFromClusterSnapshotInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "RestoreTableFromClusterSnapshot", "2012-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ClusterIdentifier");
    if let Some(var_2) = &input.cluster_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SnapshotIdentifier");
    if let Some(var_4) = &input.snapshot_identifier {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("SourceDatabaseName");
    if let Some(var_6) = &input.source_database_name {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("SourceSchemaName");
    if let Some(var_8) = &input.source_schema_name {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("SourceTableName");
    if let Some(var_10) = &input.source_table_name {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("TargetDatabaseName");
    if let Some(var_12) = &input.target_database_name {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("TargetSchemaName");
    if let Some(var_14) = &input.target_schema_name {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("NewTableName");
    if let Some(var_16) = &input.new_table_name {
        scope_15.string(var_16);
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("EnableCaseSensitiveIdentifier");
    if let Some(var_18) = &input.enable_case_sensitive_identifier {
        scope_17.boolean(*var_18);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
