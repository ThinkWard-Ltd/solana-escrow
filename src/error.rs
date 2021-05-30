use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    // Invalid instruction (defining an error type)
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

// Implementing a genetic traits, converting ```EscrowError``` to ```ProgramError```
impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}