use libaeron_driver_sys as aeron_driver;

use std::ptr;
use std::ffi::{CStr, CString};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use libaeron_driver_sys::aeron_threading_mode_enum;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {

    // Flag to indicate when the application should stop (set on Ctrl+C)
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    // Register signal handler for SIGINT (Ctrl+C)
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    })?;

    // Create the media driver context
    let mut context: *mut aeron_driver::aeron_driver_context_t = ptr::null_mut();

    // Initialize the media driver context
    let result = unsafe { aeron_driver::aeron_driver_context_init(&mut context) };
    if result < 0 {
        return Err("Failed to initialize Aeron driver context".into());
    }

    print_aeron_config(context)?;

    // Create the media driver
    let mut driver: *mut aeron_driver::aeron_driver_t = ptr::null_mut();
    let result = unsafe { aeron_driver::aeron_driver_init(&mut driver, context) };
    if result < 0 {
        return Err("Failed to initialize Aeron driver".into());
    }

    // Start the media driver
    let result = unsafe { aeron_driver::aeron_driver_start(driver, false) };
    if result < 0 {
        return Err("Failed to start Aeron driver".into());
    }

    println!("Aeron media driver started successfully. Press Ctrl+C to stop.");

    // Poll for work until Ctrl+C is pressed
    while running.load(Ordering::Acquire) {
        while unsafe { aeron_driver::aeron_driver_main_do_work(driver) } != 0 {
            // busy spin
        }
    }

    println!("Received signal to stop the media driver...");

    // Clean up: stop and close the media driver
    let result = unsafe { aeron_driver::aeron_driver_close(driver) };
    if result < 0 {
        return Err("Failed to close Aeron driver".into());
    }

    // Clean up the context
    let result = unsafe { aeron_driver::aeron_driver_context_close(context) };
    if result < 0 {
        return Err("Failed to close Aeron driver context".into());
    }

    println!("Aeron media driver stopped successfully");
    Ok(())
}

fn threading_mode_to_str(mode: aeron_driver::aeron_threading_mode_t) -> &'static str {
    match mode {
        aeron_driver::aeron_threading_mode_enum::AERON_THREADING_MODE_DEDICATED => "DEDICATED",
        aeron_driver::aeron_threading_mode_enum::AERON_THREADING_MODE_SHARED_NETWORK => "SHARED_NETWORK",
        aeron_driver::aeron_threading_mode_enum::AERON_THREADING_MODE_SHARED => "SHARED",
        aeron_driver::aeron_threading_mode_enum::AERON_THREADING_MODE_INVOKER => "INVOKER",
        _ => "UNKNOWN",
    }
}

fn print_aeron_config(context: *mut aeron_driver::aeron_driver_context_t) -> Result<()> {

    let config_entries = vec![
        ("dir", format!("{:?}", unsafe { CStr::from_ptr(aeron_driver::aeron_driver_context_get_dir(context)) })),
        ("dir_warn_if_exists", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_dir_warn_if_exists(context) })),
        ("threading_mode",format!("{}", threading_mode_to_str(unsafe { aeron_driver::aeron_driver_context_get_threading_mode(context) }))),
        ("dir_delete_on_start", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_dir_delete_on_start(context) })),
        ("dir_delete_on_shutdown", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_dir_delete_on_shutdown(context) })),
        ("to_conductor_buffer_length", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_to_conductor_buffer_length(context) })),
        ("to_clients_buffer_length", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_to_clients_buffer_length(context) })),
        ("counters_buffer_length", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_counters_buffer_length(context) })),
        ("error_buffer_length", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_error_buffer_length(context) })),
        ("client_liveness_timeout_ns", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_client_liveness_timeout_ns(context) })),
        ("term_buffer_length", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_term_buffer_length(context) })),
        ("ipc_term_buffer_length", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_ipc_term_buffer_length(context) })),
        ("term_buffer_sparse_file", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_term_buffer_sparse_file(context) })),
        ("perform_storage_checks", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_perform_storage_checks(context) })),
        ("low_file_store_warning_threshold", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_low_file_store_warning_threshold(context) })),
        ("spies_simulate_connection", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_spies_simulate_connection(context) })),
        ("file_page_size", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_file_page_size(context) })),
        ("mtu_length", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_mtu_length(context) })),
        ("ipc_mtu_length", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_ipc_mtu_length(context) })),
        ("ipc_publication_term_window_length", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_ipc_publication_term_window_length(context) })),
        ("publication_term_window_length", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_publication_term_window_length(context) })),
        ("publication_linger_timeout_ns", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_publication_linger_timeout_ns(context) })),
        ("socket_so_rcvbuf", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_socket_so_rcvbuf(context) })),
        ("socket_so_sndbuf", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_socket_so_sndbuf(context) })),
        ("socket_multicast_ttl", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_socket_multicast_ttl(context) })),
        ("send_to_status_poll_ratio", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_send_to_status_poll_ratio(context) })),
        ("rcv_status_message_timeout_ns", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_rcv_status_message_timeout_ns(context) })),
        ("multicast_flowcontrol_supplier", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_multicast_flowcontrol_supplier(context) })),
        ("unicast_flowcontrol_supplier", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_unicast_flowcontrol_supplier(context) })),
        ("image_liveness_timeout_ns", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_image_liveness_timeout_ns(context) })),
        ("rcv_initial_window_length", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_rcv_initial_window_length(context) })),
        ("congestioncontrol_supplier", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_congestioncontrol_supplier(context) })),
        ("loss_report_buffer_length", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_loss_report_buffer_length(context) })),
        ("publication_unblock_timeout_ns", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_publication_unblock_timeout_ns(context) })),
        ("publication_connection_timeout_ns", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_publication_connection_timeout_ns(context) })),
        ("timer_interval_ns", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_timer_interval_ns(context) })),
        ("sender_idle_strategy", format!("{:?}", unsafe { CStr::from_ptr(aeron_driver::aeron_driver_context_get_sender_idle_strategy(context)) })),
        ("conductor_idle_strategy", format!("{:?}", unsafe { CStr::from_ptr(aeron_driver::aeron_driver_context_get_conductor_idle_strategy(context)) })),
        ("receiver_idle_strategy", format!("{:?}", unsafe { CStr::from_ptr(aeron_driver::aeron_driver_context_get_receiver_idle_strategy(context)) })),
        ("sharednetwork_idle_strategy", format!("{:?}", unsafe { CStr::from_ptr(aeron_driver::aeron_driver_context_get_sharednetwork_idle_strategy(context)) })),
        ("shared_idle_strategy", format!("{:?}", unsafe { CStr::from_ptr(aeron_driver::aeron_driver_context_get_shared_idle_strategy(context)) })),
        ("sender_idle_strategy_init_args", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_sender_idle_strategy_init_args(context) })),
        ("conductor_idle_strategy_init_args", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_conductor_idle_strategy_init_args(context) })),
        ("receiver_idle_strategy_init_args", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_receiver_idle_strategy_init_args(context) })),
        ("sharednetwork_idle_strategy_init_args", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_sharednetwork_idle_strategy_init_args(context) })),
        ("shared_idle_strategy_init_args", format!("{:?}", unsafe { aeron_driver::aeron_driver_context_get_shared_idle_strategy_init_args(context) })),
    ];

    // Find the maximum length of the keys
    let max_key_len = config_entries.iter().map(|(key, _)| key.len() + 2).max().unwrap_or(0);

    // Print the aligned configuration entries
    for (key, value) in config_entries {
        println!("{:width$}: {}", key, value, width = max_key_len);
    }

    println!();

    Ok(())
}
