use core::convert::TryFrom;
use mproto::{BaseLen, Compatible, Decode, DecodeCursor, DecodeError, DecodeResult, Encode, EncodeCursor, Lazy, Owned, max};

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RegisteredUser {
    pub endpoint: u64,
    pub alias: String,
}

pub struct RegisteredUserLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct RegisteredUserGen<
    Alias: Encode + Compatible<String>,
> {
    pub endpoint: u64,
    pub alias: Alias,
}

impl<
    Alias: Encode + Compatible<String>
> Compatible<RegisteredUser> for RegisteredUserGen<Alias> { }
impl<
    Alias: Encode + Compatible<String>
> Compatible<RegisteredUserGen<Alias>> for RegisteredUser { }

impl<
    Alias: Encode + Compatible<String>,
> BaseLen for RegisteredUserGen<Alias> {
    const BASE_LEN: usize = 8 + Alias::BASE_LEN;
}

impl<
    Alias: Encode + Compatible<String>,
> Encode for RegisteredUserGen<Alias> {
    fn scratch_len(&self) -> usize {
        self.endpoint.scratch_len() + self.alias.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.endpoint.encode(cursor);
        self.alias.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Owned for RegisteredUser {
    type Lazy<'a> = RegisteredUserLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for RegisteredUserLazy<'a> {
    type Owned = RegisteredUser;
}

impl<'a> Compatible<RegisteredUserLazy<'a>> for RegisteredUserLazy<'a> { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<RegisteredUserLazy<'a>> for RegisteredUser { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl Compatible<RegisteredUser> for RegisteredUser { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<RegisteredUser> for RegisteredUserLazy<'a> { }

impl<'a> RegisteredUserLazy<'a> {

    pub fn endpoint(&self) -> DecodeResult<u64> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }

    pub fn alias(&self) -> DecodeResult<&'a str> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl BaseLen for RegisteredUser {
    const BASE_LEN: usize = 16;
}

impl Encode for RegisteredUser {
    fn scratch_len(&self) -> usize {
        self.endpoint.scratch_len() + self.alias.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.endpoint.encode(cursor);
        self.alias.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Decode<'a> for RegisteredUser {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let endpoint = Decode::decode(cursor)?;
        let alias = Decode::decode(cursor)?;

        Ok(RegisteredUser {
            endpoint,
            alias,
        })
    }
}

impl<'a> BaseLen for RegisteredUserLazy<'a> {
    const BASE_LEN: usize = 16;
}

impl<'a> Encode for RegisteredUserLazy<'a> {
    fn scratch_len(&self) -> usize {
        let endpoint: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let alias: &'a str = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        endpoint.scratch_len() + alias.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let endpoint: u64 = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        let alias: &'a str = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 8)).unwrap();
        endpoint.encode(cursor);
        alias.encode(cursor);
    }
}

impl<'a> Decode<'a> for RegisteredUserLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(RegisteredUserLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> TryFrom<RegisteredUserLazy<'a>> for RegisteredUser {
    type Error = DecodeError;

    fn try_from(other: RegisteredUserLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for RegisteredUserLazy<'a> { }

impl<'a> Clone for RegisteredUserLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for RegisteredUserLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RegisteredUserLazy")
            .finish()
    }
}

impl<'a> PartialEq for RegisteredUserLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint().unwrap() == other.endpoint().unwrap()
            && self.alias().unwrap() == other.alias().unwrap()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RegisterRequest {
    pub alias: String,
}

pub struct RegisterRequestLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct RegisterRequestGen<
    Alias: Encode + Compatible<String>,
> {
    pub alias: Alias,
}

impl<
    Alias: Encode + Compatible<String>
> Compatible<RegisterRequest> for RegisterRequestGen<Alias> { }
impl<
    Alias: Encode + Compatible<String>
> Compatible<RegisterRequestGen<Alias>> for RegisterRequest { }

impl<
    Alias: Encode + Compatible<String>,
> BaseLen for RegisterRequestGen<Alias> {
    const BASE_LEN: usize = Alias::BASE_LEN;
}

impl<
    Alias: Encode + Compatible<String>,
> Encode for RegisterRequestGen<Alias> {
    fn scratch_len(&self) -> usize {
        self.alias.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.alias.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Owned for RegisterRequest {
    type Lazy<'a> = RegisterRequestLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for RegisterRequestLazy<'a> {
    type Owned = RegisterRequest;
}

impl<'a> Compatible<RegisterRequestLazy<'a>> for RegisterRequestLazy<'a> { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<RegisterRequestLazy<'a>> for RegisterRequest { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl Compatible<RegisterRequest> for RegisterRequest { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<RegisterRequest> for RegisterRequestLazy<'a> { }

impl<'a> RegisterRequestLazy<'a> {

    pub fn alias(&self) -> DecodeResult<&'a str> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl BaseLen for RegisterRequest {
    const BASE_LEN: usize = 8;
}

impl Encode for RegisterRequest {
    fn scratch_len(&self) -> usize {
        self.alias.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.alias.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Decode<'a> for RegisterRequest {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let alias = Decode::decode(cursor)?;

        Ok(RegisterRequest {
            alias,
        })
    }
}

impl<'a> BaseLen for RegisterRequestLazy<'a> {
    const BASE_LEN: usize = 8;
}

impl<'a> Encode for RegisterRequestLazy<'a> {
    fn scratch_len(&self) -> usize {
        let alias: &'a str = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        alias.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let alias: &'a str = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        alias.encode(cursor);
    }
}

impl<'a> Decode<'a> for RegisterRequestLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(RegisterRequestLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> TryFrom<RegisterRequestLazy<'a>> for RegisterRequest {
    type Error = DecodeError;

    fn try_from(other: RegisterRequestLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for RegisterRequestLazy<'a> { }

impl<'a> Clone for RegisterRequestLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for RegisterRequestLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RegisterRequestLazy")
            .finish()
    }
}

impl<'a> PartialEq for RegisterRequestLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.alias().unwrap() == other.alias().unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct RegisterSuccess {}

pub struct RegisterSuccessLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct RegisterSuccessGen<> {}

impl<> Compatible<RegisterSuccess> for RegisterSuccessGen<> { }
impl<> Compatible<RegisterSuccessGen<>> for RegisterSuccess { }

impl<> BaseLen for RegisterSuccessGen<> {
    const BASE_LEN: usize = 0;
}

impl<> Encode for RegisterSuccessGen<> {
    fn scratch_len(&self) -> usize {
        0
    }

    fn encode(&self, _: &mut EncodeCursor) {}
}

impl Owned for RegisterSuccess {
    type Lazy<'a> = RegisterSuccessLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for RegisterSuccessLazy<'a> {
    type Owned = RegisterSuccess;
}

impl<'a> Compatible<RegisterSuccessLazy<'a>> for RegisterSuccessLazy<'a> { }
impl<'a> Compatible<RegisterSuccessLazy<'a>> for RegisterSuccess { }
impl Compatible<RegisterSuccess> for RegisterSuccess { }
impl<'a> Compatible<RegisterSuccess> for RegisterSuccessLazy<'a> { }

impl<'a> RegisterSuccessLazy<'a> {}

impl BaseLen for RegisterSuccess {
    const BASE_LEN: usize = 0;
}

impl Encode for RegisterSuccess {
    fn scratch_len(&self) -> usize {
        0
    }

    fn encode(&self, _: &mut EncodeCursor) {}
}

impl<'a> Decode<'a> for RegisterSuccess {
    fn decode(_: &DecodeCursor<'a>) -> DecodeResult<Self> {

        Ok(RegisterSuccess {})
    }
}

impl<'a> BaseLen for RegisterSuccessLazy<'a> {
    const BASE_LEN: usize = 0;
}

impl<'a> Encode for RegisterSuccessLazy<'a> {
    fn scratch_len(&self) -> usize {
        0
    }

    fn encode(&self, _: &mut EncodeCursor) {}
}

impl<'a> Decode<'a> for RegisterSuccessLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(RegisterSuccessLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

impl<'a> TryFrom<RegisterSuccessLazy<'a>> for RegisterSuccess {
    type Error = DecodeError;

    fn try_from(other: RegisterSuccessLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for RegisterSuccessLazy<'a> { }

impl<'a> Clone for RegisterSuccessLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for RegisterSuccessLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RegisterSuccessLazy")
            .finish()
    }
}

impl<'a> PartialEq for RegisterSuccessLazy<'a> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum RegisterError {
    Internal,
    UserAlreadyExists,
    ChatFull,
}

#[derive(Clone)]
pub enum RegisterErrorLazy {
    Internal,
    UserAlreadyExists,
    ChatFull,
}

impl Compatible<RegisterErrorLazy> for RegisterErrorLazy { }
impl Compatible<RegisterErrorLazy> for RegisterError { }
impl Compatible<RegisterError> for RegisterErrorLazy { }
impl Compatible<RegisterError> for RegisterError { }

impl Owned for RegisterError {
    type Lazy<'a> = RegisterErrorLazy;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for RegisterErrorLazy {
    type Owned = RegisterError;
}

impl BaseLen for RegisterError {
    const BASE_LEN: usize = 1 + max(max(max(0, 0), 0), 0);
}

impl Encode for RegisterError {
    fn scratch_len(&self) -> usize {
        match self {
            RegisterError::Internal => 0,
            RegisterError::UserAlreadyExists => 0,
            RegisterError::ChatFull => 0,
        }
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        match self {
            RegisterError::Internal => {
                cursor.base(1)[0] = 0;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            RegisterError::UserAlreadyExists => {
                cursor.base(1)[0] = 1;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            RegisterError::ChatFull => {
                cursor.base(1)[0] = 2;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
        }
    }
}

impl<'a> Decode<'a> for RegisterError {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let variant = cursor.base(1)[0];
        match variant {
            0 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(RegisterError::Internal)
            }
            1 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(RegisterError::UserAlreadyExists)
            }
            2 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(RegisterError::ChatFull)
            }
            _ => { Err(DecodeError) }
        }
    }
}

impl BaseLen for RegisterErrorLazy {
    const BASE_LEN: usize = 1 + max(max(max(0, 0), 0), 0);
}

impl Encode for RegisterErrorLazy {
    fn scratch_len(&self) -> usize {
        match self {
            RegisterErrorLazy::Internal => 0,
            RegisterErrorLazy::UserAlreadyExists => 0,
            RegisterErrorLazy::ChatFull => 0,
        }
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        match self {
            RegisterErrorLazy::Internal => {
                cursor.base(1)[0] = 0;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            RegisterErrorLazy::UserAlreadyExists => {
                cursor.base(1)[0] = 1;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            RegisterErrorLazy::ChatFull => {
                cursor.base(1)[0] = 2;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
        }
    }
}

impl<'a> Decode<'a> for RegisterErrorLazy {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let variant = cursor.base(1)[0];
        match variant {
            0 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(RegisterErrorLazy::Internal)
            }
            1 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(RegisterErrorLazy::UserAlreadyExists)
            }
            2 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(RegisterErrorLazy::ChatFull)
            }
            _ => { Err(DecodeError) }
        }
    }
}

impl TryFrom<RegisterErrorLazy> for RegisterError {
    type Error = DecodeError;

    fn try_from(other: RegisterErrorLazy) -> Result<Self, Self::Error> {
        match other {
            RegisterErrorLazy::Internal => Ok(RegisterError::Internal),
            RegisterErrorLazy::UserAlreadyExists => Ok(RegisterError::UserAlreadyExists),
            RegisterErrorLazy::ChatFull => Ok(RegisterError::ChatFull),
        }
    }
}

impl Copy for RegisterErrorLazy { }

impl core::fmt::Debug for RegisterErrorLazy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RegisterErrorLazy")
            .finish()
    }
}

impl PartialEq for RegisterErrorLazy {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RegisterErrorLazy::Internal, RegisterErrorLazy::Internal) => true,
            (RegisterErrorLazy::UserAlreadyExists, RegisterErrorLazy::UserAlreadyExists) => true,
            (RegisterErrorLazy::ChatFull, RegisterErrorLazy::ChatFull) => true,
            #[allow(unreachable_patterns)]
            _ => false,
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct SendMessageRequest {
    pub content: String,
}

pub struct SendMessageRequestLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct SendMessageRequestGen<
    Content: Encode + Compatible<String>,
> {
    pub content: Content,
}

impl<
    Content: Encode + Compatible<String>
> Compatible<SendMessageRequest> for SendMessageRequestGen<Content> { }
impl<
    Content: Encode + Compatible<String>
> Compatible<SendMessageRequestGen<Content>> for SendMessageRequest { }

impl<
    Content: Encode + Compatible<String>,
> BaseLen for SendMessageRequestGen<Content> {
    const BASE_LEN: usize = Content::BASE_LEN;
}

impl<
    Content: Encode + Compatible<String>,
> Encode for SendMessageRequestGen<Content> {
    fn scratch_len(&self) -> usize {
        self.content.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.content.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Owned for SendMessageRequest {
    type Lazy<'a> = SendMessageRequestLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for SendMessageRequestLazy<'a> {
    type Owned = SendMessageRequest;
}

impl<'a> Compatible<SendMessageRequestLazy<'a>> for SendMessageRequestLazy<'a> { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<SendMessageRequestLazy<'a>> for SendMessageRequest { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl Compatible<SendMessageRequest> for SendMessageRequest { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<SendMessageRequest> for SendMessageRequestLazy<'a> { }

impl<'a> SendMessageRequestLazy<'a> {

    pub fn content(&self) -> DecodeResult<&'a str> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl BaseLen for SendMessageRequest {
    const BASE_LEN: usize = 8;
}

impl Encode for SendMessageRequest {
    fn scratch_len(&self) -> usize {
        self.content.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.content.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Decode<'a> for SendMessageRequest {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let content = Decode::decode(cursor)?;

        Ok(SendMessageRequest {
            content,
        })
    }
}

impl<'a> BaseLen for SendMessageRequestLazy<'a> {
    const BASE_LEN: usize = 8;
}

impl<'a> Encode for SendMessageRequestLazy<'a> {
    fn scratch_len(&self) -> usize {
        let content: &'a str = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        content.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let content: &'a str = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        content.encode(cursor);
    }
}

impl<'a> Decode<'a> for SendMessageRequestLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(SendMessageRequestLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> TryFrom<SendMessageRequestLazy<'a>> for SendMessageRequest {
    type Error = DecodeError;

    fn try_from(other: SendMessageRequestLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for SendMessageRequestLazy<'a> { }

impl<'a> Clone for SendMessageRequestLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for SendMessageRequestLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SendMessageRequestLazy")
            .finish()
    }
}

impl<'a> PartialEq for SendMessageRequestLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.content().unwrap() == other.content().unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct SendMessageSuccess {}

pub struct SendMessageSuccessLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct SendMessageSuccessGen<> {}

impl<> Compatible<SendMessageSuccess> for SendMessageSuccessGen<> { }
impl<> Compatible<SendMessageSuccessGen<>> for SendMessageSuccess { }

impl<> BaseLen for SendMessageSuccessGen<> {
    const BASE_LEN: usize = 0;
}

impl<> Encode for SendMessageSuccessGen<> {
    fn scratch_len(&self) -> usize {
        0
    }

    fn encode(&self, _: &mut EncodeCursor) {}
}

impl Owned for SendMessageSuccess {
    type Lazy<'a> = SendMessageSuccessLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for SendMessageSuccessLazy<'a> {
    type Owned = SendMessageSuccess;
}

impl<'a> Compatible<SendMessageSuccessLazy<'a>> for SendMessageSuccessLazy<'a> { }
impl<'a> Compatible<SendMessageSuccessLazy<'a>> for SendMessageSuccess { }
impl Compatible<SendMessageSuccess> for SendMessageSuccess { }
impl<'a> Compatible<SendMessageSuccess> for SendMessageSuccessLazy<'a> { }

impl<'a> SendMessageSuccessLazy<'a> {}

impl BaseLen for SendMessageSuccess {
    const BASE_LEN: usize = 0;
}

impl Encode for SendMessageSuccess {
    fn scratch_len(&self) -> usize {
        0
    }

    fn encode(&self, _: &mut EncodeCursor) {}
}

impl<'a> Decode<'a> for SendMessageSuccess {
    fn decode(_: &DecodeCursor<'a>) -> DecodeResult<Self> {

        Ok(SendMessageSuccess {})
    }
}

impl<'a> BaseLen for SendMessageSuccessLazy<'a> {
    const BASE_LEN: usize = 0;
}

impl<'a> Encode for SendMessageSuccessLazy<'a> {
    fn scratch_len(&self) -> usize {
        0
    }

    fn encode(&self, _: &mut EncodeCursor) {}
}

impl<'a> Decode<'a> for SendMessageSuccessLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(SendMessageSuccessLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

impl<'a> TryFrom<SendMessageSuccessLazy<'a>> for SendMessageSuccess {
    type Error = DecodeError;

    fn try_from(other: SendMessageSuccessLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for SendMessageSuccessLazy<'a> { }

impl<'a> Clone for SendMessageSuccessLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for SendMessageSuccessLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SendMessageSuccessLazy")
            .finish()
    }
}

impl<'a> PartialEq for SendMessageSuccessLazy<'a> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SendMessageError {
    Internal,
    NotRegistered,
    MessageTooLong,
}

#[derive(Clone)]
pub enum SendMessageErrorLazy {
    Internal,
    NotRegistered,
    MessageTooLong,
}

impl Compatible<SendMessageErrorLazy> for SendMessageErrorLazy { }
impl Compatible<SendMessageErrorLazy> for SendMessageError { }
impl Compatible<SendMessageError> for SendMessageErrorLazy { }
impl Compatible<SendMessageError> for SendMessageError { }

impl Owned for SendMessageError {
    type Lazy<'a> = SendMessageErrorLazy;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for SendMessageErrorLazy {
    type Owned = SendMessageError;
}

impl BaseLen for SendMessageError {
    const BASE_LEN: usize = 1 + max(max(max(0, 0), 0), 0);
}

impl Encode for SendMessageError {
    fn scratch_len(&self) -> usize {
        match self {
            SendMessageError::Internal => 0,
            SendMessageError::NotRegistered => 0,
            SendMessageError::MessageTooLong => 0,
        }
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        match self {
            SendMessageError::Internal => {
                cursor.base(1)[0] = 0;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            SendMessageError::NotRegistered => {
                cursor.base(1)[0] = 1;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            SendMessageError::MessageTooLong => {
                cursor.base(1)[0] = 2;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
        }
    }
}

impl<'a> Decode<'a> for SendMessageError {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let variant = cursor.base(1)[0];
        match variant {
            0 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(SendMessageError::Internal)
            }
            1 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(SendMessageError::NotRegistered)
            }
            2 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(SendMessageError::MessageTooLong)
            }
            _ => { Err(DecodeError) }
        }
    }
}

impl BaseLen for SendMessageErrorLazy {
    const BASE_LEN: usize = 1 + max(max(max(0, 0), 0), 0);
}

impl Encode for SendMessageErrorLazy {
    fn scratch_len(&self) -> usize {
        match self {
            SendMessageErrorLazy::Internal => 0,
            SendMessageErrorLazy::NotRegistered => 0,
            SendMessageErrorLazy::MessageTooLong => 0,
        }
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        match self {
            SendMessageErrorLazy::Internal => {
                cursor.base(1)[0] = 0;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            SendMessageErrorLazy::NotRegistered => {
                cursor.base(1)[0] = 1;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
            SendMessageErrorLazy::MessageTooLong => {
                cursor.base(1)[0] = 2;
                cursor.base(Self::BASE_LEN - 1).fill(0);
            }
        }
    }
}

impl<'a> Decode<'a> for SendMessageErrorLazy {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let variant = cursor.base(1)[0];
        match variant {
            0 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(SendMessageErrorLazy::Internal)
            }
            1 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(SendMessageErrorLazy::NotRegistered)
            }
            2 => {
                cursor.advance(Self::BASE_LEN - 1);
                Ok(SendMessageErrorLazy::MessageTooLong)
            }
            _ => { Err(DecodeError) }
        }
    }
}

impl TryFrom<SendMessageErrorLazy> for SendMessageError {
    type Error = DecodeError;

    fn try_from(other: SendMessageErrorLazy) -> Result<Self, Self::Error> {
        match other {
            SendMessageErrorLazy::Internal => Ok(SendMessageError::Internal),
            SendMessageErrorLazy::NotRegistered => Ok(SendMessageError::NotRegistered),
            SendMessageErrorLazy::MessageTooLong => Ok(SendMessageError::MessageTooLong),
        }
    }
}

impl Copy for SendMessageErrorLazy { }

impl core::fmt::Debug for SendMessageErrorLazy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SendMessageErrorLazy")
            .finish()
    }
}

impl PartialEq for SendMessageErrorLazy {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SendMessageErrorLazy::Internal, SendMessageErrorLazy::Internal) => true,
            (SendMessageErrorLazy::NotRegistered, SendMessageErrorLazy::NotRegistered) => true,
            (SendMessageErrorLazy::MessageTooLong, SendMessageErrorLazy::MessageTooLong) => true,
            #[allow(unreachable_patterns)]
            _ => false,
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ChatInitState {
    pub users: Vec<RegisteredUser>,
}

pub struct ChatInitStateLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct ChatInitStateGen<
    Users: Encode + Compatible<Vec<RegisteredUser>>,
> {
    pub users: Users,
}

impl<
    Users: Encode + Compatible<Vec<RegisteredUser>>
> Compatible<ChatInitState> for ChatInitStateGen<Users> { }
impl<
    Users: Encode + Compatible<Vec<RegisteredUser>>
> Compatible<ChatInitStateGen<Users>> for ChatInitState { }

impl<
    Users: Encode + Compatible<Vec<RegisteredUser>>,
> BaseLen for ChatInitStateGen<Users> {
    const BASE_LEN: usize = Users::BASE_LEN;
}

impl<
    Users: Encode + Compatible<Vec<RegisteredUser>>,
> Encode for ChatInitStateGen<Users> {
    fn scratch_len(&self) -> usize {
        self.users.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.users.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Owned for ChatInitState {
    type Lazy<'a> = ChatInitStateLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for ChatInitStateLazy<'a> {
    type Owned = ChatInitState;
}

impl<'a> Compatible<ChatInitStateLazy<'a>> for ChatInitStateLazy<'a> { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<ChatInitStateLazy<'a>> for ChatInitState { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl Compatible<ChatInitState> for ChatInitState { }
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Compatible<ChatInitState> for ChatInitStateLazy<'a> { }

impl<'a> ChatInitStateLazy<'a> {

    pub fn users(&self) -> DecodeResult<mproto::ListLazy<'a, RegisteredUser>> {
        Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl BaseLen for ChatInitState {
    const BASE_LEN: usize = 8;
}

impl Encode for ChatInitState {
    fn scratch_len(&self) -> usize {
        self.users.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        self.users.encode(cursor);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Decode<'a> for ChatInitState {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let users = Decode::decode(cursor)?;

        Ok(ChatInitState {
            users,
        })
    }
}

impl<'a> BaseLen for ChatInitStateLazy<'a> {
    const BASE_LEN: usize = 8;
}

impl<'a> Encode for ChatInitStateLazy<'a> {
    fn scratch_len(&self) -> usize {
        let users: mproto::ListLazy<'a, RegisteredUser> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        users.scratch_len()
    }

    fn encode(&self, cursor: &mut EncodeCursor) {
        let users: mproto::ListLazy<'a, RegisteredUser> = Decode::decode(&DecodeCursor::at_offset(self.buffer, self.offset + 0)).unwrap();
        users.encode(cursor);
    }
}

impl<'a> Decode<'a> for ChatInitStateLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(ChatInitStateLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> TryFrom<ChatInitStateLazy<'a>> for ChatInitState {
    type Error = DecodeError;

    fn try_from(other: ChatInitStateLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for ChatInitStateLazy<'a> { }

impl<'a> Clone for ChatInitStateLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for ChatInitStateLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ChatInitStateLazy")
            .finish()
    }
}

impl<'a> PartialEq for ChatInitStateLazy<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.users().unwrap() == other.users().unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct ChatClientConfig {}

pub struct ChatClientConfigLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct ChatClientConfigGen<> {}

impl<> Compatible<ChatClientConfig> for ChatClientConfigGen<> { }
impl<> Compatible<ChatClientConfigGen<>> for ChatClientConfig { }

impl<> BaseLen for ChatClientConfigGen<> {
    const BASE_LEN: usize = 0;
}

impl<> Encode for ChatClientConfigGen<> {
    fn scratch_len(&self) -> usize {
        0
    }

    fn encode(&self, _: &mut EncodeCursor) {}
}

impl Owned for ChatClientConfig {
    type Lazy<'a> = ChatClientConfigLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for ChatClientConfigLazy<'a> {
    type Owned = ChatClientConfig;
}

impl<'a> Compatible<ChatClientConfigLazy<'a>> for ChatClientConfigLazy<'a> { }
impl<'a> Compatible<ChatClientConfigLazy<'a>> for ChatClientConfig { }
impl Compatible<ChatClientConfig> for ChatClientConfig { }
impl<'a> Compatible<ChatClientConfig> for ChatClientConfigLazy<'a> { }

impl<'a> ChatClientConfigLazy<'a> {}

impl BaseLen for ChatClientConfig {
    const BASE_LEN: usize = 0;
}

impl Encode for ChatClientConfig {
    fn scratch_len(&self) -> usize {
        0
    }

    fn encode(&self, _: &mut EncodeCursor) {}
}

impl<'a> Decode<'a> for ChatClientConfig {
    fn decode(_: &DecodeCursor<'a>) -> DecodeResult<Self> {

        Ok(ChatClientConfig {})
    }
}

impl<'a> BaseLen for ChatClientConfigLazy<'a> {
    const BASE_LEN: usize = 0;
}

impl<'a> Encode for ChatClientConfigLazy<'a> {
    fn scratch_len(&self) -> usize {
        0
    }

    fn encode(&self, _: &mut EncodeCursor) {}
}

impl<'a> Decode<'a> for ChatClientConfigLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(ChatClientConfigLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

impl<'a> TryFrom<ChatClientConfigLazy<'a>> for ChatClientConfig {
    type Error = DecodeError;

    fn try_from(other: ChatClientConfigLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for ChatClientConfigLazy<'a> { }

impl<'a> Clone for ChatClientConfigLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for ChatClientConfigLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ChatClientConfigLazy")
            .finish()
    }
}

impl<'a> PartialEq for ChatClientConfigLazy<'a> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct ChatServerConfig {}

pub struct ChatServerConfigLazy<'a> {
    buffer: &'a [u8],
    offset: usize,
}

pub struct ChatServerConfigGen<> {}

impl<> Compatible<ChatServerConfig> for ChatServerConfigGen<> { }
impl<> Compatible<ChatServerConfigGen<>> for ChatServerConfig { }

impl<> BaseLen for ChatServerConfigGen<> {
    const BASE_LEN: usize = 0;
}

impl<> Encode for ChatServerConfigGen<> {
    fn scratch_len(&self) -> usize {
        0
    }

    fn encode(&self, _: &mut EncodeCursor) {}
}

impl Owned for ChatServerConfig {
    type Lazy<'a> = ChatServerConfigLazy<'a>;

    fn lazy_to_owned(lazy: Self::Lazy<'_>) -> DecodeResult<Self> {
        TryFrom::try_from(lazy)
    }
}

impl<'a> Lazy<'a> for ChatServerConfigLazy<'a> {
    type Owned = ChatServerConfig;
}

impl<'a> Compatible<ChatServerConfigLazy<'a>> for ChatServerConfigLazy<'a> { }
impl<'a> Compatible<ChatServerConfigLazy<'a>> for ChatServerConfig { }
impl Compatible<ChatServerConfig> for ChatServerConfig { }
impl<'a> Compatible<ChatServerConfig> for ChatServerConfigLazy<'a> { }

impl<'a> ChatServerConfigLazy<'a> {}

impl BaseLen for ChatServerConfig {
    const BASE_LEN: usize = 0;
}

impl Encode for ChatServerConfig {
    fn scratch_len(&self) -> usize {
        0
    }

    fn encode(&self, _: &mut EncodeCursor) {}
}

impl<'a> Decode<'a> for ChatServerConfig {
    fn decode(_: &DecodeCursor<'a>) -> DecodeResult<Self> {

        Ok(ChatServerConfig {})
    }
}

impl<'a> BaseLen for ChatServerConfigLazy<'a> {
    const BASE_LEN: usize = 0;
}

impl<'a> Encode for ChatServerConfigLazy<'a> {
    fn scratch_len(&self) -> usize {
        0
    }

    fn encode(&self, _: &mut EncodeCursor) {}
}

impl<'a> Decode<'a> for ChatServerConfigLazy<'a> {
    fn decode(cursor: &DecodeCursor<'a>) -> DecodeResult<Self> {
        let offset = cursor.offset();
        cursor.advance(Self::BASE_LEN);
        Ok(ChatServerConfigLazy {
            buffer: cursor.buffer(),
            offset,
        })
    }
}

impl<'a> TryFrom<ChatServerConfigLazy<'a>> for ChatServerConfig {
    type Error = DecodeError;

    fn try_from(other: ChatServerConfigLazy<'a>) -> Result<Self, Self::Error> {
        let cursor = DecodeCursor::at_offset(other.buffer, other.offset);
        Decode::decode(&cursor)
    }
}

impl<'a> Copy for ChatServerConfigLazy<'a> { }

impl<'a> Clone for ChatServerConfigLazy<'a> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer,
            offset: self.offset,
        }
    }
}

impl<'a> core::fmt::Debug for ChatServerConfigLazy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ChatServerConfigLazy")
            .finish()
    }
}

impl<'a> PartialEq for ChatServerConfigLazy<'a> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
