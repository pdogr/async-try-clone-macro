use async_trait::async_trait;
use async_try_clone_macro::AsyncTryClone;
pub type Result<T, E = Error> = core::result::Result<T, E>;
#[async_trait]
pub trait AsyncTryClone: Sized {
    async fn async_try_clone(&self) -> Result<Self, Error>;
}
pub struct Error;

#[async_trait]
impl<T: Clone + Sync> AsyncTryClone for T {
    async fn async_try_clone(&self) -> Result<Self, Error> {
        Ok(self.clone())
    }
}
#[derive(AsyncTryClone)]
pub struct Tmp<T> {
    a: T,
    b: String,
    c: u32,
}
fn assert_impl<T: AsyncTryClone>() {}
fn main() {
    assert_impl::<String>();
    assert_impl::<Tmp<u32>>();
}
