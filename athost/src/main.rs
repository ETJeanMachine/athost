use athost_identity::DID; // imports the DID struct

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let did_test: DID = DID::from_str("did:web:nat.vg")?;
    let did_plc_test: DID = DID::from_str("did:plc:6vxtya3serxcwvcdk5e7psvv")?;
    println!("{}", did_test.to_string());
    let document = did_test.fetch_document().await?;
    println!("{:?}", document);
    let document_plc = did_plc_test.fetch_document().await?;
    println!("{:?}", document_plc);
    Ok(())
}
