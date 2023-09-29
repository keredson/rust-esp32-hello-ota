use std::{thread::sleep, time::Duration};

use embedded_svc::{wifi::{ClientConfiguration, Configuration}, io::{Write}};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, wifi::EspWifi, nvs::EspDefaultNvsPartition, http::server::EspHttpServer};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;



const SSID: &str = "ssid";
const PASS: &str = "pass";
const BUILD_ID: &str = "7";

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();

    let _wifi = wifi(peripherals, sys_loop);
    let mut _server = setup_http_server();

    setup_ota(&mut _server);


    loop{
        sleep(Duration::new(10,0));
    }

}

fn setup_ota(server:&mut EspHttpServer) {
    server.fn_handler("/flash", embedded_svc::http::Method::Post, |mut request| {
        println!("got request: {:?}", request.uri());
        let mut ota = esp_ota::OtaUpdate::begin()?;
        let mut buf = [0; 1024];
        loop {
            let n = request.read(&mut buf).unwrap();
            if n==0 {break;}
            println!("read {:?}", n);
            ota.write(&buf[0..n]).unwrap();
        }
        println!("wrote img");
        let mut completed_ota = ota.finalize()?;
        println!("finalized");
        completed_ota.set_as_boot_partition()?;
        println!("set as boot partition");
        let mut resp = request.into_ok_response().unwrap();
        resp.write_all(b"OK")?;
        resp.flush()?;
        drop(resp);
        println!("restarting");
        sleep(Duration::new(1,0));
        completed_ota.restart();
    }).unwrap();
    server.fn_handler("/revert", embedded_svc::http::Method::Post, |request| Ok({
        println!("got request: {:?}", request.uri());
        request.into_ok_response().unwrap();
        esp_ota::rollback_and_reboot().unwrap();
    })).unwrap();
}

fn setup_http_server() -> EspHttpServer {
    let server_config = esp_idf_svc::http::server::Configuration::default();
    let mut server = EspHttpServer::new(&server_config).unwrap();
    server.fn_handler("/", embedded_svc::http::Method::Get, |request| {
        println!("got request: {:?} {:?}", request.uri(), BUILD_ID);
        Ok({
            let html = "build:";
            let mut response = request.into_ok_response().unwrap();
            response.write(html.as_bytes())?;
            response.write(BUILD_ID.as_bytes())?;
        })
    }).unwrap();
    return server;
}

fn wifi(peripherals: Peripherals, sys_loop:esp_idf_svc::eventloop::EspEventLoop<esp_idf_svc::eventloop::System>) -> anyhow::Result<EspWifi<'static>> {
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi_driver = EspWifi::new(
        peripherals.modem,
        sys_loop,
        Some(nvs)
    ).unwrap();

    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration{
        ssid: SSID.into(),
        password: PASS.into(),
        ..Default::default()
    })).unwrap();

    wifi_driver.start().unwrap();
    wifi_driver.connect().unwrap();
    while !wifi_driver.is_connected().unwrap(){
        let config = wifi_driver.get_configuration().unwrap();
        println!("Waiting for station: {:?}", config);
        sleep(Duration::new(0,100_000_000));
    }
    loop{
        let ip_info = wifi_driver.sta_netif().get_ip_info().unwrap();
        println!("Waiting for IP: {:?}", ip_info);
        sleep(Duration::new(1,0));
        if ip_info.ip.to_string() != "0.0.0.0" {
            println!("IP info: {:?}", ip_info);
            break;
        }
    }
    return Ok(wifi_driver);
}

