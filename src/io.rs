use libc::termios;

// David: there are some really good crates for error handling: 
// - anyhow/color-eyre for generic error (often used in binaries)
// - thiserror for custom errors (used in libs)
// These are so good they are consider standard practice afaik
#[derive(Debug)]
pub struct IoError(pub std::io::Error);


fn last_io_error() -> IoError {
    IoError(std::io::Error::last_os_error())
}

pub fn term_setup() -> Result<(), IoError> {
    // remove canonical mode for stdin in order to disable buffering and make symbols accessible immediately
    // David: didn't look in details but compiler wasn't happy, wrong argument
    let mut term: termios = termios { c_iflag: 0, c_oflag: 0, c_cflag: 0, c_lflag: 0, c_cc: [0 as libc::cc_t; libc::NCCS], c_ispeed: 0, c_ospeed: 0 };
    if unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut term as *mut termios) } != 0 {
        return Err(last_io_error());
    }
    term.c_lflag &= !libc::ICANON & !libc::ECHO;
    if unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &term as *const termios) } != 0 {
        return Err(last_io_error());
    }
    Ok(())
}

pub fn getc() -> Result<u8, IoError> {
    let mut buf = [0u8];
    let result = unsafe { libc::read(libc::STDIN_FILENO, buf.as_mut_ptr() as *mut libc::c_void, 1) };
    if result < 0 {
        return Err(last_io_error());
    }
    // David: I'm assuming the aim of this is for development - debug assert won't be compiled in release build.
    debug_assert!(result == 1);
    Ok(buf[0])
}

pub fn putc(c: u8) -> Result<(), IoError> {
    let buf = [c];
    let result = unsafe { libc::write(libc::STDOUT_FILENO, buf.as_ptr() as *const libc::c_void, 1) };
    if result < 0 {
        return Err(last_io_error());
    }
    assert!(result == 1);
    Ok(())
}

pub fn puts(buf: &[u8]) -> Result<(), IoError> {
    let mut current = buf;
    while !current.is_empty() {
        let result = unsafe { libc::write(libc::STDIN_FILENO, buf.as_ptr() as *const libc::c_void, buf.len()) };
        if result < 0 {
            return Err(last_io_error());
        }
        assert!(result > 0);
        current = &current[result as usize..];
    }
    Ok(())
}

pub fn hasc() -> Result<bool, IoError> {
    Ok(unsafe {
        let mut n: libc::c_int = 0;
        let result = libc::ioctl(libc::STDIN_FILENO, libc::FIONREAD, &mut n as *mut libc::c_int);
        if result < 0 {
            return Err(last_io_error());
        } else {
            n > 0
        }
    })
}
