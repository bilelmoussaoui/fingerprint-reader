#[derive(zbus::DBusError, Debug)]
#[zbus(prefix = "net.reactivated.Fprint.Error")]
pub enum DBusError {
    #[zbus(error)]
    /// ZBus specific error.
    ZBus(zbus::Error),
    /// Fingerprint is not deleted from fprintd storage
    PrintsNotDeleted,
    /// Finger name is invalid
    InvalidFingername,
    /// The device was not claimed
    ClaimDevice,
    /// No ongoing verification
    NoActionInProgress,
    // The chosen user doesn't have the requested fingerprint enrolled
    NoEnrolledPrints,
    /// The caller lacks the appropriate PolicyKit authorization
    PermissionDenied,
    /// The device was already being used
    AlreadyInUse,
    /// An internal error
    Internal,
}

#[derive(Debug)]
pub enum Error {
    DBus(DBusError),
    InvalidScanType(String),
    InvalidFingerprint(String),
    InvalidVerifyStatus(String),
    InvalidEnrollStatus(String),
}

impl From<DBusError> for Error {
    fn from(value: DBusError) -> Self {
        Self::DBus(value)
    }
}

impl From<zbus::Error> for Error {
    fn from(value: zbus::Error) -> Self {
        Self::DBus(DBusError::ZBus(value))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DBus(dbus_error) => f.write_str(&dbus_error.to_string()),
            Error::InvalidScanType(e) => f.write_fmt(format_args!("Invalid scan type {e}")),
            Error::InvalidFingerprint(e) => f.write_fmt(format_args!("Invalid fingerprint {e}")),
            Error::InvalidVerifyStatus(e) => f.write_fmt(format_args!("Invalid verify status {e}")),
            Error::InvalidEnrollStatus(e) => f.write_fmt(format_args!("Invalid enroll status {e}")),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DBus(dbus_error) => Some(dbus_error),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
