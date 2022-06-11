use std::os::unix::io::AsRawFd;
nix::ioctl_write_ptr!(set_ch, 0x8d, 0x01, Freq);
nix::ioctl_none!(start_rec, 0x8d, 0x02);
nix::ioctl_none!(stop_rec, 0x8d, 0x03);
nix::ioctl_read!(ptx_get_cnr, 0x8d, 0x04, i32);
nix::ioctl_write_int!(ptx_enable_lnb, 0x8d, 0x05);
nix::ioctl_none!(ptx_disable_lnb, 0x8d, 0x06);
nix::ioctl_write_int!(ptx_set_sys_mode, 0x8d, 0x0b);

#[repr(C)]
pub struct Freq {
    pub ch: i32,
    pub slot: i32,
}

fn main() {
    let path = std::path::Path::new("/dev/px4video2");

    let freq = Freq {
        ch: 68,
        slot: 0,
    };

    let mut output = 0i32;
    let f = std::fs::OpenOptions::new().read(true).open(path).unwrap();
    let _errno = unsafe { set_ch(f.as_raw_fd(), &freq) }.unwrap();
    let _errno = unsafe { ptx_get_cnr(f.as_raw_fd(), &mut output) }.unwrap();
}
