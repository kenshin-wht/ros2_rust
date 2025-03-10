use anyhow::{Error, Result};
use rclrs::*;

fn handle_service(
    _request_header: &rclrs::rmw_request_id_t,
    request: example_interfaces::srv::AddTwoInts_Request,
) -> example_interfaces::srv::AddTwoInts_Response {
    println!("request: {} + {}", request.a, request.b);
    example_interfaces::srv::AddTwoInts_Response {
        sum: request.a + request.b,
    }
}

fn main() -> Result<(), Error> {
    let mut executor = Context::default_from_env()?.create_basic_executor();

    let node = executor.create_node("minimal_service")?;

    let _server = node
        .create_service::<example_interfaces::srv::AddTwoInts, _>("add_two_ints", handle_service)?;

    println!("Starting server");
    executor
        .spin(SpinOptions::default())
        .first_error()
        .map_err(|err| err.into())
}
