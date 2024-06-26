use std::fs::File;
use crate::commands::CrosEcCmd;
use crate::EcCmdResult;
use crate::EcError;
use nix::ioctl_readwrite;
use num_traits::FromPrimitive;
use std::os::unix::io::AsRawFd;

use super::EcResponseStatus;

pub const IN_SIZE: usize = 256;
pub const BUF_SIZE: usize = IN_SIZE - 8;

#[repr(C)]
struct _CrosEcCommandV2 {
    version: u32,
    command: u32,
    outsize: u32,
    insize: u32,
    result: u32,
    data: [u8; 0],
}

#[repr(C)]
struct CrosEcCommandV2 {
    version: u32,
    command: CrosEcCmd,
    outsize: u32,
    insize: u32,
    result: u32,
    data: [u8; IN_SIZE],
}

const CROS_EC_IOC_MAGIC: u8 = 0xEC;
ioctl_readwrite!(cros_ec_cmd, CROS_EC_IOC_MAGIC, 0, _CrosEcCommandV2);

pub fn dev_ec_command(command: CrosEcCmd, command_version: u8, data: &[u8], dev_path: &str) -> EcCmdResult<Vec<u8>> {

    let size = std::cmp::min(IN_SIZE, data.len());

    let mut cmd = CrosEcCommandV2 {
        version: command_version as u32,
        command,
        outsize: size as u32,
        insize: IN_SIZE as u32,
        result: 0xFF,
        data: [0; IN_SIZE],
    };

    cmd.data[0..size].copy_from_slice(data);
    let cmd_ptr = &mut cmd as *mut _ as *mut _CrosEcCommandV2;

    let cros_ec_fd = File::open(dev_path).unwrap();
    let fildes = cros_ec_fd.as_raw_fd();

    let result = unsafe { cros_ec_cmd(fildes, cmd_ptr) };
    let status =
        FromPrimitive::from_u32(cmd.result).ok_or(EcError::UnknownResponseCode(cmd.result))?;
    let EcResponseStatus::Success = status else {
        return Err(EcError::Response(status));
    };
    result
        .map(|result| cmd.data[0..result as usize].to_vec())
        .map_err(|err| EcError::DeviceError(err))
}
