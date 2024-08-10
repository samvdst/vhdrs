#![allow(clippy::enum_variant_names, non_camel_case_types)]

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to open virtual disk. Error code: {0}")]
    UNKNOWN_OPEN_ERROR(u32),

    #[error("Failed to attach virtual disk. Error code: {0}")]
    UNKNOWN_ATTACH_ERROR(u32),

    #[error("Incorrect function.")]
    ERROR_INVALID_FUNCTION,

    #[error("The system cannot find the file specified.")]
    ERROR_FILE_NOT_FOUND,

    #[error("The system cannot find the path specified.")]
    ERROR_PATH_NOT_FOUND,

    #[error("The system cannot open the file.")]
    ERROR_TOO_MANY_OPEN_FILES,

    #[error("Access is denied.")]
    ERROR_ACCESS_DENIED,

    #[error("The handle is invalid.")]
    ERROR_INVALID_HANDLE,

    #[error("The storage control blocks were destroyed.")]
    ERROR_ARENA_TRASHED,

    #[error("Not enough memory resources are available to process this command.")]
    ERROR_NOT_ENOUGH_MEMORY,

    #[error("The storage control block address is invalid.")]
    ERROR_INVALID_BLOCK,

    #[error("The environment is incorrect.")]
    ERROR_BAD_ENVIRONMENT,

    #[error("An attempt was made to load a program with an incorrect format.")]
    ERROR_BAD_FORMAT,

    #[error("The access code is invalid.")]
    ERROR_INVALID_ACCESS,

    #[error("The data is invalid.")]
    ERROR_INVALID_DATA,

    #[error("Not enough storage is available to complete this operation.")]
    ERROR_OUTOFMEMORY,

    #[error("The system cannot find the drive specified.")]
    ERROR_INVALID_DRIVE,

    #[error("The directory cannot be removed.")]
    ERROR_CURRENT_DIRECTORY,

    #[error("The system cannot move the file to a different disk drive.")]
    ERROR_NOT_SAME_DEVICE,

    #[error("There are no more files.")]
    ERROR_NO_MORE_FILES,

    #[error("The media is write protected.")]
    ERROR_WRITE_PROTECT,

    #[error("The system cannot find the device specified.")]
    ERROR_BAD_UNIT,

    #[error("The device is not ready.")]
    ERROR_NOT_READY,

    #[error("The device does not recognize the command.")]
    ERROR_BAD_COMMAND,

    #[error("Data error (cyclic redundancy check).")]
    ERROR_CRC,

    #[error("The program issued a command but the command length is incorrect.")]
    ERROR_BAD_LENGTH,

    #[error("The drive cannot locate a specific area or track on the disk.")]
    ERROR_SEEK,

    #[error("The specified disk or diskette cannot be accessed.")]
    ERROR_NOT_DOS_DISK,

    #[error("The drive cannot find the sector requested.")]
    ERROR_SECTOR_NOT_FOUND,

    #[error("The printer is out of paper.")]
    ERROR_OUT_OF_PAPER,

    #[error("The system cannot write to the specified device.")]
    ERROR_WRITE_FAULT,

    #[error("The system cannot read from the specified device.")]
    ERROR_READ_FAULT,

    #[error("A device attached to the system is not functioning.")]
    ERROR_GEN_FAILURE,

    #[error("The process cannot access the file because it is being used by another process.")]
    ERROR_SHARING_VIOLATION,

    #[error("The process cannot access the file because another process has locked a portion of the file.")]
    ERROR_LOCK_VIOLATION,

    #[error(
        "The wrong diskette is in the drive. Insert %2 (Volume Serial Number: %3) into drive %1."
    )]
    ERROR_WRONG_DISK,

    #[error("Too many files opened for sharing.")]
    ERROR_SHARING_BUFFER_EXCEEDED,

    #[error("Reached the end of the file.")]
    ERROR_HANDLE_EOF,

    #[error("The disk is full.")]
    ERROR_HANDLE_DISK_FULL,

    #[error("The request is not supported.")]
    ERROR_NOT_SUPPORTED,

    #[error("Windows cannot find the network path. Verify that the network path is correct and the destination computer is not busy or turned off. If Windows still cannot find the network path, contact your network administrator.")]
    ERROR_REM_NOT_LIST,

    #[error("You were not connected because a duplicate name exists on the network. If joining a domain, go to System in Control Panel to change the computer name and try again. If joining a workgroup, choose another workgroup name.")]
    ERROR_DUP_NAME,

    #[error("The network path was not found.")]
    ERROR_BAD_NETPATH,

    #[error("The network is busy.")]
    ERROR_NETWORK_BUSY,

    #[error("The specified network resource or device is no longer available.")]
    ERROR_DEV_NOT_EXIST,

    #[error("The network BIOS command limit has been reached.")]
    ERROR_TOO_MANY_CMDS,

    #[error("A network adapter hardware error occurred.")]
    ERROR_ADAP_HDW_ERR,

    #[error("The specified server cannot perform the requested operation.")]
    ERROR_BAD_NET_RESP,

    #[error("An unexpected network error occurred.")]
    ERROR_UNEXP_NET_ERR,

    #[error("The remote adapter is not compatible.")]
    ERROR_BAD_REM_ADAP,

    #[error("The printer queue is full.")]
    ERROR_PRINTQ_FULL,

    #[error("Space to store the file waiting to be printed is not available on the server.")]
    ERROR_NO_SPOOL_SPACE,

    #[error("Your file waiting to be printed was deleted.")]
    ERROR_PRINT_CANCELLED,

    #[error("The specified network name is no longer available.")]
    ERROR_NETNAME_DELETED,

    #[error("Network access is denied.")]
    ERROR_NETWORK_ACCESS_DENIED,

    #[error("The network resource type is not correct.")]
    ERROR_BAD_DEV_TYPE,

    #[error("The network name cannot be found.")]
    ERROR_BAD_NET_NAME,

    #[error("The name limit for the local computer network adapter card was exceeded.")]
    ERROR_TOO_MANY_NAMES,

    #[error("The network BIOS session limit was exceeded.")]
    ERROR_TOO_MANY_SESS,

    #[error("The remote server has been paused or is in the process of being started.")]
    ERROR_SHARING_PAUSED,

    #[error("No more connections can be made to this remote computer at this time because there are already as many connections as the computer can accept.")]
    ERROR_REQ_NOT_ACCEP,

    #[error("The specified printer or disk device has been paused.")]
    ERROR_REDIR_PAUSED,

    #[error("The file exists.")]
    ERROR_FILE_EXISTS,

    #[error("The directory or file cannot be created.")]
    ERROR_CANNOT_MAKE,

    #[error("Fail on INT 24.")]
    ERROR_FAIL_I24,

    #[error("Storage to process this request is not available.")]
    ERROR_OUT_OF_STRUCTURES,

    #[error("The local device name is already in use.")]
    ERROR_ALREADY_ASSIGNED,

    #[error("The specified network password is not correct.")]
    ERROR_INVALID_PASSWORD,

    #[error("The parameter is incorrect.")]
    ERROR_INVALID_PARAMETER,

    #[error("A write fault occurred on the network.")]
    ERROR_NET_WRITE_FAULT,

    #[error("The system cannot start another process at this time.")]
    ERROR_NO_PROC_SLOTS,

    #[error("Cannot create another system semaphore.")]
    ERROR_TOO_MANY_SEMAPHORES,

    #[error("The exclusive semaphore is owned by another process.")]
    ERROR_EXCL_SEM_ALREADY_OWNED,

    #[error("The semaphore is set and cannot be closed.")]
    ERROR_SEM_IS_SET,

    #[error("The semaphore cannot be set again.")]
    ERROR_TOO_MANY_SEM_REQUESTS,

    #[error("Cannot request exclusive semaphores at interrupt time.")]
    ERROR_INVALID_AT_INTERRUPT_TIME,

    #[error("The previous ownership of this semaphore has ended.")]
    ERROR_SEM_OWNER_DIED,

    #[error("Insert the diskette for drive %1.")]
    ERROR_SEM_USER_LIMIT,

    #[error("The program stopped because an alternate diskette was not inserted.")]
    ERROR_DISK_CHANGE,

    #[error("The disk is in use or locked by another process.")]
    ERROR_DRIVE_LOCKED,

    #[error("The pipe has been ended.")]
    ERROR_BROKEN_PIPE,

    #[error("The system cannot open the device or file specified.")]
    ERROR_OPEN_FAILED,

    #[error("The file name is too long.")]
    ERROR_BUFFER_OVERFLOW,

    #[error("There is not enough space on the disk.")]
    ERROR_DISK_FULL,

    #[error("No more internal file identifiers available.")]
    ERROR_NO_MORE_SEARCH_HANDLES,

    #[error("The target internal file identifier is incorrect.")]
    ERROR_INVALID_TARGET_HANDLE,

    #[error("The IOCTL call made by the application program is not correct.")]
    ERROR_INVALID_CATEGORY,

    #[error("The verify-on-write switch parameter value is not correct.")]
    ERROR_INVALID_VERIFY_SWITCH,

    #[error("The system does not support the command requested.")]
    ERROR_BAD_DRIVER_LEVEL,

    #[error("This function is not supported on this system.")]
    ERROR_CALL_NOT_IMPLEMENTED,

    #[error("The semaphore timeout period has expired.")]
    ERROR_SEM_TIMEOUT,

    #[error("The data area passed to a system call is too small.")]
    ERROR_INSUFFICIENT_BUFFER,

    #[error("The filename, directory name, or volume label syntax is incorrect.")]
    ERROR_INVALID_NAME,

    #[error("The system call level is not correct.")]
    ERROR_INVALID_LEVEL,

    #[error("The disk has no volume label.")]
    ERROR_NO_VOLUME_LABEL,

    #[error("The specified module could not be found.")]
    ERROR_MOD_NOT_FOUND,

    #[error("The specified procedure could not be found.")]
    ERROR_PROC_NOT_FOUND,

    #[error("There are no child processes to wait for.")]
    ERROR_WAIT_NO_CHILDREN,

    #[error("The %1 application cannot be run in Win32 mode.")]
    ERROR_CHILD_NOT_COMPLETE,

    #[error("Attempt to use a file handle to an open disk partition for an operation other than raw disk I/O.")]
    ERROR_DIRECT_ACCESS_HANDLE,

    #[error("An attempt was made to move the file pointer before the beginning of the file.")]
    ERROR_NEGATIVE_SEEK,

    #[error("The file pointer cannot be set on the specified device or file.")]
    ERROR_SEEK_ON_DEVICE,

    #[error("A JOIN or SUBST command cannot be used for a drive that contains previously joined drives.")]
    ERROR_IS_JOIN_TARGET,

    #[error("An attempt was made to use a JOIN or SUBST command on a drive that has already been joined.")]
    ERROR_IS_JOINED,

    #[error("An attempt was made to use a JOIN or SUBST command on a drive that has already been substituted.")]
    ERROR_IS_SUBSTED,

    #[error("The system tried to delete the JOIN of a drive that is not joined.")]
    ERROR_NOT_JOINED,

    #[error("The system tried to delete the substitution of a drive that is not substituted.")]
    ERROR_NOT_SUBSTED,

    #[error("The system tried to join a drive to a directory on a joined drive.")]
    ERROR_JOIN_TO_JOIN,

    #[error("The system tried to substitute a drive to a directory on a substituted drive.")]
    ERROR_SUBST_TO_SUBST,

    #[error("The system tried to join a drive to a directory on a substituted drive.")]
    ERROR_JOIN_TO_SUBST,

    #[error("The system tried to SUBST a drive to a directory on a joined drive.")]
    ERROR_SUBST_TO_JOIN,

    #[error("The system cannot perform a JOIN or SUBST at this time.")]
    ERROR_BUSY_DRIVE,

    #[error(
        "The system cannot join or substitute a drive to or for a directory on the same drive."
    )]
    ERROR_SAME_DRIVE,

    #[error("The directory is not a subdirectory of the root directory.")]
    ERROR_DIR_NOT_ROOT,

    #[error("The directory is not empty.")]
    ERROR_DIR_NOT_EMPTY,

    #[error("The path specified is being used in a substitute.")]
    ERROR_IS_SUBST_PATH,

    #[error("Not enough resources are available to process this command.")]
    ERROR_IS_JOIN_PATH,

    #[error("The path specified cannot be used at this time.")]
    ERROR_PATH_BUSY,

    #[error("An attempt was made to join or substitute a drive for which a directory on the drive is the target of a previous substitute.")]
    ERROR_IS_SUBST_TARGET,

    #[error("System trace information was not specified in your CONFIG.SYS file, or tracing is disallowed.")]
    ERROR_SYSTEM_TRACE,

    #[error("The number of specified semaphore events for DosMuxSemWait is not correct.")]
    ERROR_INVALID_EVENT_COUNT,

    #[error("DosMuxSemWait did not execute; too many semaphores are already set.")]
    ERROR_TOO_MANY_MUXWAITERS,

    #[error("The DosMuxSemWait list is not correct.")]
    ERROR_INVALID_LIST_FORMAT,

    #[error(
        "The volume label you entered exceeds the label character limit of the target file system."
    )]
    ERROR_LABEL_TOO_LONG,

    #[error("Cannot create another thread.")]
    ERROR_TOO_MANY_TCBS,

    #[error("The recipient process has refused the signal.")]
    ERROR_SIGNAL_REFUSED,

    #[error("The segment is already discarded and cannot be locked.")]
    ERROR_DISCARDED,

    #[error("The segment is already unlocked.")]
    ERROR_NOT_LOCKED,

    #[error("The address for the thread ID is not correct.")]
    ERROR_BAD_THREADID_ADDR,

    #[error("One or more arguments are not correct.")]
    ERROR_BAD_ARGUMENTS,

    #[error("The specified path is invalid.")]
    ERROR_BAD_PATHNAME,

    #[error("A signal is already pending.")]
    ERROR_SIGNAL_PENDING,

    #[error("No more threads can be created in the system.")]
    ERROR_MAX_THRDS_REACHED,

    #[error("Unable to lock a region of a file.")]
    ERROR_LOCK_FAILED,

    #[error("The requested resource is in use.")]
    ERROR_BUSY,

    #[error("Device's command support detection is in progress.")]
    ERROR_DEVICE_SUPPORT_IN_PROGRESS,

    #[error("A lock request was not outstanding for the supplied cancel region.")]
    ERROR_CANCEL_VIOLATION,

    #[error("The file system does not support atomic changes to the lock type.")]
    ERROR_ATOMIC_LOCKS_NOT_SUPPORTED,

    #[error("The system detected a segment number that was not correct.")]
    ERROR_INVALID_SEGMENT_NUMBER,

    #[error("The operating system cannot run %1.")]
    ERROR_INVALID_ORDINAL,

    #[error("Cannot create a file when that file already exists.")]
    ERROR_ALREADY_EXISTS,

    #[error("The flag passed is not correct.")]
    ERROR_INVALID_FLAG_NUMBER,

    #[error("The specified system semaphore name was not found.")]
    ERROR_SEM_NOT_FOUND,

    #[error("The operating system cannot run %1.")]
    ERROR_INVALID_STARTING_CODESEG,

    #[error("The operating system cannot run %1.")]
    ERROR_INVALID_STACKSEG,

    #[error("The operating system cannot run %1.")]
    ERROR_INVALID_MODULETYPE,

    #[error("Cannot run %1 in Win32 mode.")]
    ERROR_INVALID_EXE_SIGNATURE,

    #[error("The operating system cannot run %1.")]
    ERROR_EXE_MARKED_INVALID,

    #[error("%1 is not a valid Win32 application.")]
    ERROR_BAD_EXE_FORMAT,

    #[error("The operating system cannot run %1.")]
    ERROR_ITERATED_DATA_EXCEEDS_64k,

    #[error("The operating system cannot run %1.")]
    ERROR_INVALID_MINALLOCSIZE,

    #[error("The operating system cannot run this application program.")]
    ERROR_DYNLINK_FROM_INVALID_RING,

    #[error("The operating system is not presently configured to run this application.")]
    ERROR_IOPL_NOT_ENABLED,

    #[error("The operating system cannot run %1.")]
    ERROR_INVALID_SEGDPL,

    #[error("The operating system cannot run this application program.")]
    ERROR_AUTODATASEG_EXCEEDS_64k,

    #[error("The code segment cannot be greater than or equal to 64K.")]
    ERROR_RING2SEG_MUST_BE_MOVABLE,

    #[error("The operating system cannot run %1.")]
    ERROR_RELOC_CHAIN_XEEDS_SEGLIM,

    #[error("The operating system cannot run %1.")]
    ERROR_INFLOOP_IN_RELOC_CHAIN,

    #[error("The system could not find the environment option that was entered.")]
    ERROR_ENVVAR_NOT_FOUND,

    #[error("No process in the command subtree has a signal handler.")]
    ERROR_NO_SIGNAL_SENT,

    #[error("The filename or extension is too long.")]
    ERROR_FILENAME_EXCED_RANGE,

    #[error("The ring 2 stack is in use.")]
    ERROR_RING2_STACK_IN_USE,

    #[error("The global filename characters, * or ?, are entered incorrectly or too many global filename characters are specified.")]
    ERROR_META_EXPANSION_TOO_LONG,

    #[error("The signal being posted is not correct.")]
    ERROR_INVALID_SIGNAL_NUMBER,

    #[error("The signal handler cannot be set.")]
    ERROR_THREAD_1_INACTIVE,

    #[error("The segment is locked and cannot be reallocated.")]
    ERROR_LOCKED,

    #[error("Too many dynamic-link modules are attached to this program or dynamic-link module.")]
    ERROR_TOO_MANY_MODULES,

    #[error("Cannot nest calls to LoadModule.")]
    ERROR_NESTING_NOT_ALLOWED,

    #[error("This version of %1 is not compatible with the version of Windows you're running. Check your computer's system information and then contact the software publisher.")]
    ERROR_EXE_MACHINE_TYPE_MISMATCH,

    #[error("The image file %1 is signed, unable to modify.")]
    ERROR_EXE_CANNOT_MODIFY_SIGNED_BINARY,

    #[error("The image file %1 is strong signed, unable to modify.")]
    ERROR_EXE_CANNOT_MODIFY_STRONG_SIGNED_BINARY,

    #[error("This file is checked out or locked for editing by another user.")]
    ERROR_FILE_CHECKED_OUT,

    #[error("The file must be checked out before saving changes.")]
    ERROR_CHECKOUT_REQUIRED,

    #[error("The file type being saved or retrieved has been blocked.")]
    ERROR_BAD_FILE_TYPE,

    #[error("The file size exceeds the limit allowed and cannot be saved.")]
    ERROR_FILE_TOO_LARGE,

    #[error("Access Denied. Before opening files in this location, you must first add the web site to your trusted sites list, browse to the web site, and select the option to login automatically.")]
    ERROR_FORMS_AUTH_REQUIRED,

    #[error("Operation did not complete successfully because the file contains a virus or potentially unwanted software.")]
    ERROR_VIRUS_INFECTED,

    #[error("This file contains a virus or potentially unwanted software and cannot be opened. Due to the nature of this virus or potentially unwanted software, the file has been removed from this location.")]
    ERROR_VIRUS_DELETED,

    #[error("The pipe is local.")]
    ERROR_PIPE_LOCAL,

    #[error("The pipe state is invalid.")]
    ERROR_BAD_PIPE,

    #[error("All pipe instances are busy.")]
    ERROR_PIPE_BUSY,

    #[error("The pipe is being closed.")]
    ERROR_NO_DATA,

    #[error("No process is on the other end of the pipe.")]
    ERROR_PIPE_NOT_CONNECTED,

    #[error("More data is available.")]
    ERROR_MORE_DATA,

    #[error("The session was canceled.")]
    ERROR_VC_DISCONNECTED,

    #[error("The specified extended attribute name was invalid.")]
    ERROR_INVALID_EA_NAME,

    #[error("The extended attributes are inconsistent.")]
    ERROR_EA_LIST_INCONSISTENT,

    #[error("The wait operation timed out.")]
    WAIT_TIMEOUT,

    #[error("No more data is available.")]
    ERROR_NO_MORE_ITEMS,

    #[error("The copy functions cannot be used.")]
    ERROR_CANNOT_COPY,

    #[error("The directory name is invalid.")]
    ERROR_DIRECTORY,

    #[error("The extended attributes did not fit in the buffer.")]
    ERROR_EAS_DIDNT_FIT,

    #[error("The extended attribute file on the mounted file system is corrupt.")]
    ERROR_EA_FILE_CORRUPT,

    #[error("The extended attribute table file is full.")]
    ERROR_EA_TABLE_FULL,

    #[error("The specified extended attribute handle is invalid.")]
    ERROR_INVALID_EA_HANDLE,

    #[error("The mounted file system does not support extended attributes.")]
    ERROR_EAS_NOT_SUPPORTED,

    #[error("Attempt to release mutex not owned by caller.")]
    ERROR_NOT_OWNER,

    #[error("Too many posts were made to a semaphore.")]
    ERROR_TOO_MANY_POSTS,

    #[error("Only part of a ReadProcessMemory or WriteProcessMemory request was completed.")]
    ERROR_PARTIAL_COPY,

    #[error("The oplock request is denied.")]
    ERROR_OPLOCK_NOT_GRANTED,

    #[error("An invalid oplock acknowledgment was received by the system.")]
    ERROR_INVALID_OPLOCK_PROTOCOL,

    #[error("The volume is too fragmented to complete this operation.")]
    ERROR_DISK_TOO_FRAGMENTED,

    #[error("The file cannot be opened because it is in the process of being deleted.")]
    ERROR_DELETE_PENDING,

    #[error(
        "Short name settings may not be changed on this volume due to the global registry setting."
    )]
    ERROR_INCOMPATIBLE_WITH_GLOBAL_SHORT_NAME_REGISTRY_SETTING,

    #[error("Short names are not enabled on this volume.")]
    ERROR_SHORT_NAMES_NOT_ENABLED_ON_VOLUME,

    #[error("The security stream for the given volume is in an inconsistent state. Please run CHKDSK on the volume.")]
    ERROR_SECURITY_STREAM_IS_INCONSISTENT,

    #[error("A requested file lock operation cannot be processed due to an invalid byte range.")]
    ERROR_INVALID_LOCK_RANGE,

    #[error("The subsystem needed to support the image type is not present.")]
    ERROR_IMAGE_SUBSYSTEM_NOT_PRESENT,

    #[error("The specified file already has a notification GUID associated with it.")]
    ERROR_NOTIFICATION_GUID_ALREADY_DEFINED,

    #[error("An invalid exception handler routine has been detected.")]
    ERROR_INVALID_EXCEPTION_HANDLER,

    #[error("Duplicate privileges were specified for the token.")]
    ERROR_DUPLICATE_PRIVILEGES,

    #[error("No ranges for the specified operation were able to be processed.")]
    ERROR_NO_RANGES_PROCESSED,

    #[error("Operation is not allowed on a file system internal file.")]
    ERROR_NOT_ALLOWED_ON_SYSTEM_FILE,

    #[error("The physical resources of this disk have been exhausted.")]
    ERROR_DISK_RESOURCES_EXHAUSTED,

    #[error("The token representing the data is invalid.")]
    ERROR_INVALID_TOKEN,

    #[error("The device does not support the command feature.")]
    ERROR_DEVICE_FEATURE_NOT_SUPPORTED,

    #[error(
        "The system cannot find message text for message number 0x%1 in the message file for %2."
    )]
    ERROR_MR_MID_NOT_FOUND,

    #[error("The scope specified was not found.")]
    ERROR_SCOPE_NOT_FOUND,

    #[error("The Central Access Policy specified is not defined on the target machine.")]
    ERROR_UNDEFINED_SCOPE,

    #[error("The Central Access Policy obtained from Active Directory is invalid.")]
    ERROR_INVALID_CAP,

    #[error("The device is unreachable.")]
    ERROR_DEVICE_UNREACHABLE,

    #[error("The target device has insufficient resources to complete the operation.")]
    ERROR_DEVICE_NO_RESOURCES,

    #[error("A data integrity checksum error occurred. Data in the file stream is corrupt.")]
    ERROR_DATA_CHECKSUM_ERROR,

    #[error("An attempt was made to modify both a KERNEL and normal Extended Attribute (EA) in the same operation.")]
    ERROR_INTERMIXED_KERNEL_EA_OPERATION,

    #[error("Device does not support file-level TRIM.")]
    ERROR_FILE_LEVEL_TRIM_NOT_SUPPORTED,

    #[error("The command specified a data offset that does not align to the device's granularity/alignment.")]
    ERROR_OFFSET_ALIGNMENT_VIOLATION,

    #[error("The command specified an invalid field in its parameter list.")]
    ERROR_INVALID_FIELD_IN_PARAMETER_LIST,

    #[error("An operation is currently in progress with the device.")]
    ERROR_OPERATION_IN_PROGRESS,

    #[error(
        "An attempt was made to send down the command via an invalid path to the target device."
    )]
    ERROR_BAD_DEVICE_PATH,

    #[error("The command specified a number of descriptors that exceeded the maximum supported by the device.")]
    ERROR_TOO_MANY_DESCRIPTORS,

    #[error("Scrub is disabled on the specified file.")]
    ERROR_SCRUB_DATA_DISABLED,

    #[error("The storage device does not provide redundancy.")]
    ERROR_NOT_REDUNDANT_STORAGE,

    #[error("An operation is not supported on a resident file.")]
    ERROR_RESIDENT_FILE_NOT_SUPPORTED,

    #[error("An operation is not supported on a compressed file.")]
    ERROR_COMPRESSED_FILE_NOT_SUPPORTED,

    #[error("An operation is not supported on a directory.")]
    ERROR_DIRECTORY_NOT_SUPPORTED,

    #[error("The specified copy of the requested data could not be read.")]
    ERROR_NOT_READ_FROM_COPY,

    #[error("No action was taken as a system reboot is required.")]
    ERROR_FAIL_NOACTION_REBOOT,

    #[error("The shutdown operation failed.")]
    ERROR_FAIL_SHUTDOWN,

    #[error("The restart operation failed.")]
    ERROR_FAIL_RESTART,

    #[error("The maximum number of sessions has been reached.")]
    ERROR_MAX_SESSIONS_REACHED,

    #[error("The thread is already in background processing mode.")]
    ERROR_THREAD_MODE_ALREADY_BACKGROUND,

    #[error("The thread is not in background processing mode.")]
    ERROR_THREAD_MODE_NOT_BACKGROUND,

    #[error("The process is already in background processing mode.")]
    ERROR_PROCESS_MODE_ALREADY_BACKGROUND,

    #[error("The process is not in background processing mode.")]
    ERROR_PROCESS_MODE_NOT_BACKGROUND,

    #[error("Attempt to access invalid address.")]
    ERROR_INVALID_ADDRESS,
}
