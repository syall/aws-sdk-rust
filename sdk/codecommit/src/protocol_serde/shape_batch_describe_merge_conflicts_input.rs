// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_describe_merge_conflicts_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::batch_describe_merge_conflicts::BatchDescribeMergeConflictsInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.repository_name {
        object.key("repositoryName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.destination_commit_specifier {
        object.key("destinationCommitSpecifier").string(var_2.as_str());
    }
    if let Some(var_3) = &input.source_commit_specifier {
        object.key("sourceCommitSpecifier").string(var_3.as_str());
    }
    if let Some(var_4) = &input.merge_option {
        object.key("mergeOption").string(var_4.as_str());
    }
    if let Some(var_5) = &input.max_merge_hunks {
        object.key("maxMergeHunks").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.max_conflict_files {
        object.key("maxConflictFiles").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.file_paths {
        let mut array_8 = object.key("filePaths").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.conflict_detail_level {
        object.key("conflictDetailLevel").string(var_10.as_str());
    }
    if let Some(var_11) = &input.conflict_resolution_strategy {
        object.key("conflictResolutionStrategy").string(var_11.as_str());
    }
    if let Some(var_12) = &input.next_token {
        object.key("nextToken").string(var_12.as_str());
    }
    Ok(())
}
