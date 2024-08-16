use crate::sessions::PasswordSession;

use super::AA;

pub struct AuthorizatinArea0;

impl AA<PasswordSession, PasswordSession, PasswordSession> for AuthorizatinArea0 {
    fn decompose(
        self,
    ) -> (
        Option<PasswordSession>,
        Option<PasswordSession>,
        Option<PasswordSession>,
    ) {
        (None, None, None)
    }

    fn decompose_ref(
        &self,
    ) -> (
        Option<&PasswordSession>,
        Option<&PasswordSession>,
        Option<&PasswordSession>,
    ) {
        (None, None, None)
    }

    fn decompose_mut(
        &mut self,
    ) -> (
        Option<&mut PasswordSession>,
        Option<&mut PasswordSession>,
        Option<&mut PasswordSession>,
    ) {
        (None, None, None)
    }
}
