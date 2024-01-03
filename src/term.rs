use libc::termios;
pub fn setup() {
    // remove canonical mode for stdin in order to disable buffering and make symbols accessible immediately
    let mut term: termios = termios { c_iflag: 0, c_oflag: 0, c_cflag: 0, c_lflag: 0, c_line: 0, c_cc: [0 as libc::cc_t; libc::NCCS], c_ispeed: 0, c_ospeed: 0 };
    assert!(unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut term as *mut termios) } == 0);
    term.c_lflag &= !libc::ICANON & !libc::ECHO;
    assert!(unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &term as *const termios) } == 0);
}

pub fn getc() -> u8 {
    let mut buf = [0 as u8];
    assert!(unsafe { libc::read(libc::STDIN_FILENO, buf.as_mut_ptr() as *mut libc::c_void, 1) } == 1);
    return buf[0];
}

pub fn putc(c: u8) {
    let buf = [c];
    assert!(unsafe { libc::write(libc::STDOUT_FILENO, buf.as_ptr() as *const libc::c_void, 1) } == 1);
}

pub fn puts(buf: &[u8]) {
    assert!(unsafe { libc::write(libc::STDIN_FILENO, buf.as_ptr() as *const libc::c_void, buf.len()) } as usize == buf.len());
}

pub fn hasc() -> bool {
    return unsafe {
        let mut n: libc::c_int = 0;
        assert!(libc::ioctl(libc::STDIN_FILENO, libc::FIONREAD, &mut n as *mut libc::c_int) != -1);
        n
    } > 0;
}
