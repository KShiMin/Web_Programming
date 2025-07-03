// Import necessary modules and traits from external crates `jsonwebtoken`, `serde`, `std`, and `uuid`.
// These are used for JWT creation/validation, serialization/deserialization, environment variable handling, and UUID generation.
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::env;
use uuid::Uuid;
use easy_salt::salty_sha::*;


#[derive(Serialize, Deserialize)]
pub struct Staff {
    staff_id: Uuid,
    name: String,
    // password: ,
}

// A public function that creates a JWT token for a given user ID.
// It takes a `Uuid` parameter representing the user's unique identifier and returns a String (the JWT).
pub fn create_token(staff_id: Uuid) -> String {
    // Calculate expiration time for the token. This example sets it to one hour from the current time.
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .unwrap() // Ensure the addition operation doesn't fail.
        .timestamp() as usize; // Convert the expiration time to a `usize`.

    let hash = salty_sha256("", length)
        
    // Create a `Claims` instance with the user ID as the subject and the calculated expiration time.
    let staff = Staff {
        staff_id: staff_id.to_string(), // Convert `Uuid` to string to store in `sub`.
        name: "staff1".to_string(),
        password: 
    };

    // Encode the claims into a JWT using a default header and a secret key.
    // The secret key can be any byte array, but it should be kept secure and private.
    encode(
        &Header::default(), // Use default JWT header settings.
        &staff,            // Pass in the claims data.
        &EncodingKey::from_secret(b"schoolsalt2025"), // Secret key for encoding.
    )
    .unwrap() // Unwrap the result, assuming encoding is successful.
}

// A public function that validates a JWT token.
// It takes a string slice representing the token and returns a boolean indicating if the token is valid.
pub fn validate_token(token: &str) -> bool {
    // Attempt to decode the token using the same secret key used for encoding and default validation settings.
    decode::<Staff>(
        token,                                 // The JWT to be decoded.
        &DecodingKey::from_secret(b"secretkey"), // Secret key must match the one used during encoding.
        &Validation::default(),               // Use default validation parameters.
    )
    .is_ok() // Check if the decoding operation was successful.
}