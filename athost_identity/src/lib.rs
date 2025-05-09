use std::error::Error as StdError;

use regex::Regex;
use serde::{Deserialize, Serialize};

type BoxDynError = Box<dyn StdError + 'static + Send + Sync>;

/// DIDMethod represents the method used to represent a DID.
enum DIDMethod {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DIDDocument {
    #[serde(rename = "@context")]
    context: Vec<String>,
    id: String,
    #[serde(rename = "alsoKnownAs")]
    also_known_as: Vec<String>,
    #[serde(rename = "verificationMethod")]
    verification_method: Vec<DIDVerificationMethod>,
    service: Vec<DIDService>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DIDVerificationMethod {
    id: String,
    r#type: String,
    controller: String,
    #[serde(rename = "publicKeyMultibase")]
    public_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DIDService {
    id: String,
    r#type: String,
    #[serde(rename = "serviceEndpoint")]
    service_endpoint: String,
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
    /// let did = DID::new("web", "example.com".to_string());
    /// ```
    fn new(method: &str, identifier: String) -> Result<Self, String> {
        let method = DIDMethod::from_str(method)?;
        if DID::validate_identifier(&method, identifier.as_str()) {
            Ok(DID { method, identifier })
        } else {
            Err("Invalid identifier".to_string())
        }
    }

    /// Helper method for the identity interface for validating did:web or did:plc identifiers.
    fn validate_identifier(method: &DIDMethod, identifier: &str) -> bool {
        let reg_str = match method {
            DIDMethod::Web => {
                r"^([a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?\.)+[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?$"
            }
            DIDMethod::PLC => r"^[a-z0-9]+$",
        };
        let re = Regex::new(reg_str).unwrap();
        re.is_match(identifier)
    }

    pub fn from_str(did: &str) -> Result<Self, String> {
        let re = Regex::new(r"^did:[a-z]+:[a-zA-Z0-9._:%-]*[a-zA-Z0-9._-]$").unwrap();
        if !re.is_match(did) {
            return Err("Invalid DID Syntax.".to_string());
        }
        let parts: Vec<&str> = did.split(':').collect();
        // will return an error if the identifier and/or is invalid
        DID::new(parts[1], parts[2].to_string())
    }

    pub async fn fetch_document(&self) -> Result<DIDDocument, BoxDynError> {
        let document_url = match self.method {
            DIDMethod::PLC => format!("https://plc.directory/{}", self.to_string()),
            DIDMethod::Web => format!("https://{}/.well-known/did.json", self.identifier),
        };
        let result = reqwest::get(&document_url).await?.text().await?;
        let doc = serde_json::from_str(&result)?;

        Ok(doc)
    }

    pub async fn resolve_handle(&self) -> Result<String, BoxDynError> {
        let did_document = self.fetch_document().await?;
        let handle = did_document
            .also_known_as
            .get(0)
            .ok_or("DID Document is missing a handle.")?;
        Ok(handle.to_string())
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
