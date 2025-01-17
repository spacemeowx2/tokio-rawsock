mod data_link;
mod err;
mod interf_desc;
mod lib_version;
mod packet;

use crate::traits::Library;
use crate::{pcap, pfring, wpcap};
use std::sync::Arc;

pub use self::data_link::DataLink;
pub use self::err::Error;
pub use self::interf_desc::InterfaceDescription;
pub use self::lib_version::LibraryVersion;
pub use self::packet::{BorrowedPacket, OwnedPacket, Packet};

/**
Opens optimal library available on the platform.

# Example

```no_run
use tokio_rawsock::open_best_library;

let lib = open_best_library().expect("Could not open any library.");

// do something with the library
```
*/
pub fn open_best_library() -> Result<Box<dyn Library>, Error> {
    if let Ok(l) = pfring::Library::open_default_paths() {
        return Ok(Box::new(l));
    }
    if let Ok(l) = wpcap::Library::open_default_paths() {
        return Ok(Box::new(l));
    }
    match pcap::Library::open_default_paths() {
        Ok(l) => Ok(Box::new(l)),
        Err(e) => Err(e),
    }
}

/// Multi-thread version of open_best_library()
pub fn open_best_library_arc() -> Result<Arc<dyn Library>, Error> {
    if let Ok(l) = pfring::Library::open_default_paths() {
        return Ok(Arc::new(l));
    }
    if let Ok(l) = wpcap::Library::open_default_paths() {
        return Ok(Arc::new(l));
    }
    match pcap::Library::open_default_paths() {
        Ok(l) => Ok(Arc::new(l)),
        Err(e) => Err(e),
    }
}

///Provides library statistics
#[derive(Copy, Clone, Debug)]
pub struct Stats {
    /// Received frames
    pub received: u64,
    ///Dropped frames
    pub dropped: u64,
}
