mod aa0;
mod aa1;
mod aa2;
mod aa3;

use super::Session;
pub use aa0::AuthorizatinArea0;
pub use aa1::AuthorizationArea1;
pub use aa2::AuthorizationArea2;
pub use aa3::AuthorizationArea3;

pub trait AA<T: Session, U: Session, V: Session> {
    fn decompose(self) -> (Option<T>, Option<U>, Option<V>);
    fn decompose_ref(&self) -> (Option<&T>, Option<&U>, Option<&V>);
    fn decompose_mut(&mut self) -> (Option<&mut T>, Option<&mut U>, Option<&mut V>);
    fn is_empty(&self) -> bool {
        self.decompose_ref().0.is_none()
    }
}

/// Authorization area with 1+ sessions
pub trait AA1P<T: Session, U: Session, V: Session>: AA<T, U, V> {
    fn decompose(self) -> (T, Option<U>, Option<V>);
    fn decompose_ref(&self) -> (&T, Option<&U>, Option<&V>);
    fn decompose_mut(&mut self) -> (&mut T, Option<&mut U>, Option<&mut V>);
}

/// Authorization area with 2+ sessions
pub trait AA2P<T: Session, U: Session, V: Session>: AA1P<T, U, V> {
    fn decompose(self) -> (T, U, Option<V>);
    fn decompose_ref(&self) -> (&T, &U, Option<&V>);
    fn decompose_mut(&mut self) -> (&mut T, &mut U, Option<&mut V>);
}