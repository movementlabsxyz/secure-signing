use crate::cryptography::HashiCorpVaultCryptographySpec;
use secure_signer::cryptography::ed25519::Ed25519;
use vaultrs::api::transit::KeyType;

impl HashiCorpVaultCryptographySpec for Ed25519 {
	fn key_type() -> KeyType {
		KeyType::Ed25519
	}
}
