use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub fn add_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
    location: String, // Adım 4
) -> ProgramResult {
    // Mevcut kod...

    let mut account_data =
        try_from_slice_unchecked::<AccountState>(&pda_account.data.borrow()).unwrap();
    // Mevcut kod...
    account_data.location = location; // Adım 4
    // Mevcut kod...

    Ok(())
}

pub fn update_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _title: String,
    rating: u8,
    description: String,
    location: String, // Adım 4
) -> ProgramResult {
    let mut account_data =
        try_from_slice_unchecked::<AccountState>(&pda_account.data.borrow()).unwrap();
    account_data.location = location; // Adım 4
  

    Ok(())
}


#[derive(BorshDeserialize)]
struct ReviewPayload {
    title: String,
    rating: u8,
    description: String,
    location: String, // Adım 2
}


impl ReviewInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let payload = ReviewPayload::try_from_slice(rest).unwrap();
        Ok(match variant {
            0 => Self::AddReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
            },
            1 => Self::UpdateReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum ReviewInstruction {
    AddReview {
        title: String,
        rating: u8,
        description: String,
    },
    UpdateReview {
        title: String,
        rating: u8,
        description: String,
    },
}

#[derive(BorshDeserialize)]
struct ReviewPayload {
    title: String,
    rating: u8,
    description: String,
}

impl ReviewInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let payload = ReviewPayload::try_from_slice(rest).unwrap();
        Ok(match variant {
            0 => Self::AddReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
                location: payload.location, // Adım 3
            },
            1 => Self::UpdateReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
                location: payload.location, // Adım 3
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum ReviewInstruction {
    AddReview {
        title: String,
        rating: u8,
        description: String,
    },
    UpdateReview {
        title: String,
        rating: u8,
        description: String,
    },
}

#[derive(BorshDeserialize)]
struct ReviewPayload {
    title: String,
    rating: u8,
    description: String,
}

impl ReviewInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let payload = ReviewPayload::try_from_slice(rest).unwrap();
        Ok(match variant {
            0 => Self::AddReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
            },
            1 => Self::UpdateReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
