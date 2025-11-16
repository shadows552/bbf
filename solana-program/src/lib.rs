use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    clock::Clock,
    sysvar::Sysvar,
};

/// Program entrypoint
entrypoint!(process_instruction);

/// Transaction types for product lifecycle
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum TransactionType {
    Manufacture,
    Repair,
    Transfer,
    EndOfLife,
}

/// Product provenance record stored on-chain
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ProductRecord {
    /// Unique product identifier (serial number)
    pub product_id: String,
    /// Transaction type
    pub transaction_type: TransactionType,
    /// Link to previous record (for chain validation)
    pub previous_record: Option<Pubkey>,
    /// Current owner's public key
    pub current_owner: Pubkey,
    /// Next owner's public key (for transfers)
    pub next_owner: Option<Pubkey>,
    /// Timestamp of transaction
    pub timestamp: i64,
    /// Optional metadata (repair details, etc.)
    pub metadata: String,
}

/// Instruction data for the program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ProductInstruction {
    /// Create a new product record (manufacturer only)
    CreateProduct {
        product_id: String,
        metadata: String,
    },
    /// Transfer ownership
    TransferOwnership {
        next_owner: Pubkey,
    },
    /// Record a repair
    RecordRepair {
        metadata: String,
    },
    /// Mark product as end-of-life
    MarkEndOfLife,
}

/// Program instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ProductInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        ProductInstruction::CreateProduct { product_id, metadata } => {
            msg!("Instruction: CreateProduct");
            create_product(program_id, accounts, product_id, metadata)
        }
        ProductInstruction::TransferOwnership { next_owner } => {
            msg!("Instruction: TransferOwnership");
            transfer_ownership(program_id, accounts, next_owner)
        }
        ProductInstruction::RecordRepair { metadata } => {
            msg!("Instruction: RecordRepair");
            record_repair(program_id, accounts, metadata)
        }
        ProductInstruction::MarkEndOfLife => {
            msg!("Instruction: MarkEndOfLife");
            mark_end_of_life(program_id, accounts)
        }
    }
}

fn create_product(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    product_id: String,
    metadata: String,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let product_account = next_account_info(accounts_iter)?;
    let manufacturer = next_account_info(accounts_iter)?;

    // Verify manufacturer signature
    if !manufacturer.is_signer {
        msg!("Manufacturer must sign the transaction");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let clock = Clock::get()?;
    let record = ProductRecord {
        product_id: product_id.clone(),
        transaction_type: TransactionType::Manufacture,
        previous_record: None,
        current_owner: *manufacturer.key,
        next_owner: None,
        timestamp: clock.unix_timestamp,
        metadata,
    };

    record.serialize(&mut &mut product_account.data.borrow_mut()[..])?;
    msg!("Product created: {}", product_id);

    Ok(())
}

fn transfer_ownership(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    next_owner: Pubkey,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let product_account = next_account_info(accounts_iter)?;
    let current_owner_account = next_account_info(accounts_iter)?;
    let new_record_account = next_account_info(accounts_iter)?;

    // Verify current owner signature
    if !current_owner_account.is_signer {
        msg!("Current owner must sign the transfer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut record = ProductRecord::try_from_slice(&product_account.data.borrow())?;

    // Verify ownership
    if record.current_owner != *current_owner_account.key {
        msg!("Signer is not the current owner");
        return Err(ProgramError::IllegalOwner);
    }

    let clock = Clock::get()?;
    let new_record = ProductRecord {
        product_id: record.product_id.clone(),
        transaction_type: TransactionType::Transfer,
        previous_record: Some(*product_account.key),
        current_owner: next_owner,
        next_owner: None,
        timestamp: clock.unix_timestamp,
        metadata: String::new(),
    };

    // Update old record with next owner
    record.next_owner = Some(next_owner);
    record.serialize(&mut &mut product_account.data.borrow_mut()[..])?;

    // Create new record
    new_record.serialize(&mut &mut new_record_account.data.borrow_mut()[..])?;
    msg!("Ownership transferred to: {}", next_owner);

    Ok(())
}

fn record_repair(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    metadata: String,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let product_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;
    let repair_record_account = next_account_info(accounts_iter)?;

    if !owner_account.is_signer {
        msg!("Owner must authorize repair record");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let record = ProductRecord::try_from_slice(&product_account.data.borrow())?;

    if record.current_owner != *owner_account.key {
        msg!("Signer is not the current owner");
        return Err(ProgramError::IllegalOwner);
    }

    let clock = Clock::get()?;
    let repair_record = ProductRecord {
        product_id: record.product_id.clone(),
        transaction_type: TransactionType::Repair,
        previous_record: Some(*product_account.key),
        current_owner: record.current_owner,
        next_owner: None,
        timestamp: clock.unix_timestamp,
        metadata,
    };

    repair_record.serialize(&mut &mut repair_record_account.data.borrow_mut()[..])?;
    msg!("Repair recorded for product: {}", record.product_id);

    Ok(())
}

fn mark_end_of_life(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let product_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;
    let eol_record_account = next_account_info(accounts_iter)?;

    if !owner_account.is_signer {
        msg!("Owner must authorize end-of-life marking");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let record = ProductRecord::try_from_slice(&product_account.data.borrow())?;

    if record.current_owner != *owner_account.key {
        msg!("Signer is not the current owner");
        return Err(ProgramError::IllegalOwner);
    }

    let clock = Clock::get()?;
    let eol_record = ProductRecord {
        product_id: record.product_id.clone(),
        transaction_type: TransactionType::EndOfLife,
        previous_record: Some(*product_account.key),
        current_owner: record.current_owner,
        next_owner: None,
        timestamp: clock.unix_timestamp,
        metadata: String::from("Product marked as end-of-life"),
    };

    eol_record.serialize(&mut &mut eol_record_account.data.borrow_mut()[..])?;
    msg!("Product marked as end-of-life: {}", record.product_id);

    Ok(())
}
