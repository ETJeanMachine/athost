pub enum DIDType {
    Web,
    PLC,
}

impl DIDType {
    fn as_str(&self) -> &'static str {
        match self {
            DIDType::Web => "web",
            DIDType::PLC => "plc",
        }
    }
}

pub struct DID {
    did_type: DIDType,
    value: String,
}

impl ToString for DID {
    fn to_string(&self) -> String {
        format!("did:{}:{}", self.did_type.as_str(), self.value)
    }
}

impl DID {
    pub fn new(did_type: DIDType, value: String) -> Self {
        DID { did_type, value }
    }

    pub fn from_string(did: &str) -> Result<Self, String> {
        let parts: Vec<&str> = did.split(':').collect();
        if parts.len() != 3 {
            return Err("Invalid DID format".to_string());
        }
        let did_type = match parts[1] {
            "web" => DIDType::Web,
            "plc" => DIDType::PLC,
            _ => return Err("Unsupported DID type".to_string()),
        };
        Ok(DID {
            did_type,
            value: parts[2].to_string(),
        })
    }

    pub fn resolve_handle() {
        todo!()
    }

    pub fn uri(&self) -> String {
        format!("at://{:?}", self.to_string())
    }
}
