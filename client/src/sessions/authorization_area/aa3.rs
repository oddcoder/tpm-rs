use crate::sessions::Session;

use super::{AA, AA1P, AA2P};

pub struct AuthorizationArea3<T: Session, U: Session, V: Session>(T, U, V);

impl<T: Session, U: Session, V: Session> AuthorizationArea3<T, U, V> {
    pub fn new(s1: T, s2: U, s3: V) -> Self {
        AuthorizationArea3(s1, s2, s3)
    }
    pub fn decompose(self) -> (T, U, V) {
        (self.0, self.1, self.2)
    }

    pub fn decompose_ref(&self) -> (&T, &U, &V) {
        (&self.0, &self.1, &self.2)
    }

    pub fn decompose_mut(&mut self) -> (&mut T, &mut U, &mut V) {
        (&mut self.0, &mut self.1, &mut self.2)
    }
}

impl<T: Session, U: Session, V: Session> AA<T, U, V> for AuthorizationArea3<T, U, V> {
    fn decompose(self) -> (Option<T>, Option<U>, Option<V>) {
        (Some(self.0), Some(self.1), Some(self.2))
    }

    fn decompose_ref(&self) -> (Option<&T>, Option<&U>, Option<&V>) {
        (Some(&self.0), Some(&self.1), Some(&self.2))
    }

    fn decompose_mut(&mut self) -> (Option<&mut T>, Option<&mut U>, Option<&mut V>) {
        (Some(&mut self.0), Some(&mut self.1), Some(&mut self.2))
    }
}

impl<T: Session, U: Session, V: Session> AA1P<T, U, V> for AuthorizationArea3<T, U, V> {
    fn decompose(self) -> (T, Option<U>, Option<V>) {
        (self.0, Some(self.1), Some(self.2))
    }

    fn decompose_ref(&self) -> (&T, Option<&U>, Option<&V>) {
        (&self.0, Some(&self.1), Some(&self.2))
    }

    fn decompose_mut(&mut self) -> (&mut T, Option<&mut U>, Option<&mut V>) {
        (&mut self.0, Some(&mut self.1), Some(&mut self.2))
    }
}

impl<T: Session, U: Session, V: Session> AA2P<T, U, V> for AuthorizationArea3<T, U, V> {
    fn decompose(self) -> (T, U, Option<V>) {
        (self.0, self.1, Some(self.2))
    }

    fn decompose_ref(&self) -> (&T, &U, Option<&V>) {
        (&self.0, &self.1, Some(&self.2))
    }

    fn decompose_mut(&mut self) -> (&mut T, &mut U, Option<&mut V>) {
        (&mut self.0, &mut self.1, Some(&mut self.2))
    }
}
