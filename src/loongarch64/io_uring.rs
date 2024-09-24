/* automatically generated by rust-bindgen 0.70.1 */

pub type __s8 = crate::ctypes::c_schar;
pub type __u8 = crate::ctypes::c_uchar;
pub type __s16 = crate::ctypes::c_short;
pub type __u16 = crate::ctypes::c_ushort;
pub type __s32 = crate::ctypes::c_int;
pub type __u32 = crate::ctypes::c_uint;
pub type __s64 = crate::ctypes::c_longlong;
pub type __u64 = crate::ctypes::c_ulonglong;
pub type __kernel_key_t = crate::ctypes::c_int;
pub type __kernel_mqd_t = crate::ctypes::c_int;
pub type __kernel_long_t = crate::ctypes::c_long;
pub type __kernel_ulong_t = crate::ctypes::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = crate::ctypes::c_uint;
pub type __kernel_pid_t = crate::ctypes::c_int;
pub type __kernel_ipc_pid_t = crate::ctypes::c_int;
pub type __kernel_uid_t = crate::ctypes::c_uint;
pub type __kernel_gid_t = crate::ctypes::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = crate::ctypes::c_int;
pub type __kernel_uid32_t = crate::ctypes::c_uint;
pub type __kernel_gid32_t = crate::ctypes::c_uint;
pub type __kernel_old_uid_t = __kernel_uid_t;
pub type __kernel_old_gid_t = __kernel_gid_t;
pub type __kernel_old_dev_t = crate::ctypes::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = crate::ctypes::c_longlong;
pub type __kernel_old_time_t = __kernel_long_t;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = crate::ctypes::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = crate::ctypes::c_int;
pub type __kernel_clockid_t = crate::ctypes::c_int;
pub type __kernel_caddr_t = *mut crate::ctypes::c_char;
pub type __kernel_uid16_t = crate::ctypes::c_ushort;
pub type __kernel_gid16_t = crate::ctypes::c_ushort;
pub type __s128 = i128;
pub type __u128 = u128;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = crate::ctypes::c_uint;
pub type __kernel_rwf_t = crate::ctypes::c_int;
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
#[repr(C)]
pub struct __BindgenUnionField<T>(::core::marker::PhantomData<T>);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fscrypt_policy_v1 {
pub version: __u8,
pub contents_encryption_mode: __u8,
pub filenames_encryption_mode: __u8,
pub flags: __u8,
pub master_key_descriptor: [__u8; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fscrypt_key {
pub mode: __u32,
pub raw: [__u8; 64usize],
pub size: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fscrypt_policy_v2 {
pub version: __u8,
pub contents_encryption_mode: __u8,
pub filenames_encryption_mode: __u8,
pub flags: __u8,
pub log2_data_unit_size: __u8,
pub __reserved: [__u8; 3usize],
pub master_key_identifier: [__u8; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_get_policy_ex_arg {
pub policy_size: __u64,
pub policy: fscrypt_get_policy_ex_arg__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_key_specifier {
pub type_: __u32,
pub __reserved: __u32,
pub u: fscrypt_key_specifier__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug)]
pub struct fscrypt_provisioning_key_payload {
pub type_: __u32,
pub __reserved: __u32,
pub raw: __IncompleteArrayField<__u8>,
}
#[repr(C)]
pub struct fscrypt_add_key_arg {
pub key_spec: fscrypt_key_specifier,
pub raw_size: __u32,
pub key_id: __u32,
pub __reserved: [__u32; 8usize],
pub raw: __IncompleteArrayField<__u8>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_remove_key_arg {
pub key_spec: fscrypt_key_specifier,
pub removal_status_flags: __u32,
pub __reserved: [__u32; 5usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_get_key_status_arg {
pub key_spec: fscrypt_key_specifier,
pub __reserved: [__u32; 6usize],
pub status: __u32,
pub status_flags: __u32,
pub user_count: __u32,
pub __out_reserved: [__u32; 13usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mount_attr {
pub attr_set: __u64,
pub attr_clr: __u64,
pub propagation: __u64,
pub userns_fd: __u64,
}
#[repr(C)]
#[derive(Debug)]
pub struct statmount {
pub size: __u32,
pub mnt_opts: __u32,
pub mask: __u64,
pub sb_dev_major: __u32,
pub sb_dev_minor: __u32,
pub sb_magic: __u64,
pub sb_flags: __u32,
pub fs_type: __u32,
pub mnt_id: __u64,
pub mnt_parent_id: __u64,
pub mnt_id_old: __u32,
pub mnt_parent_id_old: __u32,
pub mnt_attr: __u64,
pub mnt_propagation: __u64,
pub mnt_peer_group: __u64,
pub mnt_master: __u64,
pub propagate_from: __u64,
pub mnt_root: __u32,
pub mnt_point: __u32,
pub mnt_ns_id: __u64,
pub __spare2: [__u64; 49usize],
pub str_: __IncompleteArrayField<crate::ctypes::c_char>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mnt_id_req {
pub size: __u32,
pub spare: __u32,
pub mnt_id: __u64,
pub param: __u64,
pub mnt_ns_id: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct file_clone_range {
pub src_fd: __s64,
pub src_offset: __u64,
pub src_length: __u64,
pub dest_offset: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fstrim_range {
pub start: __u64,
pub len: __u64,
pub minlen: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fsuuid2 {
pub len: __u8,
pub uuid: [__u8; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fs_sysfs_path {
pub len: __u8,
pub name: [__u8; 128usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct file_dedupe_range_info {
pub dest_fd: __s64,
pub dest_offset: __u64,
pub bytes_deduped: __u64,
pub status: __s32,
pub reserved: __u32,
}
#[repr(C)]
#[derive(Debug)]
pub struct file_dedupe_range {
pub src_offset: __u64,
pub src_length: __u64,
pub dest_count: __u16,
pub reserved1: __u16,
pub reserved2: __u32,
pub info: __IncompleteArrayField<file_dedupe_range_info>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct files_stat_struct {
pub nr_files: crate::ctypes::c_ulong,
pub nr_free_files: crate::ctypes::c_ulong,
pub max_files: crate::ctypes::c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct inodes_stat_t {
pub nr_inodes: crate::ctypes::c_long,
pub nr_unused: crate::ctypes::c_long,
pub dummy: [crate::ctypes::c_long; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fsxattr {
pub fsx_xflags: __u32,
pub fsx_extsize: __u32,
pub fsx_nextents: __u32,
pub fsx_projid: __u32,
pub fsx_cowextsize: __u32,
pub fsx_pad: [crate::ctypes::c_uchar; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct page_region {
pub start: __u64,
pub end: __u64,
pub categories: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pm_scan_arg {
pub size: __u64,
pub flags: __u64,
pub start: __u64,
pub end: __u64,
pub walk_end: __u64,
pub vec: __u64,
pub vec_len: __u64,
pub max_pages: __u64,
pub category_inverted: __u64,
pub category_mask: __u64,
pub category_anyof_mask: __u64,
pub return_mask: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct procmap_query {
pub size: __u64,
pub query_flags: __u64,
pub query_addr: __u64,
pub vma_start: __u64,
pub vma_end: __u64,
pub vma_flags: __u64,
pub vma_page_size: __u64,
pub vma_offset: __u64,
pub inode: __u64,
pub dev_major: __u32,
pub dev_minor: __u32,
pub vma_name_size: __u32,
pub build_id_size: __u32,
pub vma_name_addr: __u64,
pub build_id_addr: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_timespec {
pub tv_sec: __kernel_time64_t,
pub tv_nsec: crate::ctypes::c_longlong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_itimerspec {
pub it_interval: __kernel_timespec,
pub it_value: __kernel_timespec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_old_timeval {
pub tv_sec: __kernel_long_t,
pub tv_usec: __kernel_long_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_old_timespec {
pub tv_sec: __kernel_old_time_t,
pub tv_nsec: crate::ctypes::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_old_itimerval {
pub it_interval: __kernel_old_timeval,
pub it_value: __kernel_old_timeval,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_sock_timeval {
pub tv_sec: __s64,
pub tv_usec: __s64,
}
#[repr(C)]
pub struct io_uring_sqe {
pub opcode: __u8,
pub flags: __u8,
pub ioprio: __u16,
pub fd: __s32,
pub __bindgen_anon_1: io_uring_sqe__bindgen_ty_1,
pub __bindgen_anon_2: io_uring_sqe__bindgen_ty_2,
pub len: __u32,
pub __bindgen_anon_3: io_uring_sqe__bindgen_ty_3,
pub user_data: __u64,
pub __bindgen_anon_4: io_uring_sqe__bindgen_ty_4,
pub personality: __u16,
pub __bindgen_anon_5: io_uring_sqe__bindgen_ty_5,
pub __bindgen_anon_6: io_uring_sqe__bindgen_ty_6,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_sqe__bindgen_ty_1__bindgen_ty_1 {
pub cmd_op: __u32,
pub __pad1: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_sqe__bindgen_ty_2__bindgen_ty_1 {
pub level: __u32,
pub optname: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_sqe__bindgen_ty_5__bindgen_ty_1 {
pub addr_len: __u16,
pub __pad3: [__u16; 1usize],
}
#[repr(C)]
pub struct io_uring_sqe__bindgen_ty_6 {
pub __bindgen_anon_1: __BindgenUnionField<io_uring_sqe__bindgen_ty_6__bindgen_ty_1>,
pub optval: __BindgenUnionField<__u64>,
pub cmd: __BindgenUnionField<[__u8; 0usize]>,
pub bindgen_union_field: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_sqe__bindgen_ty_6__bindgen_ty_1 {
pub addr3: __u64,
pub __pad2: [__u64; 1usize],
}
#[repr(C)]
#[derive(Debug)]
pub struct io_uring_cqe {
pub user_data: __u64,
pub res: __s32,
pub flags: __u32,
pub big_cqe: __IncompleteArrayField<__u64>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_sqring_offsets {
pub head: __u32,
pub tail: __u32,
pub ring_mask: __u32,
pub ring_entries: __u32,
pub flags: __u32,
pub dropped: __u32,
pub array: __u32,
pub resv1: __u32,
pub user_addr: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_cqring_offsets {
pub head: __u32,
pub tail: __u32,
pub ring_mask: __u32,
pub ring_entries: __u32,
pub overflow: __u32,
pub cqes: __u32,
pub flags: __u32,
pub resv1: __u32,
pub user_addr: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_params {
pub sq_entries: __u32,
pub cq_entries: __u32,
pub flags: __u32,
pub sq_thread_cpu: __u32,
pub sq_thread_idle: __u32,
pub features: __u32,
pub wq_fd: __u32,
pub resv: [__u32; 3usize],
pub sq_off: io_sqring_offsets,
pub cq_off: io_cqring_offsets,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_files_update {
pub offset: __u32,
pub resv: __u32,
pub fds: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_rsrc_register {
pub nr: __u32,
pub flags: __u32,
pub resv2: __u64,
pub data: __u64,
pub tags: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_rsrc_update {
pub offset: __u32,
pub resv: __u32,
pub data: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_rsrc_update2 {
pub offset: __u32,
pub resv: __u32,
pub data: __u64,
pub tags: __u64,
pub nr: __u32,
pub resv2: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_probe_op {
pub op: __u8,
pub resv: __u8,
pub flags: __u16,
pub resv2: __u32,
}
#[repr(C)]
#[derive(Debug)]
pub struct io_uring_probe {
pub last_op: __u8,
pub ops_len: __u8,
pub resv: __u16,
pub resv2: [__u32; 3usize],
pub ops: __IncompleteArrayField<io_uring_probe_op>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_restriction {
pub opcode: __u16,
pub __bindgen_anon_1: io_uring_restriction__bindgen_ty_1,
pub resv: __u8,
pub resv2: [__u32; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_buf {
pub addr: __u64,
pub len: __u32,
pub bid: __u16,
pub resv: __u16,
}
#[repr(C)]
pub struct io_uring_buf_ring {
pub __bindgen_anon_1: io_uring_buf_ring__bindgen_ty_1,
}
#[repr(C)]
pub struct io_uring_buf_ring__bindgen_ty_1 {
pub __bindgen_anon_1: __BindgenUnionField<io_uring_buf_ring__bindgen_ty_1__bindgen_ty_1>,
pub __bindgen_anon_2: __BindgenUnionField<io_uring_buf_ring__bindgen_ty_1__bindgen_ty_2>,
pub bindgen_union_field: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_buf_ring__bindgen_ty_1__bindgen_ty_1 {
pub resv1: __u64,
pub resv2: __u32,
pub resv3: __u16,
pub tail: __u16,
}
#[repr(C)]
#[derive(Debug)]
pub struct io_uring_buf_ring__bindgen_ty_1__bindgen_ty_2 {
pub __empty_bufs: io_uring_buf_ring__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1,
pub bufs: __IncompleteArrayField<io_uring_buf>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_buf_ring__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_buf_reg {
pub ring_addr: __u64,
pub ring_entries: __u32,
pub bgid: __u16,
pub flags: __u16,
pub resv: [__u64; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_buf_status {
pub buf_group: __u32,
pub head: __u32,
pub resv: [__u32; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_napi {
pub busy_poll_to: __u32,
pub prefer_busy_poll: __u8,
pub pad: [__u8; 3usize],
pub resv: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_getevents_arg {
pub sigmask: __u64,
pub sigmask_sz: __u32,
pub pad: __u32,
pub ts: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_sync_cancel_reg {
pub addr: __u64,
pub fd: __s32,
pub flags: __u32,
pub timeout: __kernel_timespec,
pub opcode: __u8,
pub pad: [__u8; 7usize],
pub pad2: [__u64; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_file_index_range {
pub off: __u32,
pub len: __u32,
pub resv: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct io_uring_recvmsg_out {
pub namelen: __u32,
pub controllen: __u32,
pub payloadlen: __u32,
pub flags: __u32,
}
pub const NR_OPEN: u32 = 1024;
pub const NGROUPS_MAX: u32 = 65536;
pub const ARG_MAX: u32 = 131072;
pub const LINK_MAX: u32 = 127;
pub const MAX_CANON: u32 = 255;
pub const MAX_INPUT: u32 = 255;
pub const NAME_MAX: u32 = 255;
pub const PATH_MAX: u32 = 4096;
pub const PIPE_BUF: u32 = 4096;
pub const XATTR_NAME_MAX: u32 = 255;
pub const XATTR_SIZE_MAX: u32 = 65536;
pub const XATTR_LIST_MAX: u32 = 65536;
pub const RTSIG_MAX: u32 = 32;
pub const _IOC_NRBITS: u32 = 8;
pub const _IOC_TYPEBITS: u32 = 8;
pub const _IOC_SIZEBITS: u32 = 14;
pub const _IOC_DIRBITS: u32 = 2;
pub const _IOC_NRMASK: u32 = 255;
pub const _IOC_TYPEMASK: u32 = 255;
pub const _IOC_SIZEMASK: u32 = 16383;
pub const _IOC_DIRMASK: u32 = 3;
pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = 8;
pub const _IOC_SIZESHIFT: u32 = 16;
pub const _IOC_DIRSHIFT: u32 = 30;
pub const _IOC_NONE: u32 = 0;
pub const _IOC_WRITE: u32 = 1;
pub const _IOC_READ: u32 = 2;
pub const IOC_IN: u32 = 1073741824;
pub const IOC_OUT: u32 = 2147483648;
pub const IOC_INOUT: u32 = 3221225472;
pub const IOCSIZE_MASK: u32 = 1073676288;
pub const IOCSIZE_SHIFT: u32 = 16;
pub const __BITS_PER_LONG_LONG: u32 = 64;
pub const FSCRYPT_POLICY_FLAGS_PAD_4: u32 = 0;
pub const FSCRYPT_POLICY_FLAGS_PAD_8: u32 = 1;
pub const FSCRYPT_POLICY_FLAGS_PAD_16: u32 = 2;
pub const FSCRYPT_POLICY_FLAGS_PAD_32: u32 = 3;
pub const FSCRYPT_POLICY_FLAGS_PAD_MASK: u32 = 3;
pub const FSCRYPT_POLICY_FLAG_DIRECT_KEY: u32 = 4;
pub const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64: u32 = 8;
pub const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32: u32 = 16;
pub const FSCRYPT_MODE_AES_256_XTS: u32 = 1;
pub const FSCRYPT_MODE_AES_256_CTS: u32 = 4;
pub const FSCRYPT_MODE_AES_128_CBC: u32 = 5;
pub const FSCRYPT_MODE_AES_128_CTS: u32 = 6;
pub const FSCRYPT_MODE_SM4_XTS: u32 = 7;
pub const FSCRYPT_MODE_SM4_CTS: u32 = 8;
pub const FSCRYPT_MODE_ADIANTUM: u32 = 9;
pub const FSCRYPT_MODE_AES_256_HCTR2: u32 = 10;
pub const FSCRYPT_POLICY_V1: u32 = 0;
pub const FSCRYPT_KEY_DESCRIPTOR_SIZE: u32 = 8;
pub const FSCRYPT_KEY_DESC_PREFIX: &[u8; 9] = b"fscrypt:\0";
pub const FSCRYPT_KEY_DESC_PREFIX_SIZE: u32 = 8;
pub const FSCRYPT_MAX_KEY_SIZE: u32 = 64;
pub const FSCRYPT_POLICY_V2: u32 = 2;
pub const FSCRYPT_KEY_IDENTIFIER_SIZE: u32 = 16;
pub const FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR: u32 = 1;
pub const FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER: u32 = 2;
pub const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY: u32 = 1;
pub const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS: u32 = 2;
pub const FSCRYPT_KEY_STATUS_ABSENT: u32 = 1;
pub const FSCRYPT_KEY_STATUS_PRESENT: u32 = 2;
pub const FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED: u32 = 3;
pub const FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF: u32 = 1;
pub const FS_KEY_DESCRIPTOR_SIZE: u32 = 8;
pub const FS_POLICY_FLAGS_PAD_4: u32 = 0;
pub const FS_POLICY_FLAGS_PAD_8: u32 = 1;
pub const FS_POLICY_FLAGS_PAD_16: u32 = 2;
pub const FS_POLICY_FLAGS_PAD_32: u32 = 3;
pub const FS_POLICY_FLAGS_PAD_MASK: u32 = 3;
pub const FS_POLICY_FLAG_DIRECT_KEY: u32 = 4;
pub const FS_POLICY_FLAGS_VALID: u32 = 7;
pub const FS_ENCRYPTION_MODE_INVALID: u32 = 0;
pub const FS_ENCRYPTION_MODE_AES_256_XTS: u32 = 1;
pub const FS_ENCRYPTION_MODE_AES_256_GCM: u32 = 2;
pub const FS_ENCRYPTION_MODE_AES_256_CBC: u32 = 3;
pub const FS_ENCRYPTION_MODE_AES_256_CTS: u32 = 4;
pub const FS_ENCRYPTION_MODE_AES_128_CBC: u32 = 5;
pub const FS_ENCRYPTION_MODE_AES_128_CTS: u32 = 6;
pub const FS_ENCRYPTION_MODE_ADIANTUM: u32 = 9;
pub const FS_KEY_DESC_PREFIX: &[u8; 9] = b"fscrypt:\0";
pub const FS_KEY_DESC_PREFIX_SIZE: u32 = 8;
pub const FS_MAX_KEY_SIZE: u32 = 64;
pub const MS_RDONLY: u32 = 1;
pub const MS_NOSUID: u32 = 2;
pub const MS_NODEV: u32 = 4;
pub const MS_NOEXEC: u32 = 8;
pub const MS_SYNCHRONOUS: u32 = 16;
pub const MS_REMOUNT: u32 = 32;
pub const MS_MANDLOCK: u32 = 64;
pub const MS_DIRSYNC: u32 = 128;
pub const MS_NOSYMFOLLOW: u32 = 256;
pub const MS_NOATIME: u32 = 1024;
pub const MS_NODIRATIME: u32 = 2048;
pub const MS_BIND: u32 = 4096;
pub const MS_MOVE: u32 = 8192;
pub const MS_REC: u32 = 16384;
pub const MS_VERBOSE: u32 = 32768;
pub const MS_SILENT: u32 = 32768;
pub const MS_POSIXACL: u32 = 65536;
pub const MS_UNBINDABLE: u32 = 131072;
pub const MS_PRIVATE: u32 = 262144;
pub const MS_SLAVE: u32 = 524288;
pub const MS_SHARED: u32 = 1048576;
pub const MS_RELATIME: u32 = 2097152;
pub const MS_KERNMOUNT: u32 = 4194304;
pub const MS_I_VERSION: u32 = 8388608;
pub const MS_STRICTATIME: u32 = 16777216;
pub const MS_LAZYTIME: u32 = 33554432;
pub const MS_SUBMOUNT: u32 = 67108864;
pub const MS_NOREMOTELOCK: u32 = 134217728;
pub const MS_NOSEC: u32 = 268435456;
pub const MS_BORN: u32 = 536870912;
pub const MS_ACTIVE: u32 = 1073741824;
pub const MS_NOUSER: u32 = 2147483648;
pub const MS_RMT_MASK: u32 = 41943121;
pub const MS_MGC_VAL: u32 = 3236757504;
pub const MS_MGC_MSK: u32 = 4294901760;
pub const OPEN_TREE_CLONE: u32 = 1;
pub const MOVE_MOUNT_F_SYMLINKS: u32 = 1;
pub const MOVE_MOUNT_F_AUTOMOUNTS: u32 = 2;
pub const MOVE_MOUNT_F_EMPTY_PATH: u32 = 4;
pub const MOVE_MOUNT_T_SYMLINKS: u32 = 16;
pub const MOVE_MOUNT_T_AUTOMOUNTS: u32 = 32;
pub const MOVE_MOUNT_T_EMPTY_PATH: u32 = 64;
pub const MOVE_MOUNT_SET_GROUP: u32 = 256;
pub const MOVE_MOUNT_BENEATH: u32 = 512;
pub const MOVE_MOUNT__MASK: u32 = 887;
pub const FSOPEN_CLOEXEC: u32 = 1;
pub const FSPICK_CLOEXEC: u32 = 1;
pub const FSPICK_SYMLINK_NOFOLLOW: u32 = 2;
pub const FSPICK_NO_AUTOMOUNT: u32 = 4;
pub const FSPICK_EMPTY_PATH: u32 = 8;
pub const FSMOUNT_CLOEXEC: u32 = 1;
pub const MOUNT_ATTR_RDONLY: u32 = 1;
pub const MOUNT_ATTR_NOSUID: u32 = 2;
pub const MOUNT_ATTR_NODEV: u32 = 4;
pub const MOUNT_ATTR_NOEXEC: u32 = 8;
pub const MOUNT_ATTR__ATIME: u32 = 112;
pub const MOUNT_ATTR_RELATIME: u32 = 0;
pub const MOUNT_ATTR_NOATIME: u32 = 16;
pub const MOUNT_ATTR_STRICTATIME: u32 = 32;
pub const MOUNT_ATTR_NODIRATIME: u32 = 128;
pub const MOUNT_ATTR_IDMAP: u32 = 1048576;
pub const MOUNT_ATTR_NOSYMFOLLOW: u32 = 2097152;
pub const MOUNT_ATTR_SIZE_VER0: u32 = 32;
pub const MNT_ID_REQ_SIZE_VER0: u32 = 24;
pub const MNT_ID_REQ_SIZE_VER1: u32 = 32;
pub const STATMOUNT_SB_BASIC: u32 = 1;
pub const STATMOUNT_MNT_BASIC: u32 = 2;
pub const STATMOUNT_PROPAGATE_FROM: u32 = 4;
pub const STATMOUNT_MNT_ROOT: u32 = 8;
pub const STATMOUNT_MNT_POINT: u32 = 16;
pub const STATMOUNT_FS_TYPE: u32 = 32;
pub const STATMOUNT_MNT_NS_ID: u32 = 64;
pub const STATMOUNT_MNT_OPTS: u32 = 128;
pub const LSMT_ROOT: i32 = -1;
pub const LISTMOUNT_REVERSE: u32 = 1;
pub const INR_OPEN_CUR: u32 = 1024;
pub const INR_OPEN_MAX: u32 = 4096;
pub const BLOCK_SIZE_BITS: u32 = 10;
pub const BLOCK_SIZE: u32 = 1024;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const SEEK_DATA: u32 = 3;
pub const SEEK_HOLE: u32 = 4;
pub const SEEK_MAX: u32 = 4;
pub const RENAME_NOREPLACE: u32 = 1;
pub const RENAME_EXCHANGE: u32 = 2;
pub const RENAME_WHITEOUT: u32 = 4;
pub const FILE_DEDUPE_RANGE_SAME: u32 = 0;
pub const FILE_DEDUPE_RANGE_DIFFERS: u32 = 1;
pub const NR_FILE: u32 = 8192;
pub const FS_XFLAG_REALTIME: u32 = 1;
pub const FS_XFLAG_PREALLOC: u32 = 2;
pub const FS_XFLAG_IMMUTABLE: u32 = 8;
pub const FS_XFLAG_APPEND: u32 = 16;
pub const FS_XFLAG_SYNC: u32 = 32;
pub const FS_XFLAG_NOATIME: u32 = 64;
pub const FS_XFLAG_NODUMP: u32 = 128;
pub const FS_XFLAG_RTINHERIT: u32 = 256;
pub const FS_XFLAG_PROJINHERIT: u32 = 512;
pub const FS_XFLAG_NOSYMLINKS: u32 = 1024;
pub const FS_XFLAG_EXTSIZE: u32 = 2048;
pub const FS_XFLAG_EXTSZINHERIT: u32 = 4096;
pub const FS_XFLAG_NODEFRAG: u32 = 8192;
pub const FS_XFLAG_FILESTREAM: u32 = 16384;
pub const FS_XFLAG_DAX: u32 = 32768;
pub const FS_XFLAG_COWEXTSIZE: u32 = 65536;
pub const FS_XFLAG_HASATTR: u32 = 2147483648;
pub const BMAP_IOCTL: u32 = 1;
pub const FSLABEL_MAX: u32 = 256;
pub const FS_SECRM_FL: u32 = 1;
pub const FS_UNRM_FL: u32 = 2;
pub const FS_COMPR_FL: u32 = 4;
pub const FS_SYNC_FL: u32 = 8;
pub const FS_IMMUTABLE_FL: u32 = 16;
pub const FS_APPEND_FL: u32 = 32;
pub const FS_NODUMP_FL: u32 = 64;
pub const FS_NOATIME_FL: u32 = 128;
pub const FS_DIRTY_FL: u32 = 256;
pub const FS_COMPRBLK_FL: u32 = 512;
pub const FS_NOCOMP_FL: u32 = 1024;
pub const FS_ENCRYPT_FL: u32 = 2048;
pub const FS_BTREE_FL: u32 = 4096;
pub const FS_INDEX_FL: u32 = 4096;
pub const FS_IMAGIC_FL: u32 = 8192;
pub const FS_JOURNAL_DATA_FL: u32 = 16384;
pub const FS_NOTAIL_FL: u32 = 32768;
pub const FS_DIRSYNC_FL: u32 = 65536;
pub const FS_TOPDIR_FL: u32 = 131072;
pub const FS_HUGE_FILE_FL: u32 = 262144;
pub const FS_EXTENT_FL: u32 = 524288;
pub const FS_VERITY_FL: u32 = 1048576;
pub const FS_EA_INODE_FL: u32 = 2097152;
pub const FS_EOFBLOCKS_FL: u32 = 4194304;
pub const FS_NOCOW_FL: u32 = 8388608;
pub const FS_DAX_FL: u32 = 33554432;
pub const FS_INLINE_DATA_FL: u32 = 268435456;
pub const FS_PROJINHERIT_FL: u32 = 536870912;
pub const FS_CASEFOLD_FL: u32 = 1073741824;
pub const FS_RESERVED_FL: u32 = 2147483648;
pub const FS_FL_USER_VISIBLE: u32 = 253951;
pub const FS_FL_USER_MODIFIABLE: u32 = 229631;
pub const SYNC_FILE_RANGE_WAIT_BEFORE: u32 = 1;
pub const SYNC_FILE_RANGE_WRITE: u32 = 2;
pub const SYNC_FILE_RANGE_WAIT_AFTER: u32 = 4;
pub const SYNC_FILE_RANGE_WRITE_AND_WAIT: u32 = 7;
pub const PROCFS_IOCTL_MAGIC: u8 = 102u8;
pub const PAGE_IS_WPALLOWED: u32 = 1;
pub const PAGE_IS_WRITTEN: u32 = 2;
pub const PAGE_IS_FILE: u32 = 4;
pub const PAGE_IS_PRESENT: u32 = 8;
pub const PAGE_IS_SWAPPED: u32 = 16;
pub const PAGE_IS_PFNZERO: u32 = 32;
pub const PAGE_IS_HUGE: u32 = 64;
pub const PAGE_IS_SOFT_DIRTY: u32 = 128;
pub const PM_SCAN_WP_MATCHING: u32 = 1;
pub const PM_SCAN_CHECK_WPASYNC: u32 = 2;
pub const IORING_FILE_INDEX_ALLOC: i32 = -1;
pub const IORING_SETUP_IOPOLL: u32 = 1;
pub const IORING_SETUP_SQPOLL: u32 = 2;
pub const IORING_SETUP_SQ_AFF: u32 = 4;
pub const IORING_SETUP_CQSIZE: u32 = 8;
pub const IORING_SETUP_CLAMP: u32 = 16;
pub const IORING_SETUP_ATTACH_WQ: u32 = 32;
pub const IORING_SETUP_R_DISABLED: u32 = 64;
pub const IORING_SETUP_SUBMIT_ALL: u32 = 128;
pub const IORING_SETUP_COOP_TASKRUN: u32 = 256;
pub const IORING_SETUP_TASKRUN_FLAG: u32 = 512;
pub const IORING_SETUP_SQE128: u32 = 1024;
pub const IORING_SETUP_CQE32: u32 = 2048;
pub const IORING_SETUP_SINGLE_ISSUER: u32 = 4096;
pub const IORING_SETUP_DEFER_TASKRUN: u32 = 8192;
pub const IORING_SETUP_NO_MMAP: u32 = 16384;
pub const IORING_SETUP_REGISTERED_FD_ONLY: u32 = 32768;
pub const IORING_SETUP_NO_SQARRAY: u32 = 65536;
pub const IORING_URING_CMD_FIXED: u32 = 1;
pub const IORING_URING_CMD_MASK: u32 = 1;
pub const IORING_FSYNC_DATASYNC: u32 = 1;
pub const IORING_TIMEOUT_ABS: u32 = 1;
pub const IORING_TIMEOUT_UPDATE: u32 = 2;
pub const IORING_TIMEOUT_BOOTTIME: u32 = 4;
pub const IORING_TIMEOUT_REALTIME: u32 = 8;
pub const IORING_LINK_TIMEOUT_UPDATE: u32 = 16;
pub const IORING_TIMEOUT_ETIME_SUCCESS: u32 = 32;
pub const IORING_TIMEOUT_MULTISHOT: u32 = 64;
pub const IORING_TIMEOUT_CLOCK_MASK: u32 = 12;
pub const IORING_TIMEOUT_UPDATE_MASK: u32 = 18;
pub const SPLICE_F_FD_IN_FIXED: u32 = 2147483648;
pub const IORING_POLL_ADD_MULTI: u32 = 1;
pub const IORING_POLL_UPDATE_EVENTS: u32 = 2;
pub const IORING_POLL_UPDATE_USER_DATA: u32 = 4;
pub const IORING_POLL_ADD_LEVEL: u32 = 8;
pub const IORING_ASYNC_CANCEL_ALL: u32 = 1;
pub const IORING_ASYNC_CANCEL_FD: u32 = 2;
pub const IORING_ASYNC_CANCEL_ANY: u32 = 4;
pub const IORING_ASYNC_CANCEL_FD_FIXED: u32 = 8;
pub const IORING_ASYNC_CANCEL_USERDATA: u32 = 16;
pub const IORING_ASYNC_CANCEL_OP: u32 = 32;
pub const IORING_RECVSEND_POLL_FIRST: u32 = 1;
pub const IORING_RECV_MULTISHOT: u32 = 2;
pub const IORING_RECVSEND_FIXED_BUF: u32 = 4;
pub const IORING_SEND_ZC_REPORT_USAGE: u32 = 8;
pub const IORING_RECVSEND_BUNDLE: u32 = 16;
pub const IORING_NOTIF_USAGE_ZC_COPIED: u32 = 2147483648;
pub const IORING_ACCEPT_MULTISHOT: u32 = 1;
pub const IORING_ACCEPT_DONTWAIT: u32 = 2;
pub const IORING_ACCEPT_POLL_FIRST: u32 = 4;
pub const IORING_MSG_RING_CQE_SKIP: u32 = 1;
pub const IORING_MSG_RING_FLAGS_PASS: u32 = 2;
pub const IORING_FIXED_FD_NO_CLOEXEC: u32 = 1;
pub const IORING_NOP_INJECT_RESULT: u32 = 1;
pub const IORING_CQE_F_BUFFER: u32 = 1;
pub const IORING_CQE_F_MORE: u32 = 2;
pub const IORING_CQE_F_SOCK_NONEMPTY: u32 = 4;
pub const IORING_CQE_F_NOTIF: u32 = 8;
pub const IORING_CQE_BUFFER_SHIFT: u32 = 16;
pub const IORING_OFF_SQ_RING: u32 = 0;
pub const IORING_OFF_CQ_RING: u32 = 134217728;
pub const IORING_OFF_SQES: u32 = 268435456;
pub const IORING_OFF_PBUF_RING: u32 = 2147483648;
pub const IORING_OFF_PBUF_SHIFT: u32 = 16;
pub const IORING_OFF_MMAP_MASK: u32 = 4160749568;
pub const IORING_SQ_NEED_WAKEUP: u32 = 1;
pub const IORING_SQ_CQ_OVERFLOW: u32 = 2;
pub const IORING_SQ_TASKRUN: u32 = 4;
pub const IORING_CQ_EVENTFD_DISABLED: u32 = 1;
pub const IORING_ENTER_GETEVENTS: u32 = 1;
pub const IORING_ENTER_SQ_WAKEUP: u32 = 2;
pub const IORING_ENTER_SQ_WAIT: u32 = 4;
pub const IORING_ENTER_EXT_ARG: u32 = 8;
pub const IORING_ENTER_REGISTERED_RING: u32 = 16;
pub const IORING_FEAT_SINGLE_MMAP: u32 = 1;
pub const IORING_FEAT_NODROP: u32 = 2;
pub const IORING_FEAT_SUBMIT_STABLE: u32 = 4;
pub const IORING_FEAT_RW_CUR_POS: u32 = 8;
pub const IORING_FEAT_CUR_PERSONALITY: u32 = 16;
pub const IORING_FEAT_FAST_POLL: u32 = 32;
pub const IORING_FEAT_POLL_32BITS: u32 = 64;
pub const IORING_FEAT_SQPOLL_NONFIXED: u32 = 128;
pub const IORING_FEAT_EXT_ARG: u32 = 256;
pub const IORING_FEAT_NATIVE_WORKERS: u32 = 512;
pub const IORING_FEAT_RSRC_TAGS: u32 = 1024;
pub const IORING_FEAT_CQE_SKIP: u32 = 2048;
pub const IORING_FEAT_LINKED_FILE: u32 = 4096;
pub const IORING_FEAT_REG_REG_RING: u32 = 8192;
pub const IORING_FEAT_RECVSEND_BUNDLE: u32 = 16384;
pub const IORING_RSRC_REGISTER_SPARSE: u32 = 1;
pub const IORING_REGISTER_FILES_SKIP: i32 = -2;
pub const IO_URING_OP_SUPPORTED: u32 = 1;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum fsconfig_command {
FSCONFIG_SET_FLAG = 0,
FSCONFIG_SET_STRING = 1,
FSCONFIG_SET_BINARY = 2,
FSCONFIG_SET_PATH = 3,
FSCONFIG_SET_PATH_EMPTY = 4,
FSCONFIG_SET_FD = 5,
FSCONFIG_CMD_CREATE = 6,
FSCONFIG_CMD_RECONFIGURE = 7,
FSCONFIG_CMD_CREATE_EXCL = 8,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum procmap_query_flags {
PROCMAP_QUERY_VMA_READABLE = 1,
PROCMAP_QUERY_VMA_WRITABLE = 2,
PROCMAP_QUERY_VMA_EXECUTABLE = 4,
PROCMAP_QUERY_VMA_SHARED = 8,
PROCMAP_QUERY_COVERING_OR_NEXT_VMA = 16,
PROCMAP_QUERY_FILE_BACKED_VMA = 32,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum io_uring_sqe_flags_bit {
IOSQE_FIXED_FILE_BIT = 0,
IOSQE_IO_DRAIN_BIT = 1,
IOSQE_IO_LINK_BIT = 2,
IOSQE_IO_HARDLINK_BIT = 3,
IOSQE_ASYNC_BIT = 4,
IOSQE_BUFFER_SELECT_BIT = 5,
IOSQE_CQE_SKIP_SUCCESS_BIT = 6,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum io_uring_op {
IORING_OP_NOP = 0,
IORING_OP_READV = 1,
IORING_OP_WRITEV = 2,
IORING_OP_FSYNC = 3,
IORING_OP_READ_FIXED = 4,
IORING_OP_WRITE_FIXED = 5,
IORING_OP_POLL_ADD = 6,
IORING_OP_POLL_REMOVE = 7,
IORING_OP_SYNC_FILE_RANGE = 8,
IORING_OP_SENDMSG = 9,
IORING_OP_RECVMSG = 10,
IORING_OP_TIMEOUT = 11,
IORING_OP_TIMEOUT_REMOVE = 12,
IORING_OP_ACCEPT = 13,
IORING_OP_ASYNC_CANCEL = 14,
IORING_OP_LINK_TIMEOUT = 15,
IORING_OP_CONNECT = 16,
IORING_OP_FALLOCATE = 17,
IORING_OP_OPENAT = 18,
IORING_OP_CLOSE = 19,
IORING_OP_FILES_UPDATE = 20,
IORING_OP_STATX = 21,
IORING_OP_READ = 22,
IORING_OP_WRITE = 23,
IORING_OP_FADVISE = 24,
IORING_OP_MADVISE = 25,
IORING_OP_SEND = 26,
IORING_OP_RECV = 27,
IORING_OP_OPENAT2 = 28,
IORING_OP_EPOLL_CTL = 29,
IORING_OP_SPLICE = 30,
IORING_OP_PROVIDE_BUFFERS = 31,
IORING_OP_REMOVE_BUFFERS = 32,
IORING_OP_TEE = 33,
IORING_OP_SHUTDOWN = 34,
IORING_OP_RENAMEAT = 35,
IORING_OP_UNLINKAT = 36,
IORING_OP_MKDIRAT = 37,
IORING_OP_SYMLINKAT = 38,
IORING_OP_LINKAT = 39,
IORING_OP_MSG_RING = 40,
IORING_OP_FSETXATTR = 41,
IORING_OP_SETXATTR = 42,
IORING_OP_FGETXATTR = 43,
IORING_OP_GETXATTR = 44,
IORING_OP_SOCKET = 45,
IORING_OP_URING_CMD = 46,
IORING_OP_SEND_ZC = 47,
IORING_OP_SENDMSG_ZC = 48,
IORING_OP_READ_MULTISHOT = 49,
IORING_OP_WAITID = 50,
IORING_OP_FUTEX_WAIT = 51,
IORING_OP_FUTEX_WAKE = 52,
IORING_OP_FUTEX_WAITV = 53,
IORING_OP_FIXED_FD_INSTALL = 54,
IORING_OP_FTRUNCATE = 55,
IORING_OP_BIND = 56,
IORING_OP_LISTEN = 57,
IORING_OP_LAST = 58,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum io_uring_msg_ring_flags {
IORING_MSG_DATA = 0,
IORING_MSG_SEND_FD = 1,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum io_uring_register_op {
IORING_REGISTER_BUFFERS = 0,
IORING_UNREGISTER_BUFFERS = 1,
IORING_REGISTER_FILES = 2,
IORING_UNREGISTER_FILES = 3,
IORING_REGISTER_EVENTFD = 4,
IORING_UNREGISTER_EVENTFD = 5,
IORING_REGISTER_FILES_UPDATE = 6,
IORING_REGISTER_EVENTFD_ASYNC = 7,
IORING_REGISTER_PROBE = 8,
IORING_REGISTER_PERSONALITY = 9,
IORING_UNREGISTER_PERSONALITY = 10,
IORING_REGISTER_RESTRICTIONS = 11,
IORING_REGISTER_ENABLE_RINGS = 12,
IORING_REGISTER_FILES2 = 13,
IORING_REGISTER_FILES_UPDATE2 = 14,
IORING_REGISTER_BUFFERS2 = 15,
IORING_REGISTER_BUFFERS_UPDATE = 16,
IORING_REGISTER_IOWQ_AFF = 17,
IORING_UNREGISTER_IOWQ_AFF = 18,
IORING_REGISTER_IOWQ_MAX_WORKERS = 19,
IORING_REGISTER_RING_FDS = 20,
IORING_UNREGISTER_RING_FDS = 21,
IORING_REGISTER_PBUF_RING = 22,
IORING_UNREGISTER_PBUF_RING = 23,
IORING_REGISTER_SYNC_CANCEL = 24,
IORING_REGISTER_FILE_ALLOC_RANGE = 25,
IORING_REGISTER_PBUF_STATUS = 26,
IORING_REGISTER_NAPI = 27,
IORING_UNREGISTER_NAPI = 28,
IORING_REGISTER_LAST = 29,
IORING_REGISTER_USE_REGISTERED_RING = 2147483648,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum io_wq_type {
IO_WQ_BOUND = 0,
IO_WQ_UNBOUND = 1,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum io_uring_register_pbuf_ring_flags {
IOU_PBUF_RING_MMAP = 1,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum io_uring_register_restriction_op {
IORING_RESTRICTION_REGISTER_OP = 0,
IORING_RESTRICTION_SQE_OP = 1,
IORING_RESTRICTION_SQE_FLAGS_ALLOWED = 2,
IORING_RESTRICTION_SQE_FLAGS_REQUIRED = 3,
IORING_RESTRICTION_LAST = 4,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum io_uring_socket_op {
SOCKET_URING_OP_SIOCINQ = 0,
SOCKET_URING_OP_SIOCOUTQ = 1,
SOCKET_URING_OP_GETSOCKOPT = 2,
SOCKET_URING_OP_SETSOCKOPT = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union fscrypt_get_policy_ex_arg__bindgen_ty_1 {
pub version: __u8,
pub v1: fscrypt_policy_v1,
pub v2: fscrypt_policy_v2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union fscrypt_key_specifier__bindgen_ty_1 {
pub __reserved: [__u8; 32usize],
pub descriptor: [__u8; 8usize],
pub identifier: [__u8; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_1 {
pub off: __u64,
pub addr2: __u64,
pub __bindgen_anon_1: io_uring_sqe__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_2 {
pub addr: __u64,
pub splice_off_in: __u64,
pub __bindgen_anon_1: io_uring_sqe__bindgen_ty_2__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_3 {
pub rw_flags: __kernel_rwf_t,
pub fsync_flags: __u32,
pub poll_events: __u16,
pub poll32_events: __u32,
pub sync_range_flags: __u32,
pub msg_flags: __u32,
pub timeout_flags: __u32,
pub accept_flags: __u32,
pub cancel_flags: __u32,
pub open_flags: __u32,
pub statx_flags: __u32,
pub fadvise_advice: __u32,
pub splice_flags: __u32,
pub rename_flags: __u32,
pub unlink_flags: __u32,
pub hardlink_flags: __u32,
pub xattr_flags: __u32,
pub msg_ring_flags: __u32,
pub uring_cmd_flags: __u32,
pub waitid_flags: __u32,
pub futex_flags: __u32,
pub install_fd_flags: __u32,
pub nop_flags: __u32,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_4 {
pub buf_index: __u16,
pub buf_group: __u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_5 {
pub splice_fd_in: __s32,
pub file_index: __u32,
pub optlen: __u32,
pub __bindgen_anon_1: io_uring_sqe__bindgen_ty_5__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_restriction__bindgen_ty_1 {
pub register_op: __u8,
pub sqe_op: __u8,
pub sqe_flags: __u8,
}
impl<T> __IncompleteArrayField<T> {
#[inline]
pub const fn new() -> Self {
__IncompleteArrayField(::core::marker::PhantomData, [])
}
#[inline]
pub fn as_ptr(&self) -> *const T {
self as *const _ as *const T
}
#[inline]
pub fn as_mut_ptr(&mut self) -> *mut T {
self as *mut _ as *mut T
}
#[inline]
pub unsafe fn as_slice(&self, len: usize) -> &[T] {
::core::slice::from_raw_parts(self.as_ptr(), len)
}
#[inline]
pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
}
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
fmt.write_str("__IncompleteArrayField")
}
}
impl<T> __BindgenUnionField<T> {
#[inline]
pub const fn new() -> Self {
__BindgenUnionField(::core::marker::PhantomData)
}
#[inline]
pub unsafe fn as_ref(&self) -> &T {
::core::mem::transmute(self)
}
#[inline]
pub unsafe fn as_mut(&mut self) -> &mut T {
::core::mem::transmute(self)
}
}
impl<T> ::core::default::Default for __BindgenUnionField<T> {
#[inline]
fn default() -> Self {
Self::new()
}
}
impl<T> ::core::clone::Clone for __BindgenUnionField<T> {
#[inline]
fn clone(&self) -> Self {
*self
}
}
impl<T> ::core::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::core::fmt::Debug for __BindgenUnionField<T> {
fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
fmt.write_str("__BindgenUnionField")
}
}
impl<T> ::core::hash::Hash for __BindgenUnionField<T> {
fn hash<H: ::core::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::core::cmp::PartialEq for __BindgenUnionField<T> {
fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
true
}
}
impl<T> ::core::cmp::Eq for __BindgenUnionField<T> {}
