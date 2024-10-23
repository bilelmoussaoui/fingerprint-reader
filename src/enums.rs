use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScanType {
    Press,
    Swipe,
}

impl FromStr for ScanType {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "press" => Ok(Self::Press),
            "swipe" => Ok(Self::Swipe),
            _ => Err(Error::InvalidScanType(s.to_owned())),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fingerprint {
    /// Left thumb
    LeftThumb,
    /// Left index finger
    LeftIndexFinger,
    /// Left middle finger
    LeftMiddleFinger,
    /// Left ring finger
    LeftRingFinger,
    /// Left little finger
    LeftLittleFinger,
    /// Right thumb
    RightThumb,
    /// Right index finger
    RightIndexFinger,
    /// Right middle finger
    RightMiddleFinger,
    /// Right ring finger
    RightRingFinger,
    /// Right little finger
    RightLittleFinger,
    /// Any finger
    Any,
}

impl FromStr for Fingerprint {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "left-thumb" => Ok(Self::LeftThumb),
            "left-index-finger" => Ok(Self::LeftIndexFinger),
            "left-middle-finger" => Ok(Self::LeftMiddleFinger),
            "left-ring-finger" => Ok(Self::LeftRingFinger),
            "left-little-finger" => Ok(Self::LeftLittleFinger),
            "right-thumb" => Ok(Self::RightThumb),
            "right-index-finger" => Ok(Self::RightIndexFinger),
            "right-middle-finger" => Ok(Self::RightMiddleFinger),
            "right-ring-finger" => Ok(Self::RightRingFinger),
            "right-little-finger" => Ok(Self::RightLittleFinger),
            "any" => Ok(Self::Any),
            _ => Err(Error::InvalidFingerprint(s.to_owned())),
        }
    }
}

impl Fingerprint {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::LeftThumb => "left-thumb",
            Self::LeftIndexFinger => "left-index-finger",
            Self::LeftMiddleFinger => "left-middle-finger",
            Self::LeftRingFinger => "left-ring-finger",
            Self::LeftLittleFinger => "left-little-finger",
            Self::RightThumb => "right-thumb",
            Self::RightIndexFinger => "right-index-finger",
            Self::RightMiddleFinger => "right-middle-finger",
            Self::RightRingFinger => "right-ring-finger",
            Self::RightLittleFinger => "right-little-finger",
            Self::Any => "any",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VerifyStatus {
    NoMatch,
    Match,
    RetryScan,
    SwipeTooShort,
    FingerNotCentered,
    RemoveAndRetry,
    Disconnected,
    Unknown,
}

impl FromStr for VerifyStatus {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "verify-no-match" => Ok(Self::NoMatch),
            "verify-match" => Ok(Self::Match),
            "verify-retry-scan" => Ok(Self::RetryScan),
            "verify-swipe-too-short" => Ok(Self::SwipeTooShort),
            "verify-finger-not-centered" => Ok(Self::FingerNotCentered),
            "verify-remove-and-retry" => Ok(Self::RemoveAndRetry),
            "verify-disconnected" => Ok(Self::Disconnected),
            "verify-unknown-error" => Ok(Self::Unknown),
            _ => Err(Error::InvalidVerifyStatus(s.to_owned())),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EnrollStatus {
    Completed,
    Failed,
    StagePassed,
    RetryScan,
    SwipeTooShort,
    FingerNotCentered,
    RemoveAndRetry,
    DataFull,
    Duplicate,
    Disconnected,
    Unknown,
}

impl FromStr for EnrollStatus {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "enroll-completed" => Ok(Self::Completed),
            "enroll-failed" => Ok(Self::Failed),
            "enroll-stage-passed" => Ok(Self::StagePassed),
            "enroll-retry-scan" => Ok(Self::RetryScan),
            "enroll-swipe-too-short" => Ok(Self::SwipeTooShort),
            "enroll-finger-not-centered" => Ok(Self::FingerNotCentered),
            "enroll-remove-and-retry" => Ok(Self::RemoveAndRetry),
            "enroll-data-full" => Ok(Self::DataFull),
            "enroll-duplicate" => Ok(Self::Duplicate),
            "enroll-disconnected" => Ok(Self::Disconnected),
            "enroll-unknown-error" => Ok(Self::Unknown),
            _ => Err(Error::InvalidEnrollStatus(s.to_owned())),
        }
    }
}
