use esp_idf_svc::hal::delay::FreeRtos;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    for i in 0..10 {
        log::info!("Hello, world! {}", i);
        // sleep 1 second
        FreeRtos::delay_ms(1000);
    }
}
