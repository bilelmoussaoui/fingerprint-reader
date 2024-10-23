use crate::enums::{EnrollStatus, Fingerprint, ScanType, VerifyStatus};
use crate::error::Result;
use futures_util::{Stream, StreamExt};
use std::str::FromStr;
use zbus::zvariant::ObjectPath;

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
        ScanType::from_str(&ty)
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
            fingers.push(Fingerprint::from_str(&name)?);
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
