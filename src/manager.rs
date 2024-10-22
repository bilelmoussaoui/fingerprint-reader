use zbus::zvariant::OwnedObjectPath;

use crate::{error::Result, Device};

#[doc(alias = "net.reactivated.Fprint.Manager")]
pub struct Manager<'a>(zbus::Proxy<'a>, zbus::Connection);

impl<'a> Manager<'a> {
    pub async fn new(cnx: &zbus::Connection) -> Result<Self> {
        let proxy = zbus::proxy::Builder::new(cnx)
            .interface("net.reactivated.Fprint.Manager")?
            .path("/net/reactivated/Fprint/Manager")?
            .destination("net.reactivated.Fprint")?
            .build()
            .await?;
        Ok(Self(proxy, cnx.clone()))
    }

    #[doc(alias = "GetDevices")]
    pub async fn devices(&self) -> Result<Vec<Device<'_>>> {
        let objects = self
            .0
            .call_method("GetDevices", &())
            .await?
            .body()
            .deserialize::<Vec<OwnedObjectPath>>()?;
        let mut devices = Vec::with_capacity(objects.capacity());
        for path in objects {
            devices.push(Device::new(&self.1, path).await?);
        }
        Ok(devices)
    }

    #[doc(alias = "GetDefaultDevice")]
    pub async fn default_device(&self) -> Result<Device<'_>> {
        let object = self
            .0
            .call_method("GetDefaultDevice", &())
            .await?
            .body()
            .deserialize::<OwnedObjectPath>()?;
        // TODO: handle net.reactivated.Fprint.Error.NoSuchDevice
        Device::new(&self.1, object).await
    }
}
