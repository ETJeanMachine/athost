use athost_identity::DID; // imports the DID struct

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let did_web_test: DID = DID::from_str("did:web:nat.vg")?;
    let did_plc_test: DID = DID::from_str("did:plc:6vxtya3serxcwvcdk5e7psvv")?;
    println!("{}", did_web_test.to_string());
    println!("{}", did_plc_test.to_string());
    let handle_web = did_web_test.resolve_handle().await?;
    let handle_plc = did_plc_test.resolve_handle().await?;
    println!("{}", handle_web);
    println!("{}", handle_plc);
    Ok(())
}
