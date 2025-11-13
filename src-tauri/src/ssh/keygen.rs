use anyhow::{anyhow, Result};
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;

pub fn get_key_type(key_path: &str) -> Result<String> {
    let path = Path::new(key_path);
    if !path.exists() {
        return Err(anyhow!("Key file does not exist"));
    }

    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.contains("BEGIN OPENSSH PRIVATE KEY") {
        // Parse OpenSSH format
        if contents.contains("ssh-ed25519") {
            Ok("ed25519".to_string())
        } else if contents.contains("ecdsa") {
            Ok("ecdsa".to_string())
        } else if contents.contains("ssh-rsa") {
            Ok("rsa".to_string())
        } else {
            Ok("unknown".to_string())
        }
    } else if contents.contains("BEGIN RSA PRIVATE KEY") {
        Ok("rsa".to_string())
    } else if contents.contains("BEGIN EC PRIVATE KEY") {
        Ok("ecdsa".to_string())
    } else if contents.contains("BEGIN DSA PRIVATE KEY") {
        Ok("dsa".to_string())
    } else {
        Ok("unknown".to_string())
    }
}

pub fn generate_ed25519_keypair(output_path: &str) -> Result<String> {
    let private_key_path = output_path;
    let public_key_path = format!("{}.pub", output_path);

    // Use ssh-keygen to generate ed25519 key
    let output = Command::new("ssh-keygen")
        .args(&[
            "-t",
            "ed25519",
            "-f",
            private_key_path,
            "-N",
            "", // No passphrase
            "-C",
            "gterm-generated",
        ])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to generate keypair: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Read public key
    let public_key = fs::read_to_string(&public_key_path)?;

    // Set proper permissions on private key (0600)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(private_key_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(private_key_path, perms)?;
    }

    Ok(public_key)
}
