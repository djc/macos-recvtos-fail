use std::os::unix::io::AsRawFd;

fn main() {
    let std = std::net::UdpSocket::bind("0.0.0.0:12345").unwrap();
    let io = mio::net::UdpSocket::from_socket(std).unwrap();
    let addr = dbg!(io.local_addr().unwrap());

    let on: libc::c_int = 1;
    let rc = unsafe {
        libc::setsockopt(
            io.as_raw_fd(),
            libc::IPPROTO_IP,
            libc::IP_RECVTOS,
            &on as *const _ as _,
            std::mem::size_of_val(&on) as _,
        )
    };

    if rc == -1 {
        println!("error IP_RECVTOS {}", std::io::Error::last_os_error());
    }

    if !addr.is_ipv6() {
        return;
    }

    let on: libc::c_int = 1;
    let rc = unsafe {
        libc::setsockopt(
            io.as_raw_fd(),
            libc::IPPROTO_IPV6,
            libc::IPV6_RECVTCLASS,
            &on as *const _ as _,
            std::mem::size_of_val(&on) as _,
        )
    };

    if rc == -1 {
        println!("error IPV6_RECVTCLASS {}", std::io::Error::last_os_error());
    }
}
