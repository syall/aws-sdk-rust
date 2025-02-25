/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![cfg(feature = "test-util")]

use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::{Client, Config};
use aws_smithy_runtime::client::http::test_util::capture_request;

#[tokio::test]
async fn test_s3_signer_query_string_with_all_valid_chars() {
    let (http_client, rcvr) = capture_request(None);
    let config = Config::builder()
        .credentials_provider(SharedCredentialsProvider::new(
            Credentials::for_tests_with_session_token(),
        ))
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .with_test_defaults()
        .build();
    let client = Client::from_conf(config);

    // Generate a string containing all printable ASCII chars
    let prefix: String = (32u8..127).map(char::from).collect();

    // The response from the fake connection won't return the expected XML but we don't care about
    // that error in this test
    let _ = client
        .list_objects_v2()
        .bucket("test-bucket")
        .prefix(&prefix)
        .send()
        .await;

    let expected_req = rcvr.expect_request();
    let auth_header = expected_req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_owned();

    // This is a snapshot test taken from a known working test result
    let snapshot_signature =
        "Signature=9a931d20606f93fa4e5553602866a9b5ccac2cd42b54ae5a4b17e4614fb443ce";
    assert!(
        auth_header
            .contains(snapshot_signature),
        "authorization header signature did not match expected signature: got {}, expected it to contain {}",
        auth_header,
        snapshot_signature
    );
}

// This test can help identify individual characters that break the signing of query strings. This
// test must be run against an actual bucket so we `ignore` it unless the runner specifically requests it
#[tokio::test]
#[ignore]
async fn test_query_strings_are_correctly_encoded() {
    use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Error;
    use aws_smithy_runtime_api::client::result::SdkError;

    tracing_subscriber::fmt::init();
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let mut chars_that_break_signing = Vec::new();
    let mut chars_that_break_uri_parsing = Vec::new();
    let mut chars_that_are_invalid_arguments = Vec::new();

    // We test all possible bytes to check for issues with URL construction or signing
    for byte in u8::MIN..u8::MAX {
        let char = char::from(byte);
        let res = client
            .list_objects_v2()
            .bucket("a-bucket-to-test-with")
            .prefix(char)
            .send()
            .await;
        if let Err(SdkError::ServiceError(context)) = res {
            match context.err() {
                ListObjectsV2Error::Unhandled(e)
                    if e.to_string().contains("SignatureDoesNotMatch") =>
                {
                    chars_that_break_signing.push(byte);
                }
                ListObjectsV2Error::Unhandled(e) if e.to_string().contains("InvalidUri") => {
                    chars_that_break_uri_parsing.push(byte);
                }
                ListObjectsV2Error::Unhandled(e) if e.to_string().contains("InvalidArgument") => {
                    chars_that_are_invalid_arguments.push(byte);
                }
                ListObjectsV2Error::Unhandled(e) if e.to_string().contains("InvalidToken") => {
                    panic!("refresh your credentials and run this test again");
                }
                e => todo!("unexpected error: {:?}", e),
            }
        }
    }

    if chars_that_break_signing.is_empty()
        && chars_that_break_uri_parsing.is_empty()
        && chars_that_are_invalid_arguments.is_empty()
    {
        return;
    }

    fn char_transform(c: u8) -> String {
        format!("byte {}: {}\n", c, char::from(c))
    }
    if !chars_that_break_signing.is_empty() {
        eprintln!(
            "The following characters caused a signature mismatch:\n{}(end)",
            chars_that_break_signing
                .clone()
                .into_iter()
                .map(char_transform)
                .collect::<String>()
        );
    }
    if !chars_that_break_uri_parsing.is_empty() {
        eprintln!(
            "The following characters caused a URI parse failure:\n{}(end)",
            chars_that_break_uri_parsing
                .clone()
                .into_iter()
                .map(char_transform)
                .collect::<String>()
        );
    }
    if !chars_that_are_invalid_arguments.is_empty() {
        eprintln!(
            "The following characters caused an \"Invalid Argument\" failure:\n{}(end)",
            chars_that_are_invalid_arguments
                .clone()
                .into_iter()
                .map(char_transform)
                .collect::<String>()
        );
    }

    panic!("test failed due to invalid characters")
}
