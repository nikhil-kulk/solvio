//! Platform-independent abstractions over [`memmap2::Mmap::advise`]/[`memmap2::MmapMut::advise`]
//! and [`memmap2::Advice`].

use std::io;

use serde::Deserialize;

/// Global [`Advice`] value, to trivially set [`Advice`] value
/// used by all memmaps created by the [`segment`] crate.
///
/// See [`store_global`] and [`load_global`].
static ADVICE: parking_lot::RwLock<Advice> = parking_lot::RwLock::new(Advice::Random);

/// Set global [`Advice`] value.
///
/// When [`segment`] crate creates [`memmap2::Mmap`] or [`memmap2::MmapMut`]
/// _for a memmapeped, on-disk HNSW index or vector storage access_
/// it will "advise" created memmap with the current global [`Advice`] value
/// (obtained with [`load_global`]).
///
/// It is recommended to set the desired [`Advice`] value before calling any other function
/// from the [`segment`] crate and not to change it afterwards.
///
/// The [`segment`] crate itself does not modify global [`Advice`] value.
///
/// Default global [`Advice`] value is [`Advice::Random`].
pub fn set_global(advice: Advice) {
    *ADVICE.write() = advice;
}

/// Get current global [`Advice`] value.
pub fn get_global() -> Advice {
    *ADVICE.read()
}

/// Platform-independent version of [`memmap2::Advice`].
/// See [`memmap2::Advice`] and [madvise()] man page.
///
/// [madvice()]: https://man7.org/linux/man-pages/man2/madvise.2.html
#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Advice {
    /// See [`memmap2::Advice::Normal`].
    Normal,

    /// See [`memmap2::Advice::Random`].
    Random,

    /// See [`memmap2::Advice::Sequential`].
    Sequential,

    /// See [`memmap2::Advice::PopulateRead`].
    PopulateRead,
}

impl TryFrom<Advice> for Option<memmap2::Advice> {
    type Error = io::Error;

    fn try_from(advice: Advice) -> Result<Self, Self::Error> {
        match advice {
            #[cfg(unix)]
            Advice::Normal => Ok(Some(memmap2::Advice::Normal)),

            #[cfg(unix)]
            Advice::Random => Ok(Some(memmap2::Advice::Random)),

            #[cfg(unix)]
            Advice::Sequential => Ok(Some(memmap2::Advice::Sequential)),

            #[cfg(target_os = "linux")]
            Advice::PopulateRead => Ok(Some(memmap2::Advice::PopulateRead)),

            #[cfg(not(target_os = "linux"))]
            Advice::PopulateRead => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "MADV_POPULATE_READ is only supported on Linux",
            )),

            #[cfg(not(unix))]
            _ => Ok(None),
        }
    }
}

/// Advise OS how given memory map will be accessed. On non-Unix platforms this is a no-op.
pub fn madvise(madviseable: &impl Madviseable, advice: Advice) -> io::Result<()> {
    madviseable.madvise(advice)
}

/// Generic, platform-independent abstraction
/// over [`memmap2::Mmap::advise`] and [`memmap2::MmapMut::advise`].
pub trait Madviseable {
    /// Advise OS how given memory map will be accessed. On non-Unix platforms this is a no-op.
    fn madvise(&self, advice: Advice) -> io::Result<()>;
}

impl Madviseable for memmap2::Mmap {
    fn madvise(&self, advice: Advice) -> io::Result<()> {
        match advice.try_into()? {
            Some(advice) => self.advise(advice),
            None => Ok(()),
        }
    }
}

impl Madviseable for memmap2::MmapMut {
    fn madvise(&self, advice: Advice) -> io::Result<()> {
        match advice.try_into()? {
            Some(advice) => self.advise(advice),
            None => Ok(()),
        }
    }
}
