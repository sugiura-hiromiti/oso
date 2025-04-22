use core::ffi::c_void;

use crate::Status;
use crate::raw::types::Char16;
use crate::raw::types::Guid;
use crate::raw::types::file::FileAttributes;
use crate::raw::types::file::FileIoToken;
use crate::raw::types::file::OpenMode;

#[repr(C)]
pub struct SimpleFileSystemProtocol {
	revision:    u64,
	open_volume: unsafe extern "efiapi" fn(*mut Self, root: *mut *mut FileProtocolV1,),
}

/**
Opens a new file relative to the source directory’s location.

# Description

The Open()function opens the file or directory referred to by FileName
relative to the location of This and returns a NewHandle.
The FileName may include the following path modifiers:

“\”

If the filename starts with a “\” the relative location is the root directory
that This resides on; otherwise “" separates name components.
Each name component is opened in turn,
and the handle to the last file opened is returned.

“.”

Opens the current location.

“..”

Opens the parent directory for the current location.
If the location is the root directory the request will return an error,
as there is no parent directory for the root directory.

If EFI_FILE_MODE_CREATE is set, then the file is created in the directory.
If the final location of FileName does not refer to a directory,
then the operation fails.
If the file does not exist in the directory, then a new file is created.
If the file already exists in the directory, then the existing file is opened.

If the medium of the device changes,
all accesses (including the File handle) will result in EFI_MEDIA_CHANGED.
To access the new medium, the volume must be reopened.

# Params

- NewHandle
A pointer to the location to return the opened handle for the new file.
See the type EFI_FILE_PROTOCOL description.

- FileName
The Null-terminated string of the name of the file to be opened.
The file name may contain the following path modifiers: “", “.”, and “..”.

- OpenMode
The mode to open the file.
The only valid combinations that the file may be opened with are:
Read, Read/Write, or Create/Read/Write.
See “Related Definitions” below.

- Attributes
Only valid for EFI_FILE_MODE_CREATE,
in which case these are the attribute bits for the newly created file.
See “Related Definitions” below.

# Return

|code|desc|
|:--|:--|
|EFI_SUCCESS |The file was opened|
|EFI_NOT_FOUND |The specified file could not be found on the device|
|EFI_NO_MEDIA |The device has no medium|
|EFI_MEDIA_CHANGED |The device has a different medium in it or the medium is no longer supported|
|EFI_DEVICE_ERROR |The device reported an error|
|EFI_VOLUME_CORRUPTED |The file system structures are corrupted|
|EFI_WRITE_PROTECTED |An attempt was made to create a file,|
| |or open a file for write when the media is write-protected|
|EFI_ACCESS_DENIED |The service denied access to the file|
|EFI_OUT_OF_RESOURCES |Not enough resources were available to open the file|
|EFI_VOLUME_FULL |The volume is full|
|EFI_INVALID_PARAMETER |This refers to a regular file, not a directory|

---
*/
type FileOpen = unsafe extern "efiapi" fn(
	this: *mut FileProtocolV1,
	new_handle: *mut *mut FileProtocolV1,
	file_name: *const Char16,
	open_mode: OpenMode,
	attr: FileAttributes,
) -> Status;

/**
Closes a specified file handle.

# Description

The Close() function closes a specified file handle.
All “dirty” cached file data is flushed to the device,
and the file is closed. In all cases the handle is closed.
The operation will wait for all pending asynchronous I/O requests to complete before completing.

# Return

|code|desc|
|:--|:--|
|EFI_SUCCESS|The file was closed|

---
*/
type FileClose = unsafe extern "efiapi" fn(this: *mut FileProtocolV1,) -> Status;

/**
Closes and deletes a file.

# Description

The Delete() function closes and deletes a file. In all cases the file handle is closed. If the file cannot be deleted, the warning code EFI_WARN_DELETE_FAILURE is returned, but the handle is still closed.

# Return

|code|desc|
|:--|:--|
|EFI_SUCCESS |The file was closed and deleted, and the handle was closed|
|EFI_WARN_DELETE_FAILURE |The handle was closed, but the file was not deleted|

---
*/
type FileDelete = unsafe extern "efiapi" fn(this: *mut FileProtocolV1,) -> Status;

