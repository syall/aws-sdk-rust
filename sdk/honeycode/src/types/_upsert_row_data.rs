// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Data needed to upsert rows in a table as part of a single item in the BatchUpsertTableRows request. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpsertRowData {
    /// <p> An external identifier that represents a single item in the request that is being upserted as part of the BatchUpsertTableRows request. This can be any string that you can use to identify the item in the request. The BatchUpsertTableRows API puts the batch item id in the results to allow you to link data in the request to data in the results. </p>
    pub batch_item_id: ::std::string::String,
    /// <p> The filter formula to use to find existing matching rows to update. The formula needs to return zero or more rows. If the formula returns 0 rows, then a new row will be appended in the target table. If the formula returns one or more rows, then the returned rows will be updated. </p>
    /// <p> Note that the filter formula needs to return rows from the target table for the upsert operation to succeed. If the filter formula has a syntax error or it doesn't evaluate to zero or more rows in the target table for any one item in the input list, then the entire BatchUpsertTableRows request fails and no updates are made to the table. </p>
    pub filter: ::std::option::Option<crate::types::Filter>,
    /// <p> A map representing the cells to update for the matching rows or an appended row. The key is the column id of the cell and the value is the CellInput object that represents the data to set in that cell. </p>
    pub cells_to_update: ::std::collections::HashMap<::std::string::String, crate::types::CellInput>,
}
impl UpsertRowData {
    /// <p> An external identifier that represents a single item in the request that is being upserted as part of the BatchUpsertTableRows request. This can be any string that you can use to identify the item in the request. The BatchUpsertTableRows API puts the batch item id in the results to allow you to link data in the request to data in the results. </p>
    pub fn batch_item_id(&self) -> &str {
        use std::ops::Deref;
        self.batch_item_id.deref()
    }
    /// <p> The filter formula to use to find existing matching rows to update. The formula needs to return zero or more rows. If the formula returns 0 rows, then a new row will be appended in the target table. If the formula returns one or more rows, then the returned rows will be updated. </p>
    /// <p> Note that the filter formula needs to return rows from the target table for the upsert operation to succeed. If the filter formula has a syntax error or it doesn't evaluate to zero or more rows in the target table for any one item in the input list, then the entire BatchUpsertTableRows request fails and no updates are made to the table. </p>
    pub fn filter(&self) -> ::std::option::Option<&crate::types::Filter> {
        self.filter.as_ref()
    }
    /// <p> A map representing the cells to update for the matching rows or an appended row. The key is the column id of the cell and the value is the CellInput object that represents the data to set in that cell. </p>
    pub fn cells_to_update(&self) -> &::std::collections::HashMap<::std::string::String, crate::types::CellInput> {
        &self.cells_to_update
    }
}
impl UpsertRowData {
    /// Creates a new builder-style object to manufacture [`UpsertRowData`](crate::types::UpsertRowData).
    pub fn builder() -> crate::types::builders::UpsertRowDataBuilder {
        crate::types::builders::UpsertRowDataBuilder::default()
    }
}

