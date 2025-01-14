mod error;
mod qgroup;
mod subvol;

use bitflags::bitflags;
use std::{
    ffi::CString,
    os::{raw::c_int, unix::prelude::OsStrExt},
    path::Path,
};

pub use error::Error;
pub use qgroup::QgroupInherit;
pub use subvol::*;
pub const FS_TREE_OBJECTID: u64 = 5;

/// Forces a sync on a Btrfs filesystem containing the `path`.
pub fn sync<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let cpath = CString::new(path.as_ref().as_os_str().as_bytes()).unwrap();
    let errcode = unsafe { ffi::btrfs_util_sync(cpath.as_ptr()) };
    if errcode == ffi::btrfs_util_error::BTRFS_UTIL_OK {
        Ok(())
    } else {
        Err(Error::new(errcode))
    }
}

/// Returns whether the given `path` is a Btrfs subvolume.
pub fn is_subvolume<P: AsRef<Path>>(path: P) -> Result<bool, Error> {
    let cpath = CString::new(path.as_ref().as_os_str().as_bytes()).unwrap();
    let errcode = unsafe { ffi::btrfs_util_is_subvolume(cpath.as_ptr()) };
    match errcode {
        ffi::btrfs_util_error::BTRFS_UTIL_OK => Ok(true),
        ffi::btrfs_util_error::BTRFS_UTIL_ERROR_NOT_SUBVOLUME
        | ffi::btrfs_util_error::BTRFS_UTIL_ERROR_NOT_BTRFS => Ok(false),
        _ => Err(Error::new(errcode)),
    }
}

/// Gets the ID of the subvolume containing the `path`.
pub fn subvolume_id<P: AsRef<Path>>(path: P) -> Result<u64, Error> {
    let cpath = CString::new(path.as_ref().as_os_str().as_bytes()).unwrap();
    let mut ret: u64 = 0;
    let errcode = unsafe { ffi::btrfs_util_subvolume_id(cpath.as_ptr(), &mut ret) };
    if errcode == ffi::btrfs_util_error::BTRFS_UTIL_OK {
        Ok(ret)
    } else {
        Err(Error::new(errcode))
    }
}

/// Gets information about the subvolume with the given `id` on the filesystem containing the `path`.
///
/// This requires appropriate privilege (`CAP_SYS_ADMIN`).
pub fn subvolume_info_with_id<P: AsRef<Path>>(path: P, id: u64) -> Result<SubvolumeInfo, Error> {
    let cpath = CString::new(path.as_ref().as_os_str().as_bytes()).unwrap();
    let mut out = SubvolumeInfo::new();
    unsafe {
        let errcode = ffi::btrfs_util_subvolume_info(cpath.as_ptr(), id, out.as_ptr());
        if errcode != ffi::btrfs_util_error::BTRFS_UTIL_OK {
            return Err(Error::new(errcode));
        }
    }
    Ok(out)
}

/// Gets information about the subvolume at the given `path`.
///
/// This requires appropriate privilege (`CAP_SYS_ADMIN`) unless the kernel supports
/// `BTRFS_IOC_GET_SUBVOL_INFO` (kernel >= 4.18).
pub fn subvolume_info<P: AsRef<Path>>(path: P) -> Result<SubvolumeInfo, Error> {
    subvolume_info_with_id(path, 0)
}

/// Returns whether a subvolume is read-only.
pub fn subvolume_read_only<P: AsRef<Path>>(path: P) -> Result<bool, Error> {
    let cpath = CString::new(path.as_ref().as_os_str().as_bytes()).unwrap();
    let mut ret: bool = false;

    let errcode = unsafe { ffi::btrfs_util_get_subvolume_read_only(cpath.as_ptr(), &mut ret) };
    if errcode == ffi::btrfs_util_error::BTRFS_UTIL_OK {
        Ok(ret)
    } else {
        Err(Error::new(errcode))
    }
}

