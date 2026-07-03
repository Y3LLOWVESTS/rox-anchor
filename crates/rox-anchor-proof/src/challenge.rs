//! RO:WHAT — Challenge posture review for ROX Anchor proof validation.
//! RO:WHY — Open or accepted challenges must block unsafe acceptance.
//! RO:INTERACTS — rox-anchor-core ChallengePosture and validate.rs.
//! RO:INVARIANTS — open/accepted challenges block acceptance; clear/rejected/expired do not.
//! RO:SECURITY — classification only; no challenge resolution authority.
//! RO:TEST — covered by challenge blocking tests.

use rox_anchor_core::ChallengePosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChallengeReview {
    Clear,
    Open,
    Accepted,
}

impl ChallengeReview {
    pub fn blocks_acceptance(self) -> bool {
        match self {
            Self::Clear => false,
            Self::Open | Self::Accepted => true,
        }
    }
}

pub fn review_challenge(posture: ChallengePosture) -> ChallengeReview {
    match posture {
        ChallengePosture::Open => ChallengeReview::Open,
        ChallengePosture::Accepted => ChallengeReview::Accepted,
        ChallengePosture::Clear | ChallengePosture::Rejected | ChallengePosture::Expired => {
            ChallengeReview::Clear
        }
    }
}
