use tpm2_rs_base::{errors::TpmResult, TpmsAuthCommand, TpmsAuthResponse};

/// Trait for types representing TPM sessions.
pub trait Session {
    /// Computes the authorization HMAC for this session.
    fn get_auth_command(&self) -> TpmsAuthCommand;
    /// Validates the authorization response for this session.
    fn validate_auth_response(&self, auth: &TpmsAuthResponse) -> TpmResult<()>;
}
