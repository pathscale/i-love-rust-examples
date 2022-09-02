use reqwest::StatusCode;
use serde::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ErrorCode {
    code: u32,
}

impl ErrorCode {
    pub fn new(code: u32) -> Self {
        Self { code }
    }

    pub fn to_status_code(self) -> Option<StatusCode> {
        StatusCode::from_u16(self.code as u16).ok()
    }
    pub fn to_u32(self) -> u32 {
        self.code
    }
    pub fn canonical_reason(self) -> Option<&'static str> {
        if let Some(x) = self.to_status_code() {
            x.canonical_reason()
        } else {
            match self.code {
                3484946 => Some("InvalidEnumLevel"), // 22P02

                45349632 => Some("Error"),                    // R0000
                45349633 => Some("InvalidArgument"),          // R0001
                45349634 => Some("InvalidState"),             // R0002
                45349635 => Some("InvalidSeq"),               // R0003
                45349636 => Some("InvalidMethod"),            // R0004
                45349637 => Some("ProtocolViolation"),        // R0005
                45349638 => Some("MalformedRequest"),         // R0006
                45349639 => Some("UnknownUser"),              // R0007
                45349640 => Some("BlockedUser"),              // R0008
                45349641 => Some("InvalidPassword"),          // R0009
                45349642 => Some("InvalidToken"),             // R000A
                45349643 => Some("TemporarilyUnavailable"),   // R000B
                45349644 => Some("UnexpectedException"),      // R000C
                45349645 => Some("BackPressureIncreased"),    // R000D
                45349646 => Some("InvalidPublicId"),          // R000E
                45349647 => Some("InvalidRange"),             // R000F
                45349648 => Some("BankAccountAlreadyExists"), // R000G
                45349649 => Some("InsufficientFunds"),        // R000H

                45349654 => Some("LogicalError"), // R000M
                45349655 => Some("RestrictedUserPrivileges"), // R000N
                45349656 => Some("IdenticalReplacement"), // R000O

                45349659 => Some("InvalidRecoveryQuestions"), // R000R
                45349660 => Some("InvalidRole"),              // R000S
                45349661 => Some("WrongRecoveryAnswers"),     // R000T
                45349662 => Some("MessageNotDelivered"),      // R000U
                45349663 => Some("NoReply"),                  // R000V
                45349664 => Some("NullAttribute"),            // R000W
                45349665 => Some("ConsentMissing"),           // R000X
                45349666 => Some("ActiveSubscriptionRequired"), // R000Y
                45349667 => Some("UsernameAlreadyRegistered"), // R000Z
                45349668 => Some("RecoveryQuestionsNotSet"),  // R0010
                45349669 => Some("MustSubmitAllRecoveryQuestions"), // R0011
                45349670 => Some("InvalidRecoveryToken"),     // R0012

                45349676 => Some("RoutingError"),        // R0018
                45349677 => Some("UnauthorizedMessage"), // R0019

                45349679 => Some("AuthError"), // R001B

                45349684 => Some("InternalError"), // R001G
                _ => None,
            }
        }
    }
}

impl From<StatusCode> for ErrorCode {
    fn from(status: StatusCode) -> Self {
        Self::new(status.as_u16() as _)
    }
}
