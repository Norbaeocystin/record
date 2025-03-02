use {pinocchio::program_error::ProgramError, std::mem::size_of};

/// Instructions supported by the program
#[derive(Clone, Debug, PartialEq)]
pub enum RecordInstruction<'a> {
    Initialize,
    Write { offset: u64, data: &'a [u8]},
    SetAuthority,
    CloseAccount,
}

impl<'a> RecordInstruction<'a> {
    pub fn unpack(input: &'a [u8]) -> Result<Self, ProgramError> {
        const U32_BYTES: usize = 4;
        const U64_BYTES: usize = 8;

        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => Self::Initialize,
            1 => {
                let offset = rest
                    .get(..U64_BYTES)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                let (length, data) = rest[U64_BYTES..].split_at(U32_BYTES);
                let length = u32::from_le_bytes(
                    length
                        .try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                ) as usize;

                Self::Write {
                    offset,
                    data: &data[..length],
                }
            }
            2 => Self::SetAuthority,
            3 => Self::CloseAccount,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    /// Packs a [`RecordInstruction`] into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            Self::Initialize => buf.push(0),
            Self::Write { offset, data } => {
                buf.push(1);
                buf.extend_from_slice(&offset.to_le_bytes());
                buf.extend_from_slice(&(data.len() as u32).to_le_bytes());
                buf.extend_from_slice(data);
            }
            Self::SetAuthority => buf.push(2),
            Self::CloseAccount => buf.push(3),
        };
        buf
    }
}
