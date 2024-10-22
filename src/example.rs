#[tokio::main]
async fn main() {
    let cnx = zbus::Connection::system().await.unwrap();
    let manager = fingerprint_reader::Manager::new(&cnx).await.unwrap();

    let device = manager.default_device().await.unwrap();

    println!("name={}", device.name().await.unwrap());
    println!(
        "num-enroll-stages={:?}",
        device.num_enroll_stages().await.unwrap()
    );
    println!("scan-type={:?}", device.scan_type().await.unwrap());
    println!("finger-present={}", device.finger_present().await.unwrap());
    println!("finger-needed={}", device.finger_needed().await.unwrap());
    println!(
        "list-enrolled-fingers={:?}",
        device.list_enrolled_fingers(None).await.unwrap()
    );
}
