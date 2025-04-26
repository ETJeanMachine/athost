use regex::Regex;

/// DIDMethod represents the method used to represent a DID.
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

    fn validate_identifier(&self, identifier: &str) -> Result<(), String> {
        match self {
            DIDMethod::Web => {
                // Validate format: domain.tld or subdomain.domain.tld (without protocol or trailing slashes)
                let re = Regex::new(r"^([a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?\.)+[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?$").unwrap();
                if re.is_match(identifier) {
                    Ok(())
                } else {
                    Err("Invalid Web DID identifier: must be a valid domain".to_string())
                }
            }
            DIDMethod::PLC => {
                // PLC DID's are just lowercase alphanumeric strings
                let re = Regex::new(r"^[a-z0-9]+$").unwrap();
                if re.is_match(identifier) {
                    Ok(())
                } else {
                    Err("Invalid PLC DID identifier".to_string())
                }
            }
        }
    }

    /// Returns the string representation of the DID method.
    ///
    /// # Returns
    ///
    /// A static string representing the DID method:
    /// - `"web"` for Web DIDs
    /// - `"plc"` for PLC DIDs
    ///
    /// # Examples
    ///
    /// ```
    /// let method = DIDMethod::Web;
    /// assert_eq!(method.as_str(), "web");
    /// ```
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
    /// Creates a new DID struct with the specified DID method and identifier value.
    ///
    /// # Arguments
    ///
    /// `method` - The DID method to use (e.g., Web, PLC)
    /// `identifier` - The identifier string for this DID
    ///
    /// # Returns
    ///
    /// A new DID struct if the identifier is valid, otherwise an error message.
    ///
    /// # Examples
    ///
    /// ```
    /// let did = DID::new(DIDMethod::Web, "example.com".to_string());
    /// ```
    pub fn new(method: DIDMethod, identifier: String) -> Result<Self, String> {
        match method.validate_identifier(identifier.as_str()) {
            Ok(_) => Ok(DID { method, identifier }),
            Err(err) => Err(err),
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
        // will return an error here if the method isn't supported. should ideally be expandable to future methods.
        let method = DIDMethod::from_str(parts[1])?;
        let identifier = parts[2].to_string();
        // will return an error if the identifier is invalid
        DID::new(method, identifier)
    }

    pub async fn resolve_handle(&self) -> Result<(), String> {
        match self.method {
            DIDMethod::Web => {
                let response =
                    reqwest::get(format!("https://{}/.well-known/did.json", self.identifier)).await;
                match response {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Failed to resolve DID".to_string()),
                }
            }
            DIDMethod::PLC => todo!(),
        }
    }

    /// Converts the DID to a URI format prefixed with "at://".
    ///
    /// # Returns
    ///
    /// A string representing this DID as a URI with the "at://" protocol prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// let did = DID::new(DIDMethod::Web, "example.com".to_string());
    /// assert_eq!(did.uri(), "at://did:web:example.com");
    /// ```
    pub fn uri(&self) -> String {
        format!("at://{:?}", self.to_string())
    }
}
