use athost_identity::DID; // imports the DID struct

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let did_test = DID::from_string("did:plc:6vxtya3serxcwvcdk5e7psvv")?;
    println!("{}", did_test.to_string());
    println!("");
    Ok(())
}
