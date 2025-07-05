use lettre::message::Message;
use lettre::transport::file::FileTransport;
use lettre::Transport;
use std::error::Error;
use std::fs;
use std::path::Path;

/// File-based email sender for testing
/// Writes each email as an .eml file into the `outbox/` directory
pub async fn send_email(
    to: &str,
    subject: &str,
    body: &str,
) -> Result<(), Box<dyn Error>> {
    // Ensure outbox directory exists
    let outbox = Path::new("outbox");
    if !outbox.exists() {
        fs::create_dir_all(outbox)?;
    }

    // Compose the email
    let email = Message::builder()
        .from("no-reply@local".parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    // FileTransport writes the raw message
    let file_transport = FileTransport::new(outbox);
    file_transport.send(&email)?;
    Ok(())
}
