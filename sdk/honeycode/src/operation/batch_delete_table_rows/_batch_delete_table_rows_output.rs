// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchDeleteTableRowsOutput {
    /// <p>The updated workbook cursor after deleting the rows from the table.</p>
    pub workbook_cursor: i64,
    /// <p> The list of row ids in the request that could not be deleted from the table. Each element in this list contains one row id from the request that could not be deleted along with the reason why that item could not be deleted. </p>
    pub failed_batch_items: ::std::option::Option<::std::vec::Vec<crate::types::FailedBatchItem>>,
    _request_id: Option<String>,
}
impl BatchDeleteTableRowsOutput {
    /// <p>The updated workbook cursor after deleting the rows from the table.</p>
    pub fn workbook_cursor(&self) -> i64 {
        self.workbook_cursor
    }
    /// <p> The list of row ids in the request that could not be deleted from the table. Each element in this list contains one row id from the request that could not be deleted along with the reason why that item could not be deleted. </p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.failed_batch_items.is_none()`.
    pub fn failed_batch_items(&self) -> &[crate::types::FailedBatchItem] {
        self.failed_batch_items.as_deref().unwrap_or_default()
    }
}
impl ::aws_http::request_id::RequestId for BatchDeleteTableRowsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchDeleteTableRowsOutput {
    /// Creates a new builder-style object to manufacture [`BatchDeleteTableRowsOutput`](crate::operation::batch_delete_table_rows::BatchDeleteTableRowsOutput).
    pub fn builder() -> crate::operation::batch_delete_table_rows::builders::BatchDeleteTableRowsOutputBuilder {
        crate::operation::batch_delete_table_rows::builders::BatchDeleteTableRowsOutputBuilder::default()
    }
}

/// A builder for [`BatchDeleteTableRowsOutput`](crate::operation::batch_delete_table_rows::BatchDeleteTableRowsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct BatchDeleteTableRowsOutputBuilder {
    pub(crate) workbook_cursor: ::std::option::Option<i64>,
    pub(crate) failed_batch_items: ::std::option::Option<::std::vec::Vec<crate::types::FailedBatchItem>>,
    _request_id: Option<String>,
}
impl BatchDeleteTableRowsOutputBuilder {
    /// <p>The updated workbook cursor after deleting the rows from the table.</p>
    /// This field is required.
    pub fn workbook_cursor(mut self, input: i64) -> Self {
        self.workbook_cursor = ::std::option::Option::Some(input);
        self
    }
    /// <p>The updated workbook cursor after deleting the rows from the table.</p>
    pub fn set_workbook_cursor(mut self, input: ::std::option::Option<i64>) -> Self {
        self.workbook_cursor = input;
        self
    }
    /// <p>The updated workbook cursor after deleting the rows from the table.</p>
    pub fn get_workbook_cursor(&self) -> &::std::option::Option<i64> {
        &self.workbook_cursor
    }
    /// Appends an item to `failed_batch_items`.
    ///
    /// To override the contents of this collection use [`set_failed_batch_items`](Self::set_failed_batch_items).
    ///
    /// <p> The list of row ids in the request that could not be deleted from the table. Each element in this list contains one row id from the request that could not be deleted along with the reason why that item could not be deleted. </p>
    pub fn failed_batch_items(mut self, input: crate::types::FailedBatchItem) -> Self {
        let mut v = self.failed_batch_items.unwrap_or_default();
        v.push(input);
        self.failed_batch_items = ::std::option::Option::Some(v);
        self
    }
    /// <p> The list of row ids in the request that could not be deleted from the table. Each element in this list contains one row id from the request that could not be deleted along with the reason why that item could not be deleted. </p>
    pub fn set_failed_batch_items(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::FailedBatchItem>>) -> Self {
        self.failed_batch_items = input;
        self
    }
    /// <p> The list of row ids in the request that could not be deleted from the table. Each element in this list contains one row id from the request that could not be deleted along with the reason why that item could not be deleted. </p>
    pub fn get_failed_batch_items(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::FailedBatchItem>> {
        &self.failed_batch_items
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`BatchDeleteTableRowsOutput`](crate::operation::batch_delete_table_rows::BatchDeleteTableRowsOutput).
    pub fn build(self) -> crate::operation::batch_delete_table_rows::BatchDeleteTableRowsOutput {
        crate::operation::batch_delete_table_rows::BatchDeleteTableRowsOutput {
            workbook_cursor: self.workbook_cursor.unwrap_or_default(),
            failed_batch_items: self.failed_batch_items,
            _request_id: self._request_id,
        }
    }
}
