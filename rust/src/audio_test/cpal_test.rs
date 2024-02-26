use cpal::traits::HostTrait;

use super::debug_log;

pub fn print_cpal_info_impl() {
    let hosts = cpal::available_hosts();
    debug_log::debug_log_string(format!("Available hosts: {:?}", hosts));

    let host = cpal::default_host();
    debug_log::debug_log_string(format!("default host: {:?}", host.id()));

    debug_log::debug_log_string(format!(
        "Host num input devices: {:?}",
        host.input_devices().unwrap().count()
    ));

    debug_log::debug_log_string(format!(
        "Host num output devices: {:?}",
        host.output_devices().unwrap().count()
    ));

    // debug_log::debug_log("Input devices:");
    // host.input_devices()
    //     .expect("Could not get input device list")
    //     .for_each(|device| {
    //         debug_log::debug_log_string(format!("Input device: {:?}", device.name()));
    //     });

    // let device = host
    //     .default_input_device()
    //     .expect("no inputs device available");

    // debug_log::debug_log_string(format!("default input device: {:?}", device.name()));

    // // let config = cpal::StreamConfig {
    // //     channels: NUM_CHANNELS as u16,
    // //     sample_rate: cpal::SampleRate(SAMPLE_RATE as u32),
    // //     buffer_size: cpal::BufferSize::Default,
    // // };

    // debug_log::debug_log_string(format!("Supported input configs:"));
    // device
    //     .supported_input_configs()
    //     .unwrap()
    //     .for_each(|config| {
    //         debug_log::debug_log_string(format!("Supported input config: {:?}", config));
    //     });

    // let config = device
    //     .default_input_config()
    //     .expect("Could not get default input config")
    //     .config();

    // debug_log::debug_log_string(format!("Stream config: {:?}", config));
}
