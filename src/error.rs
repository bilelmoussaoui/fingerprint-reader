#[derive(zbus::DBusError, Debug)]
#[zbus(prefix = "net.reactivated.Fprint.Error")]
pub enum Error {
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

pub type Result<T> = std::result::Result<T, Error>;
