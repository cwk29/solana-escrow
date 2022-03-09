// program specific errors
use thiserror::Error;

use solana_program::program_error::ProgramError;

// Supposed to add the new error variant but I have no fucking clue what I'm doing

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
  /// Invalid instruction
  #[error("Invalid Instruction")]
  InvalidInstruction,
  /// Not Rent Exempt
  #[error("Not Rent Exempt")]
  NotRentExempt,
}

impl From<EscrowError> for ProgramError {
  fn from(e: EscrowError) -> Self {
    ProgramError::Custom(e as u32)
  }
}