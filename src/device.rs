use std::str::FromStr;

use crate::error::Result;
use futures_util::{Stream, StreamExt};
use zbus::zvariant::ObjectPath;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScanType {
    Press,
    Swipe,
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
    type Err = String;

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
            _ => Err(format!("Invalid fingerprint name {s}")),
        }
    }
}

impl Fingerprint {
    fn as_str(&self) -> &'static str {
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
    type Err = String;

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
            _ => Err(format!("Invalid VerifyStatus {s}")),
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
    type Err = String;

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
            _ => Err(format!("Invalid EnrollStatus {s}")),
        }
    }
}

#[doc(alias = "net.reactivated.Fprint.Device")]
pub struct Device<'a>(zbus::Proxy<'a>);

impl<'a> Device<'a> {
    pub(crate) async fn new<P>(cnx: &zbus::Connection, path: P) -> Result<Self>
    where
        P: TryInto<ObjectPath<'a>>,
        P::Error: Into<zbus::Error>,
    {
        let proxy = zbus::proxy::Builder::new(cnx)
            .interface("net.reactivated.Fprint.Device")?
            .path(path)?
            .destination("net.reactivated.Fprint")?
            .build()
            .await?;
        Ok(Self(proxy))
    }

    pub async fn name(&self) -> Result<String> {
        self.0.get_property("name").await.map_err(From::from)
    }

    /// If the device has not been claimed yet, it returns None
    pub async fn num_enroll_stages(&self) -> Result<Option<i32>> {
        let stages = self.0.get_property::<i32>("num-enroll-stages").await?;
        if stages == -1 {
            Ok(None)
        } else {
            Ok(Some(stages))
        }
    }

    pub async fn scan_type(&self) -> Result<ScanType> {
        let ty = self.0.get_property::<String>("scan-type").await?;
        if ty == "press" {
            Ok(ScanType::Press)
        } else {
            Ok(ScanType::Swipe)
        }
    }

    pub async fn finger_present(&self) -> Result<bool> {
        self.0
            .get_property("finger-present")
            .await
            .map_err(From::from)
    }

    pub async fn finger_needed(&self) -> Result<bool> {
        self.0
            .get_property("finger-needed")
            .await
            .map_err(From::from)
    }

    #[doc(alias = "ListEnrolledFingers")]
    pub async fn list_enrolled_fingers(&self, username: Option<&str>) -> Result<Vec<Fingerprint>> {
        let names = self
            .0
            .call_method("ListEnrolledFingers", &(username.unwrap_or_default(),))
            .await?
            .body()
            .deserialize::<Vec<String>>()?;
        let mut fingers = Vec::with_capacity(names.capacity());
        for name in names {
            fingers.push(Fingerprint::from_str(&name).unwrap());
        }
        Ok(fingers)
    }

    pub async fn claim(&self, username: Option<&str>) -> Result<()> {
        self.0
            .call_method("Claim", &(username.unwrap_or_default(),))
            .await?;
        Ok(())
    }

    pub async fn release(&self) -> Result<()> {
        self.0.call_method("Release", &()).await?;
        Ok(())
    }

    pub async fn verify_start(&self, finger: Fingerprint) -> Result<()> {
        self.0
            .call_method("VerifyStart", &(finger.as_str(),))
            .await?;
        Ok(())
    }

    pub async fn verify_stop(&self) -> Result<()> {
        self.0.call_method("VerifyStop", &()).await?;
        Ok(())
    }

    #[doc(alias = "VerifyFingerSelected")]
    pub async fn receive_verify_finger_selected(
        &self,
    ) -> Result<impl Stream<Item = Fingerprint> + '_> {
        let stream = self.0.receive_signal("VerifyFingerSelected").await?;
        Ok(stream.filter_map(move |message| async move {
            let name = message.body().deserialize::<String>().ok()?;
            Fingerprint::from_str(&name).ok()
        }))
    }

    #[doc(alias = "VerifyStatus")]
    pub async fn receive_verify_status(
        &self,
    ) -> Result<impl Stream<Item = (VerifyStatus, bool)> + '_> {
        let stream = self.0.receive_signal("VerifyStatus").await?;
        Ok(stream.filter_map(move |message| async move {
            let (status, done) = message.body().deserialize::<(String, bool)>().ok()?;
            Some((VerifyStatus::from_str(&status).ok()?, done))
        }))
    }

    pub async fn enroll_start(&self, finger: Fingerprint) -> Result<()> {
        self.0
            .call_method("EnrollStart", &(finger.as_str(),))
            .await?;
        Ok(())
    }

    pub async fn enroll_stop(&self) -> Result<()> {
        self.0.call_method("EnrollStop", &()).await?;
        Ok(())
    }

    #[doc(alias = "EnrollStatus")]
    pub async fn receive_enroll_status(
        &self,
    ) -> Result<impl Stream<Item = (EnrollStatus, bool)> + '_> {
        let stream = self.0.receive_signal("EnrollStatus").await?;
        Ok(stream.filter_map(move |message| async move {
            let (status, done) = message.body().deserialize::<(String, bool)>().ok()?;
            Some((EnrollStatus::from_str(&status).ok()?, done))
        }))
    }
}