/// A builder for [`UpsertRowData`](crate::types::UpsertRowData).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpsertRowDataBuilder {
    pub(crate) batch_item_id: ::std::option::Option<::std::string::String>,
    pub(crate) filter: ::std::option::Option<crate::types::Filter>,
    pub(crate) cells_to_update: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::CellInput>>,
}
impl UpsertRowDataBuilder {
    /// <p> An external identifier that represents a single item in the request that is being upserted as part of the BatchUpsertTableRows request. This can be any string that you can use to identify the item in the request. The BatchUpsertTableRows API puts the batch item id in the results to allow you to link data in the request to data in the results. </p>
    /// This field is required.
    pub fn batch_item_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.batch_item_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> An external identifier that represents a single item in the request that is being upserted as part of the BatchUpsertTableRows request. This can be any string that you can use to identify the item in the request. The BatchUpsertTableRows API puts the batch item id in the results to allow you to link data in the request to data in the results. </p>
    pub fn set_batch_item_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.batch_item_id = input;
        self
    }
    /// <p> An external identifier that represents a single item in the request that is being upserted as part of the BatchUpsertTableRows request. This can be any string that you can use to identify the item in the request. The BatchUpsertTableRows API puts the batch item id in the results to allow you to link data in the request to data in the results. </p>
    pub fn get_batch_item_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.batch_item_id
    }
    /// <p> The filter formula to use to find existing matching rows to update. The formula needs to return zero or more rows. If the formula returns 0 rows, then a new row will be appended in the target table. If the formula returns one or more rows, then the returned rows will be updated. </p>
    /// <p> Note that the filter formula needs to return rows from the target table for the upsert operation to succeed. If the filter formula has a syntax error or it doesn't evaluate to zero or more rows in the target table for any one item in the input list, then the entire BatchUpsertTableRows request fails and no updates are made to the table. </p>
    /// This field is required.
    pub fn filter(mut self, input: crate::types::Filter) -> Self {
        self.filter = ::std::option::Option::Some(input);
        self
    }
    /// <p> The filter formula to use to find existing matching rows to update. The formula needs to return zero or more rows. If the formula returns 0 rows, then a new row will be appended in the target table. If the formula returns one or more rows, then the returned rows will be updated. </p>
    /// <p> Note that the filter formula needs to return rows from the target table for the upsert operation to succeed. If the filter formula has a syntax error or it doesn't evaluate to zero or more rows in the target table for any one item in the input list, then the entire BatchUpsertTableRows request fails and no updates are made to the table. </p>
    pub fn set_filter(mut self, input: ::std::option::Option<crate::types::Filter>) -> Self {
        self.filter = input;
        self
    }
    /// <p> The filter formula to use to find existing matching rows to update. The formula needs to return zero or more rows. If the formula returns 0 rows, then a new row will be appended in the target table. If the formula returns one or more rows, then the returned rows will be updated. </p>
    /// <p> Note that the filter formula needs to return rows from the target table for the upsert operation to succeed. If the filter formula has a syntax error or it doesn't evaluate to zero or more rows in the target table for any one item in the input list, then the entire BatchUpsertTableRows request fails and no updates are made to the table. </p>
    pub fn get_filter(&self) -> &::std::option::Option<crate::types::Filter> {
        &self.filter
    }
    /// Adds a key-value pair to `cells_to_update`.
    ///
    /// To override the contents of this collection use [`set_cells_to_update`](Self::set_cells_to_update).
    ///
    /// <p> A map representing the cells to update for the matching rows or an appended row. The key is the column id of the cell and the value is the CellInput object that represents the data to set in that cell. </p>
    pub fn cells_to_update(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::CellInput) -> Self {
        let mut hash_map = self.cells_to_update.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.cells_to_update = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p> A map representing the cells to update for the matching rows or an appended row. The key is the column id of the cell and the value is the CellInput object that represents the data to set in that cell. </p>
    pub fn set_cells_to_update(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::CellInput>>,
    ) -> Self {
        self.cells_to_update = input;
        self
    }
    /// <p> A map representing the cells to update for the matching rows or an appended row. The key is the column id of the cell and the value is the CellInput object that represents the data to set in that cell. </p>
    pub fn get_cells_to_update(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::CellInput>> {
        &self.cells_to_update
    }
    /// Consumes the builder and constructs a [`UpsertRowData`](crate::types::UpsertRowData).
    /// This method will fail if any of the following fields are not set:
    /// - [`batch_item_id`](crate::types::builders::UpsertRowDataBuilder::batch_item_id)
    /// - [`cells_to_update`](crate::types::builders::UpsertRowDataBuilder::cells_to_update)
    pub fn build(self) -> ::std::result::Result<crate::types::UpsertRowData, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::UpsertRowData {
            batch_item_id: self.batch_item_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "batch_item_id",
                    "batch_item_id was not specified but it is required when building UpsertRowData",
                )
            })?,
            filter: self.filter,
            cells_to_update: self.cells_to_update.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "cells_to_update",
                    "cells_to_update was not specified but it is required when building UpsertRowData",
                )
            })?,
        })
    }
}
