#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error(u32);

impl Error {
    pub const OK: Error = Error(ffi::btrfs_util_error_BTRFS_UTIL_OK);
    pub const STOP_ITERATION: Error = Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_STOP_ITERATION);
    pub const NO_MEMORY: Error = Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_NO_MEMORY);
    pub const INVALID_ARGUMENT: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_INVALID_ARGUMENT);
    pub const NOT_BTRFS: Error = Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_NOT_BTRFS);
    pub const NOT_SUBVOLUME: Error = Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_NOT_SUBVOLUME);
    pub const SUBVOLUME_NOT_FOUND: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_SUBVOLUME_NOT_FOUND);
    pub const OPEN_FAILED: Error = Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_OPEN_FAILED);
    pub const RMDIR_FAILED: Error = Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_RMDIR_FAILED);
    pub const UNLINK_FAILED: Error = Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_UNLINK_FAILED);
    pub const STAT_FAILED: Error = Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_STAT_FAILED);
    pub const STATFS_FAILED: Error = Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_STATFS_FAILED);
    pub const SEARCH_FAILED: Error = Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_SEARCH_FAILED);
    pub const INO_LOOKUP_FAILED: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_INO_LOOKUP_FAILED);
    pub const SUBVOL_GETFLAGS_FAILED: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_SUBVOL_GETFLAGS_FAILED);
    pub const SUBVOL_SETFLAGS_FAILED: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_SUBVOL_SETFLAGS_FAILED);
    pub const SUBVOL_CREATE_FAILED: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_SUBVOL_CREATE_FAILED);
    pub const SNAP_CREATE_FAILED: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_SNAP_CREATE_FAILED);
    pub const SNAP_DESTROY_FAILED: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_SNAP_DESTROY_FAILED);
    pub const DEFAULT_SUBVOL_FAILED: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_DEFAULT_SUBVOL_FAILED);
    pub const SYNC_FAILED: Error = Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_SYNC_FAILED);
    pub const START_SYNC_FAILED: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_START_SYNC_FAILED);
    pub const WAIT_SYNC_FAILED: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_WAIT_SYNC_FAILED);
    pub const GET_SUBVOL_INFO_FAILED: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_GET_SUBVOL_INFO_FAILED);
    pub const GET_SUBVOL_ROOTREF_FAILED: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_GET_SUBVOL_ROOTREF_FAILED);
    pub const INO_LOOKUP_USER_FAILED: Error =
        Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_INO_LOOKUP_USER_FAILED);
    pub const FS_INFO_FAILED: Error = Error(ffi::btrfs_util_error_BTRFS_UTIL_ERROR_FS_INFO_FAILED);

    #[inline]
    pub fn is_unknown(&self) -> bool {
        self.0 > Self::FS_INFO_FAILED.0
    }
}

impl From<u32> for Error {
    fn from(errcode: u32) -> Self {
        Error(errcode)
    }
}

impl From<Error> for u32 {
    fn from(err: Error) -> Self {
        err.0
    }
}