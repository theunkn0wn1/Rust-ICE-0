#![no_std]
#[cfg(test)]

use async_trait::async_trait;
use ufmt::{uWrite, uwriteln};


#[async_trait]
trait MyTraitAsync<ComError>{
    const HEADER: &'static str;

    async fn read(&mut self, cmd: VarOffset) -> Result<u16, ComError>;
    // Print one of the internal variables of the controller to a uWrite
    async fn ushow_var<W: uWrite>(&mut self, f: &mut W, var: VarOffset) -> Result<(), ComError> {
        uwriteln!(f, "{:?}: {}", var, self.read(var).await.ok().unwrap()).ok();
        Ok(())
    }
}