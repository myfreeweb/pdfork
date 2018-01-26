extern crate libc;

#[cfg(target_os = "freebsd")]
use std::os::unix::io::RawFd;

#[derive(Debug)]
pub struct ChildHandle {
    pub child_pid: libc::pid_t,
    #[cfg(target_os = "freebsd")]
    pub child_pd: RawFd,
}

impl ChildHandle {
    #[cfg(target_os = "freebsd")]
    pub fn signal(&self, sig: libc::c_int) -> bool {
        unsafe { libc::pdkill(self.child_pd, sig) == 0 }
    }

    #[cfg(not(target_os = "freebsd"))]
    pub fn signal(&self, sig: libc::c_int) -> bool {
        unsafe { libc::kill(self.child_pid, sig) == 0 }
    }
}

#[cfg(target_os = "freebsd")]
impl Drop for ChildHandle {
    fn drop(&mut self) {
        unsafe { libc::close(self.child_pd) };
    }
}

pub enum ForkResult {
    Parent(ChildHandle),
    Child,
    Fail,
}

#[cfg(target_os = "freebsd")]
pub fn fork() -> ForkResult {
    let mut child_pd = -1;
    let child_pid = unsafe { libc::pdfork(&mut child_pd, 0) };
    if child_pid < 0 {
        ForkResult::Fail
    } else if child_pid > 0 {
        ForkResult::Parent(ChildHandle {
            child_pid,
            child_pd,
        })
    } else {
        ForkResult::Child
    }
}

#[cfg(not(target_os = "freebsd"))]
pub fn fork() -> ForkResult {
    let child_pid = unsafe { libc::fork() };
    if child_pid < 0 {
        ForkResult::Fail
    } else if child_pid > 0 {
        ForkResult::Parent(ChildHandle {
            child_pid,
        })
    } else {
        ForkResult::Child
    }
}
