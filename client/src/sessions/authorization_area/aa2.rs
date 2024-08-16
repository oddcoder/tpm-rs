use crate::sessions::{PasswordSession, Session};

use super::{AA, AA1P, AA2P};

pub struct AuthorizationArea2<T: Session, U: Session>(T, U);

impl<T: Session, U: Session> AuthorizationArea2<T, U> {
    pub fn new(s1: T, s2: U) -> Self {
        AuthorizationArea2(s1, s2)
    }
}

impl<T: Session, U: Session> AA<T, U, PasswordSession> for AuthorizationArea2<T, U> {
    fn decompose(self) -> (Option<T>, Option<U>, Option<PasswordSession>) {
        (Some(self.0), Some(self.1), None)
    }

    fn decompose_ref(&self) -> (Option<&T>, Option<&U>, Option<&PasswordSession>) {
        (Some(&self.0), Some(&self.1), None)
    }

    fn decompose_mut(&mut self) -> (Option<&mut T>, Option<&mut U>, Option<&mut PasswordSession>) {
        (Some(&mut self.0), Some(&mut self.1), None)
    }
}

impl<T: Session, U: Session> AA1P<T, U, PasswordSession> for AuthorizationArea2<T, U> {
    fn decompose(self) -> (T, Option<U>, Option<PasswordSession>) {
        (self.0, Some(self.1), None)
    }

    fn decompose_ref(&self) -> (&T, Option<&U>, Option<&PasswordSession>) {
        (&self.0, Some(&self.1), None)
    }

    fn decompose_mut(&mut self) -> (&mut T, Option<&mut U>, Option<&mut PasswordSession>) {
        (&mut self.0, Some(&mut self.1), None)
    }
}

impl<T: Session, U: Session> AA2P<T, U, PasswordSession> for AuthorizationArea2<T, U> {
    fn decompose(self) -> (T, U, Option<PasswordSession>) {
        (self.0, self.1, None)
    }

    fn decompose_ref(&self) -> (&T, &U, Option<&PasswordSession>) {
        (&self.0, &self.1, None)
    }

    fn decompose_mut(&mut self) -> (&mut T, &mut U, Option<&mut PasswordSession>) {
        (&mut self.0, &mut self.1, None)
    }
}
