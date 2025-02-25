// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_accelerators_output_next_token(
    input: &crate::operation::list_accelerators::ListAcceleratorsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_byoip_cidrs_output_next_token(
    input: &crate::operation::list_byoip_cidrs::ListByoipCidrsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_custom_routing_accelerators_output_next_token(
    input: &crate::operation::list_custom_routing_accelerators::ListCustomRoutingAcceleratorsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_custom_routing_endpoint_groups_output_next_token(
    input: &crate::operation::list_custom_routing_endpoint_groups::ListCustomRoutingEndpointGroupsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_custom_routing_listeners_output_next_token(
    input: &crate::operation::list_custom_routing_listeners::ListCustomRoutingListenersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_custom_routing_port_mappings_output_next_token(
    input: &crate::operation::list_custom_routing_port_mappings::ListCustomRoutingPortMappingsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_custom_routing_port_mappings_by_destination_output_next_token(
    input: &crate::operation::list_custom_routing_port_mappings_by_destination::ListCustomRoutingPortMappingsByDestinationOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_endpoint_groups_output_next_token(
    input: &crate::operation::list_endpoint_groups::ListEndpointGroupsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_listeners_output_next_token(
    input: &crate::operation::list_listeners::ListListenersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_accelerators_output_accelerators(
    input: crate::operation::list_accelerators::ListAcceleratorsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Accelerator>> {
    let input = match input.accelerators {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_byoip_cidrs_output_byoip_cidrs(
    input: crate::operation::list_byoip_cidrs::ListByoipCidrsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ByoipCidr>> {
    let input = match input.byoip_cidrs {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_custom_routing_accelerators_output_accelerators(
    input: crate::operation::list_custom_routing_accelerators::ListCustomRoutingAcceleratorsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::CustomRoutingAccelerator>> {
    let input = match input.accelerators {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_custom_routing_endpoint_groups_output_endpoint_groups(
    input: crate::operation::list_custom_routing_endpoint_groups::ListCustomRoutingEndpointGroupsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::CustomRoutingEndpointGroup>> {
    let input = match input.endpoint_groups {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_custom_routing_listeners_output_listeners(
    input: crate::operation::list_custom_routing_listeners::ListCustomRoutingListenersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::CustomRoutingListener>> {
    let input = match input.listeners {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_custom_routing_port_mappings_output_port_mappings(
    input: crate::operation::list_custom_routing_port_mappings::ListCustomRoutingPortMappingsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::PortMapping>> {
    let input = match input.port_mappings {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_custom_routing_port_mappings_by_destination_output_destination_port_mappings(
    input: crate::operation::list_custom_routing_port_mappings_by_destination::ListCustomRoutingPortMappingsByDestinationOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::DestinationPortMapping>> {
    let input = match input.destination_port_mappings {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_endpoint_groups_output_endpoint_groups(
    input: crate::operation::list_endpoint_groups::ListEndpointGroupsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::EndpointGroup>> {
    let input = match input.endpoint_groups {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_listeners_output_listeners(
    input: crate::operation::list_listeners::ListListenersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Listener>> {
    let input = match input.listeners {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}
