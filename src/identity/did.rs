use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum DIDType {
    Web,
    PLC,
}

impl DIDType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DIDType::Web => "web",
            DIDType::PLC => "plc",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DID {
    didType: DIDType,
    value: String,
}

impl DID {
    pub fn new(didType: DIDType, value: String) -> Self {
        DID { didType, value }
    }

    pub fn from_string(did: &str) -> Result<Self, String> {
        let parts: Vec<&str> = did.split(':').collect();
        if parts.len() != 3 {
            return Err("Invalid DID format".to_string());
        }
        let didType = match parts[1] {
            "web" => DIDType::Web,
            "plc" => DIDType::PLC,
            _ => return Err("Unsupported DID type".to_string()),
        };
        Ok(DID {
            didType,
            value: parts[2].to_string(),
        })
    }

    pub fn uri(&self) -> String {
        format!("at://did:{:?}:{}", self.didType, self.value)
    }
}
