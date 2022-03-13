use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;


declare_id!("56Ye6uGJgGvk7KhWi12Z38aRMkeBEsxeAsVW9DDeXfnL");

#[program]
pub mod solinvoice {
    use super::*;
    pub fn send_invoice(ctx: Context<SendInvoice>, pref: String, items: String, quantities: String, buyer: String) -> Result<()> {
        let invoice: &mut Account<Invoice> = &mut ctx.accounts.invoice;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        invoice.author = *author.key;
        invoice.paymentref = pref;
        invoice.timestamp = clock.unix_timestamp;
        invoice.items = items;
        invoice.quantities = quantities;
        invoice.addressee = buyer;

        Ok(())
    }

    pub fn send_billable(ctx: Context<SendBillable>, key: String, desc: String, price:String) -> Result<()> {
        let billable: &mut Account<Billable> = &mut ctx.accounts.billable;
        let author: &Signer = &ctx.accounts.author;

        if desc.chars().count() > 60 {
            return Err(ErrorCode::DescTooLong.into())
        }

        billable.author = *author.key;
        billable.description = desc;
        billable.key = key.parse::<i16>().unwrap();
        billable.price = price.parse::<f32>().unwrap();

        Ok(())
    }

    pub fn send_settings(ctx: Context<SendSettings>, n: String, addr: String, b_currency: String) -> Result<()> {
        let info: &mut Account<Settings> = &mut ctx.accounts.settings;
        let author: &Signer = &ctx.accounts.author;

        if n.chars().count() > 60 {
            return Err(ErrorCode::DescTooLong.into())
        }

        info.author = *author.key;
        info.name = n;
        info.address = addr;
        info.base_currency = b_currency;

        Ok(())
    }

    pub fn update_invoice(ctx: Context<UpdateInvoice>, desc: String, date: String, items: String, quantities: String, buyer: String) -> Result<()> {
        let invoice: &mut Account<Invoice> = &mut ctx.accounts.invoice;

        if desc.chars().count() > 32 {
            return Err(ErrorCode::DescTooLong.into())
        }

        invoice.timestamp = date.parse::<i64>().unwrap();
        invoice.items = items;
        invoice.quantities = quantities;
        invoice.addressee = buyer;
        Ok(())
    }

    pub fn update_billable(ctx: Context<UpdateBillable>, desc: String, price:String) -> Result<()> {
        let desciptor: &mut Account<Billable> = &mut ctx.accounts.billable;

        if desc.chars().count() > 60 {
            return Err(ErrorCode::DescTooLong.into())
        }

        desciptor.description = desc;
        desciptor.price = price.parse::<f32>().unwrap();
        
        Ok(())
    }

    pub fn update_settings(ctx: Context<UpdateSettings>, n: String, addr: String, b_currency: String) -> Result<()> {
        let info: &mut Account<Settings> = &mut ctx.accounts.settings;

        if n.chars().count() > 60 {
            return Err(ErrorCode::DescTooLong.into())
        }
        

        info.name = n;
        info.address = addr;
        info.base_currency = b_currency;
        
        Ok(())
    }

    pub fn delete_invoice(_ctx: Context<DeleteInvoice>) -> Result<()> {
        Ok(())
    }

    pub fn delete_billable(_ctx: Context<DeleteBillable>) -> Result<()> {
        Ok(())
    }

    pub fn delete_settings(_ctx: Context<DeleteSettings>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendInvoice<'info> {
    #[account(init, payer = author, space = Invoice::LEN)]
    pub invoice: Account<'info, Invoice>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateInvoice<'info> {
    #[account(mut, has_one = author)]
    pub invoice: Account<'info, Invoice>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteInvoice<'info> {
    #[account(mut, has_one = author, close = author)]
    pub invoice: Account<'info, Invoice>,
    pub author: Signer<'info>,
}

#[account]
pub struct Invoice {
    pub author: Pubkey,
    pub timestamp: i64,
    pub paymentref: String,
    pub items: String,
    pub quantities: String,
    pub addressee: String,
}

#[derive(Accounts)]
pub struct SendBillable<'info> {
    #[account(init, payer = author, space = Billable::LEN)]
    pub billable: Account<'info, Billable>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct UpdateBillable<'info> {
    #[account(mut, has_one = author)]
    pub billable: Account<'info, Billable>,
    pub author: Signer<'info>,
}
#[derive(Accounts)]
pub struct DeleteBillable<'info> {
    #[account(mut, has_one = author, close = author)]
    pub billable: Account<'info, Billable>,
    pub author: Signer<'info>,
}
#[account]
pub struct Billable {
    pub author: Pubkey,
    pub key: i16,
    pub description: String,
    pub price: f32,
}

#[derive(Accounts)]
pub struct SendSettings<'info> {
    #[account(init, payer = author, space = Settings::LEN)]
    pub settings: Account<'info, Settings>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct UpdateSettings<'info> {
    #[account(mut, has_one = author)]
    pub settings: Account<'info, Settings>,
    pub author: Signer<'info>,
}
#[derive(Accounts)]
pub struct DeleteSettings<'info> {
    #[account(mut, has_one = author, close = author)]
    pub settings: Account<'info, Settings>,
    pub author: Signer<'info>,
}
#[account]
pub struct Settings {
    pub author: Pubkey,
    pub name: String,
    pub address: String,
    pub base_currency: String,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_DESC_LENGTH: usize = 32 * 4; // 32 chars max.
const DEBIT_ACC_LENGTH: usize = 2;
const DEBIT_ACC_VAL_LENGTH: usize = 4;

const ACC_DESC_LENGTH: usize = 60 * 4;
const ACC_KEY: usize = 2;

impl Invoice {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_DESC_LENGTH // Pubkey
        + STRING_LENGTH_PREFIX + MAX_DESC_LENGTH // Addressee
        + STRING_LENGTH_PREFIX + ACC_DESC_LENGTH // Items
        + STRING_LENGTH_PREFIX + ACC_DESC_LENGTH; // Quantities
        
}

impl Billable {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + ACC_KEY // Key.
        + STRING_LENGTH_PREFIX + ACC_DESC_LENGTH // Description.
        + STRING_LENGTH_PREFIX + DEBIT_ACC_VAL_LENGTH; // Price
}

impl Settings {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + ACC_KEY // Key.
        + STRING_LENGTH_PREFIX + MAX_DESC_LENGTH // Merchant Name.
        + STRING_LENGTH_PREFIX + MAX_DESC_LENGTH // Address.
        + STRING_LENGTH_PREFIX + MAX_DESC_LENGTH; // Currency.
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided desc should be 32/60 characters long maximum.")]
    DescTooLong,
}