/**
The Read() function reads data from a file.

# Description

If This is not a directory,
the function reads the requested number of bytes from the file
at the file’s current position and returns them in Buffer.
If the read goes beyond the end of the file,
the read length is truncated to the end of the file.
The file’s current position is increased by the number of bytes returned.

If This is a directory,
the function reads the directory entry at the
file’s current position and returns the entry in Buffer.
If the Buffer is not large enough to hold the current directory entry,
then EFI_BUFFER_TOO_SMALL is returned and the current file position is not updated.
BufferSize is set to be the size of the buffer needed to read the entry. On success,
the current position is updated to the next directory entry.
If there are no more directory entries, the read returns a zero-length buffer.
EFI_FILE_INFO is the structure returned as the directory entry.

# Params

- BufferSize
On input, the size of the Buffer. On output, the amount of data returned in Buffer.
In both cases, the size is measured in bytes.

- Buffer
The buffer into which the data is read.

# Return

|code|desc|
|:--|:--|
|EFI_SUCCESS |The data was read|
|EFI_NO_MEDIA |The device has no medium|
|EFI_DEVICE_ERROR |The device reported an error|
|EFI_DEVICE_ERROR |An attempt was made to read from a deleted file|
|EFI_DEVICE_ERROR |On entry, the current file position is beyond the end of the file|
|EFI_VOLUME_CORRUPTED |The file system structures are corrupted|
|EFI_BUFFER_TOO_SMALL |The BufferSize is too small to read the current directory entry|
| |BufferSize has been updated with the size needed to complete the request|

---
*/
type FileRead = unsafe extern "efiapi" fn(
	this: *mut FileProtocolV1,
	buf_size: *mut usize,
	buf: *mut c_void,
) -> Status;

/**
Writes data to a file.

# Description

The Write() function writes the specified number of bytes to the file at the current file position.
The current file position is advanced the actual number of bytes written,
which is returned in BufferSize.
Partial writes only occur when there has been a data error
during the write attempt (such as “file space full”).
The file is automatically grown to hold the data if required.

Direct writes to opened directories are not supported.

# Params

- BufferSize
On input, the size of the Buffer.
On output, the amount of data actually written.
In both cases, the size is measured in bytes.

- Buffer
The buffer of data to write.

# Return

|code|desc|
|:--|:--|
|EFI_SUCCESS |The data was written|
|EFI_UNSUPPORT |Writes to open directory files are not supported|
|EFI_NO_MEDIA |The device has no medium|
|EFI_DEVICE_ERROR |The device reported an error|
|EFI_DEVICE_ERROR |An attempt was made to write to a deleted file|
|EFI_VOLUME_CORRUPTED |The file system structures are corrupted|
|EFI_WRITE_PROTECTED |The file or medium is write-protected|
|EFI_ACCESS_DENIED |The file was opened read only|
|EFI_VOLUME_FULL |The volume is full|

---
*/
type FileWrite = unsafe extern "efiapi" fn(
	this: *mut FileProtocolV1,
	buf_size: *mut usize,
	buf: *mut c_void,
) -> Status;
type FileGetPosition =
	unsafe extern "efiapi" fn(this: *const FileProtocolV1, position: *mut u64,) -> Status;
type FileSetPosition =
	unsafe extern "efiapi" fn(this: *mut FileProtocolV1, position: u64,) -> Status;
type FileGetInfo = unsafe extern "efiapi" fn(
	this: *mut FileProtocolV1,
	info_type: *const Guid,
	buf_size: *mut usize,
	buf: *mut c_void,
) -> Status;
type FileSetInfo = unsafe extern "efiapi" fn(
	this: *mut FileProtocolV1,
	info_type: *const Guid,
	buf_size: usize,
	buf: *mut c_void,
) -> Status;
type FileFlush = unsafe extern "efiapi" fn(this: *mut FileProtocolV1,) -> Status;
type FileOpenEx = unsafe extern "efiapi" fn(
	this: *mut FileProtocolV2,
	new_handle: *mut *mut FileProtocolV2,
	file_name: *const Char16,
	open_mode: OpenMode,
	attrs: FileAttributes,
	token: *mut FileIoToken,
) -> Status;
type FileReadEx =
	unsafe extern "efiapi" fn(this: *mut FileProtocolV2, token: *mut FileIoToken,) -> Status;
type FileWriteEx =
	unsafe extern "efiapi" fn(this: *mut FileProtocolV2, token: *mut FileIoToken,) -> Status;
type FileFlushEx =
	unsafe extern "efiapi" fn(this: *mut FileProtocolV2, token: *mut FileIoToken,) -> Status;

#[repr(C)]
pub struct FileProtocolV1 {
	revision:     u64,
	open:         FileOpen,
	close:        FileClose,
	delete:       FileDelete,
	read:         FileRead,
	write:        FileWrite,
	get_position: FileGetPosition,
	set_position: FileSetPosition,
	get_info:     FileGetInfo,
	set_info:     FileSetInfo,
	flush:        FileFlush,
}

#[repr(C)]
pub struct FileProtocolV2 {
	v1:    FileProtocolV1,
	open:  FileOpenEx,
	read:  FileReadEx,
	write: FileWriteEx,
	flush: FileFlushEx,
}
