
#[expect(
        dead_code(
        encryption_type = "Encryption variant stores error type",
        details = "Failed to decrypt secret with age encryption"
    )]
    UnsupportedEncryptionType { encryption_type: String }