/// Set whether a subvolume is read-only.
///
/// This requires appropriate privilege (CAP_SYS_ADMIN).
pub fn set_subvolume_read_only<P: AsRef<Path>>(path: P, read_only: bool) -> Result<(), Error> {
    let cpath = CString::new(path.as_ref().as_os_str().as_bytes()).unwrap();
    let errcode = unsafe { ffi::btrfs_util_set_subvolume_read_only(cpath.as_ptr(), read_only) };
    if errcode == ffi::btrfs_util_error::BTRFS_UTIL_OK {
        Ok(())
    } else {
        Err(Error::new(errcode))
    }
}

bitflags! {
    #[derive(Default)]
    pub struct DeleteSubvolumeFlags: c_int {
        const RECURSIVE = ffi::BTRFS_UTIL_DELETE_SUBVOLUME_RECURSIVE as c_int;
    }
}

/// Deletes a subvolume or snapshot.
pub fn delete_subvolume<P: AsRef<Path>>(path: P, flags: DeleteSubvolumeFlags) -> Result<(), Error> {
    let cpath = CString::new(path.as_ref().as_os_str().as_bytes()).unwrap();
    let cflags = flags.bits();
    unsafe {
        let errcode = ffi::btrfs_util_delete_subvolume(cpath.as_ptr(), cflags);
        if errcode != ffi::btrfs_util_error::BTRFS_UTIL_OK {
            return Err(Error::new(errcode));
        }
    }
    Ok(())
}

bitflags! {
    #[derive(Default)]
    pub struct CreateSubvolumeFlags: c_int {}
}

/// Creates a new subvolume.
pub fn create_subvolume<P: AsRef<Path>>(
    path: P,
    flags: CreateSubvolumeFlags,
    qgroup: Option<QgroupInherit>,
) -> Result<(), Error> {
    let cpath = CString::new(path.as_ref().as_os_str().as_bytes()).unwrap();
    let cflags = flags.bits();
    let cqgroup: *mut ffi::btrfs_util_qgroup_inherit = if let Some(qg) = qgroup {
        qg.as_ptr()
    } else {
        std::ptr::null_mut()
    };
    let errcode = unsafe {
        ffi::btrfs_util_create_subvolume(cpath.as_ptr(), cflags, std::ptr::null_mut(), cqgroup)
    };
    if errcode != ffi::btrfs_util_error::BTRFS_UTIL_OK {
        Err(Error::new(errcode))
    } else {
        Ok(())
    }
}

bitflags! {
    #[derive(Default)]
    pub struct CreateSnapshotFlags: c_int {
        const READ_ONLY	= ffi::BTRFS_UTIL_CREATE_SNAPSHOT_READ_ONLY as c_int;
        const RECURSIVE = ffi::BTRFS_UTIL_CREATE_SNAPSHOT_RECURSIVE as c_int;
    }
}

/// Creates a new snapshot from a source subvolume.
pub fn create_snapshot<P: AsRef<Path>, Q: AsRef<Path>>(
    source: P,
    path: Q,
    flags: CreateSnapshotFlags,
    qgroup: Option<QgroupInherit>,
) -> Result<(), Error> {
    let csource = CString::new(source.as_ref().as_os_str().as_bytes()).unwrap();
    let cpath = CString::new(path.as_ref().as_os_str().as_bytes()).unwrap();
    let cflags = flags.bits();
    let unused = std::ptr::null_mut();
    let cqgroup: *mut ffi::btrfs_util_qgroup_inherit = if let Some(qg) = qgroup {
        qg.as_ptr()
    } else {
        std::ptr::null_mut()
    };
    unsafe {
        let errcode = ffi::btrfs_util_create_snapshot(
            csource.as_ptr(),
            cpath.as_ptr(),
            cflags,
            unused,
            cqgroup,
        );
        if errcode != ffi::btrfs_util_error::BTRFS_UTIL_OK {
            return Err(Error::new(errcode));
        }
    }
    Ok(())
}
