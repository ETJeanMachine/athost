use regex::Regex;

pub enum DIDMethod {
    Web,
    PLC,
}

impl DIDMethod {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "web" => Ok(DIDMethod::Web),
            "plc" => Ok(DIDMethod::PLC),
            _ => Err("Unsupported DID method".to_string()),
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            DIDMethod::Web => "web",
            DIDMethod::PLC => "plc",
        }
    }
}

pub struct DID {
    method: DIDMethod,
    identifier: String,
}

impl ToString for DID {
    fn to_string(&self) -> String {
        format!("did:{}:{}", self.method.as_str(), self.identifier)
    }
}

impl DID {
    pub fn new(did_type: DIDMethod, value: String) -> Self {
        DID {
            method: did_type,
            identifier: value,
        }
    }

    /// Parses a DID string into a DID struct, and validates the format.
    ///
    /// # Arguments
    ///
    /// `did` - A string slice representing a DID in the format "did:method:identifier"
    ///
    /// # Returns
    ///
    /// `Result<Self, String>` - A Result containing either the parsed DID or an error message.
    ///
    /// # Examples
    ///
    /// ```
    /// let did = DID::from_string("did:web:example.com").unwrap();
    /// ```
    pub fn from_string(did: &str) -> Result<Self, String> {
        let re = Regex::new(r"^did:[a-z]+:[a-zA-Z0-9._:%-]*[a-zA-Z0-9._-]$").unwrap();
        if !re.is_match(did) {
            return Err("Invalid DID Syntax.".to_string());
        }
        let parts: Vec<&str> = did.split(':').collect();
        // will return an error here if the method isn't supported
        let method = DIDMethod::from_str(parts[1])?;
        Ok(DID {
            method,
            identifier: parts[2].to_string(),
        })
    }

    pub async fn resolve_handle(&self) -> Result<(), String> {
        match self.method {
            DIDMethod::Web => todo!(),
            DIDMethod::PLC => todo!(),
        }
    }

    pub fn uri(&self) -> String {
        format!("at://{:?}", self.to_string())
    }
}
