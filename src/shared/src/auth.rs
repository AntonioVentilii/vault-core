use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::types::{DownloadToken, UploadToken};

type HmacSha256 = Hmac<Sha256>;

pub fn sign_token(token: &mut UploadToken, secret: &[u8]) {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");

    // Feed the token data into the HMAC (excluding the signature field itself)
    mac.update(&token.upload_id);
    mac.update(token.file_id.owner.as_slice());
    mac.update(&token.file_id.id);
    mac.update(token.bucket_id.as_slice());
    mac.update(token.directory_id.as_slice());
    mac.update(&token.expires_at.to_be_bytes());
    for &chunk in &token.allowed_chunks {
        mac.update(&chunk.to_be_bytes());
    }

    token.sig = mac.finalize().into_bytes().to_vec();
}

pub fn verify_token(token: &UploadToken, secret: &[u8]) -> bool {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");

    mac.update(&token.upload_id);
    mac.update(token.file_id.owner.as_slice());
    mac.update(&token.file_id.id);
    mac.update(token.bucket_id.as_slice());
    mac.update(token.directory_id.as_slice());
    mac.update(&token.expires_at.to_be_bytes());
    for &chunk in &token.allowed_chunks {
        mac.update(&chunk.to_be_bytes());
    }

    mac.verify_slice(&token.sig).is_ok()
}

pub fn sign_download_token(token: &mut DownloadToken, secret: &[u8]) {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");

    mac.update(token.file_id.owner.as_slice());
    mac.update(&token.file_id.id);
    mac.update(token.bucket_id.as_slice());
    mac.update(token.directory_id.as_slice());
    mac.update(&token.expires_at.to_be_bytes());

    token.sig = mac.finalize().into_bytes().to_vec();
}

pub fn verify_download_token(token: &DownloadToken, secret: &[u8]) -> bool {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");

    mac.update(token.file_id.owner.as_slice());
    mac.update(&token.file_id.id);
    mac.update(token.bucket_id.as_slice());
    mac.update(token.directory_id.as_slice());
    mac.update(&token.expires_at.to_be_bytes());

    mac.verify_slice(&token.sig).is_ok()
}
