#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultCode {
    Success = 0,
    OperationsError = 1,
    ProtocolError = 2,
    TimeLimitExceeded = 3,
    SizeLimitExceeded = 4,
    CompareFalse = 5,
    CompareTrue = 6,
    AuthMethodNotSupported = 7,
    StrongerAuthRequired = 8,
    NoSuchAttribute = 16,
    UndefinedAttributeType = 17,
    InappropriateMatching = 18,
    ConstraintViolation = 19,
    AttributeOrValueExists = 20,
    InvalidAttributeSyntax = 21,
    NoSuchObject = 32,
    AliasProblem = 33,
    InvalidDNSyntax = 34,
    AliasDereferencingProblem = 36,
    InappropriateAuthentication = 48,
    InvalidCredentials = 49,
    InsufficientAccessRights = 50,
    Busy = 51,
    Unavailable = 52,
    UnwillingToPerform = 53,
    LoopDetect = 54,
    NamingViolation = 64,
    ObjectClassViolation = 65,
    NotAllowedOnNonLeaf = 66,
    NotAllowedOnRDN = 67,
    EntryAlreadyExists = 68,
    ObjectClassModsProhibited = 69,
    AffectsMultipleDSAs = 71,
    Other = 80,
}

#[derive(Debug, Clone)]
pub struct LdapResult {
    pub code: ResultCode,
    pub matched_dn: String,
    pub diagnostic_message: String,
}

impl LdapResult {
    pub fn success() -> Self {
        Self {
            code: ResultCode::Success,
            matched_dn: String::new(),
            diagnostic_message: String::new(),
        }
    }
}
