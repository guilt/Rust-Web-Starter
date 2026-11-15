//! Minimal std-only server for the rust9x / Windows 9x legacy target.
//!
//! Rocket and tokio's `net` feature both hit a pre-main CRT/std issue on XP,
//! while std + Winsock works. This bin implements the same two routes with a
//! blocking thread-per-connection server.

#[cfg(target_family = "rust9x")]
mod win9x {
    use core::ffi::{c_int, c_void};

    type Bool = c_int;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn InitializeCriticalSection(cs: *mut c_void);
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "system" fn rust9x_initialize_critical_section_ex(
        cs: *mut c_void,
        _spin_count: u32,
        _flags: u32,
    ) -> Bool {
        if cs.is_null() {
            return 0;
        }
        unsafe { InitializeCriticalSection(cs) };
        1
    }

    pub fn keep_shims() {
        let _ = core::hint::black_box(
            rust9x_initialize_critical_section_ex
                as unsafe extern "system" fn(*mut c_void, u32, u32) -> Bool as usize,
        );
    }
}

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];
    let n = match stream.read(&mut buf) {
        Ok(n) if n > 0 => n,
        _ => return,
    };
    let request = String::from_utf8_lossy(&buf[..n]);
    let path = request.split_whitespace().nth(1).unwrap_or("/");

    let (status, body) = if path == "/" {
        ("200 OK", "Hello, world! Hit /hello/Potato".to_string())
    } else if let Some(name) = path.strip_prefix("/hello/") {
        ("200 OK", format!("Hello {name}!"))
    } else {
        ("404 Not Found", "Not Found".to_string())
    };

    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(response.as_bytes());
}

fn main() -> std::io::Result<()> {
    #[cfg(target_family = "rust9x")]
    win9x::keep_shims();
    let port = std::env::var("ROCKET_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000u16);
    let address = std::env::var("ROCKET_ADDRESS").unwrap_or_else(|_| "127.0.0.1".into());
    let listener = TcpListener::bind((address.as_str(), port))?;
    eprintln!("legacy server listening on {address}:{port}");

    for stream in listener.incoming().flatten() {
        std::thread::spawn(move || handle(stream));
    }
    Ok(())
}
