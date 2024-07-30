use crate::Result;
use color_eyre::eyre::eyre;

use tokio::process::Command;

pub async fn start_instance(id: &str) -> Result<()> {
    let out = Command::new("oci")
        .arg("compute")
        .arg("instance")
        .arg("action")
        .arg("--action")
        .arg("START")
        .arg("--instance-id")
        .arg(id)
        .output();

    let out = out.await?;

    if out.status.success() {
        return Ok(());
    }

    let out_str = String::from_utf8(out.stderr)?;

    return Err(eyre!("failed to start instance: ```{}```", out_str));
}
