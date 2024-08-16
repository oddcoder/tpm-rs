use super::{AA, AA1P};
use crate::sessions::{PasswordSession, Session};

pub struct AuthorizationArea1<T: Session>(T);

impl<T: Session> AuthorizationArea1<T> {
    pub fn new(session: T) -> Self {
        AuthorizationArea1(session)
    }
}

impl<T: Session> AA<T, PasswordSession, PasswordSession> for AuthorizationArea1<T> {
    fn decompose(self) -> (Option<T>, Option<PasswordSession>, Option<PasswordSession>) {
        (Some(self.0), None, None)
    }

    fn decompose_ref(
        &self,
    ) -> (
        Option<&T>,
        Option<&PasswordSession>,
        Option<&PasswordSession>,
    ) {
        (Some(&self.0), None, None)
    }

    fn decompose_mut(
        &mut self,
    ) -> (
        Option<&mut T>,
        Option<&mut PasswordSession>,
        Option<&mut PasswordSession>,
    ) {
        (Some(&mut self.0), None, None)
    }
}

impl<T: Session> AA1P<T, PasswordSession, PasswordSession> for AuthorizationArea1<T> {
    fn decompose(self) -> (T, Option<PasswordSession>, Option<PasswordSession>) {
        (self.0, None, None)
    }

    fn decompose_ref(&self) -> (&T, Option<&PasswordSession>, Option<&PasswordSession>) {
        (&self.0, None, None)
    }

    fn decompose_mut(
        &mut self,
    ) -> (
        &mut T,
        Option<&mut PasswordSession>,
        Option<&mut PasswordSession>,
    ) {
        (&mut self.0, None, None)
    }
}
