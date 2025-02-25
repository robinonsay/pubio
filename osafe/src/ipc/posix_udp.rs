use core::{ffi::{self}, mem::MaybeUninit};

use alloc::{format, string::{String, ToString}};

use crate::{error::{ErrNo, Error}, posix::{__errno_location, __socket_type_SOCK_DGRAM, bind, close, htons, in_addr, inet_aton, poll, pollfd, recv, sendto, sockaddr, sockaddr_in, socket, AF_INET, POLLIN, POLLOUT}};

use super::Communicate;


pub struct UdpSocket
{
    fd: ffi::c_int,
    pub addr: Option<String>,
    port: u16
}

impl UdpSocket
{
    fn init_socket(&mut self) -> Result<(), Error>
    {
        self.fd = unsafe{socket(AF_INET as i32,__socket_type_SOCK_DGRAM as i32, 0)};
        if self.fd == -1
        {
            let errno = unsafe{*__errno_location()};
            return Err(Error::IpcError(String::from_errno(errno)))
        }
        return Ok(());
    }

    pub fn new(addr: String, port:u16) -> Result<Self, Error>
    {
        let mut socket = Self{
            fd: 0,
            addr: Some(addr),
            port: port
        };
        socket.init_socket()?;
        return Ok(socket);
    }

    pub fn bind(port:u16) -> Result<Self, Error>
    {
        let mut socket = Self{
            fd: 0,
            addr: None,
            port: port
        };
        socket.init_socket()?;
        let saddr = sockaddr_in
        {
            sin_family: AF_INET as u16,
            sin_addr: crate::posix::in_addr { s_addr: 0},
            sin_port: unsafe{htons(socket.port)},
            sin_zero: [0;8]
        };
        let ret = unsafe {
            bind(socket.fd, &saddr as *const sockaddr_in as *const sockaddr, size_of::<sockaddr_in>() as u32)
        };
        if ret == -1
        {
            let errno = unsafe{*__errno_location()};
            return Err(Error::IpcError(String::from_errno(errno)));
        }
        return Ok(socket)        
    }
}

impl Drop for UdpSocket
{
    fn drop(&mut self) {
        unsafe
        {
            close(self.fd);
        }
        self.fd = -1;
    }
}

impl Communicate for UdpSocket
{
    fn send<T: Send>(&self, data: T) -> Result<(), Error> {
        // Get the size of the data to be sent
        let data_size = size_of::<T>();
        // Get a pointer to the data
        let data_addr = &data as *const T as *const ffi::c_void;
        // Initialize address structure
        let mut addr = in_addr{
            s_addr: 0
        };
        // Get the address string and ensure it exists
        let mut addr_str = self.addr.clone()
        .ok_or(Error::IpcError(format!("Address is none")))?;
        // Add null terminator for C string compatibility
        addr_str.push(0 as char);
        // Convert address string to C string
        let addr_cstr = ffi::CStr::from_bytes_with_nul(addr_str.as_bytes())
        .map_err(|e| Error::IpcError(e.to_string()))?;
        // Convert string address to network address
        let ret = unsafe { inet_aton(addr_cstr.as_ptr(), &mut addr as *mut in_addr) };
        if ret == 0
        {
            return Err(Error::IpcError(format!("Invalid address {}", addr_str)));
        }
        // Set up socket address structure
        let saddr = sockaddr_in
        {
            sin_family: AF_INET as u16,
            sin_addr: addr,
            sin_port: unsafe{htons(self.port)},
            sin_zero: [0;8]
        };
        // Send the data
        let ret = unsafe
        {
            sendto(
                self.fd,
                data_addr,
                data_size,
                0,
                &saddr as *const sockaddr_in as *const sockaddr,
                size_of::<sockaddr_in>() as u32
            )
        };
        // Check for errors
        if ret == -1
        {
            let errno = unsafe{*__errno_location()};
            return Err(Error::IpcError(String::from_errno(errno)));
        }
        else if ret != data_size as isize {
            return Err(Error::IpcError(format!("Failed to write all bytes")));
        }
        return Ok(());
    }

    fn recv<T: Send>(&self) -> Result<T, Error> {
        let data_size = size_of::<T>();
        let mut buffer = MaybeUninit::<T>::uninit();
        let ret = unsafe{
            recv(self.fd, buffer.as_mut_ptr() as *mut ffi::c_void, data_size, 0)
        };
        if ret == -1
        {
            let errno = unsafe{*__errno_location()};
            return Err(Error::IpcError(String::from_errno(errno)))
        }
        else if ret != data_size as isize {
            return Err(Error::IpcError(format!("Failed to read all bytes")));
        }
        return Ok(unsafe{buffer.assume_init()});
    }
    
    fn try_send<T: Send>(&self, data: T, timeout_ms:i32) -> Result<(), Error> {
        let mut pfd = pollfd
        {
            fd: self.fd,
            events: POLLOUT as i16,
            revents: 0
        };
        let ret = unsafe{poll(&mut pfd as *mut pollfd, 1, timeout_ms)};
        if ret > 0
        {
            return self.send(data);
        }
        else
        {
            let mut msg = "Send timeout".to_string();
            if ret == -1
            {
                let errno = unsafe {
                    *__errno_location()
                };
                msg = String::from_errno(errno);
            }
            return Err(Error::IpcError(msg));
        }
    }
    
    fn try_recv<T: Send>(&self, timeout_ms: i32) -> Result<Option<T>, Error> {
        let mut pfd = pollfd
        {
            fd: self.fd,
            events: POLLIN as i16,
            revents: 0
        };
        let ret = unsafe{poll(&mut pfd as *mut pollfd, 1, timeout_ms)};
        if ret > 0
        {
            let data = self.recv::<T>()?;
            return Ok(Some(data));
        }
        else if ret == 0
        {
            return Ok(None);
        }
        else
        {
            let errno = unsafe {
                *__errno_location()
            };
            return Err(Error::IpcError(String::from_errno(errno)));
        }
    }
    
}
