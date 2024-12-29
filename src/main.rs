use std::io::{ErrorKind, Read, Write};
use std::process::exit;
use std::sync::mpsc::channel;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    if ! std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let (mut response_tx, response_rx) = channel::<Vec<u8>>();
    std::thread::Builder::new().name("Stdouter".to_owned()).spawn(move || {
        while let Ok(response) = response_rx.recv() {
            let response_str = String::from_utf8_lossy(&response);
            let mut stdout = std::io::stdout().lock();
            for line in response_str.lines() {
                let line = line.trim();

                //eprintln!("{line}");
                let line_len = line.len();
                let rgb = [
                    u8::from_str_radix(&line[line_len-6..line_len - 4], 16).unwrap(),
                    u8::from_str_radix(&line[line_len-4..line_len - 2], 16).unwrap(),
                    u8::from_str_radix(&line[line_len-2..line_len - 0], 16).unwrap(),
                ];
                stdout.write_all(&rgb).unwrap();
            }
            stdout.flush().unwrap();
        }
        eprintln!("Stdouter stopped!");
        exit(1);
    })?;

    let mut conn = std::net::TcpStream::connect("wall.c3pixelflut.de:1337")?;
    //let mut conn = std::net::TcpStream::connect("localhost:1337")?;
    //conn.set_nonblocking(true)?;
    conn.set_nodelay(true)?;
    let width = 3840;
    let height = 1080;
    eprintln!("Connected");
    let mut did_spawn_reader = false;

    loop {
        let mut request = String::with_capacity("PX 1000 1000\n".len() * width * height);
        let mut response_size = 0;

        for y in 0..height {
            for x in 0..width {
                let line = format!("PX {x} {y}\n");
                request.push_str(&line);
                response_size += line.len() + " rrggbb".len();
            }
        }

        if ! did_spawn_reader {
            let mut conn = conn.try_clone()?;
            let response_tx = response_tx.clone();
            std::thread::Builder::new().name("Reader".to_owned()).spawn(move || {
                loop {
                    let mut response = vec![0; response_size];
                    conn.read_exact(&mut response).unwrap();
                    response_tx.send(response).unwrap();
                }
            })?;
            did_spawn_reader = true;
            eprintln!("Spawned reader!");
        }


        let request = request.as_bytes();
        eprintln!("Req is {}", request.len());
        conn.write_all(&request)?;
        conn.flush()?;
        /*let mut request_pos = 0;
        let mut response = vec![0; response_size];
        let mut response_pos = 0;

        while request_pos < request.len() || response_pos < response.len() {
            //eprintln!("1 {request_pos}");
            let written = match conn.write(&request[request_pos..]) {
                Ok(bytes) => bytes,
                Err(err) => match err.kind() {
                    ErrorKind::WouldBlock => 0,
                    _ => return Err(err.into()),
                }
            };
            //eprintln!("2 {written}");
            request_pos += written;
            conn.flush()?;

            //eprintln!("3 {response_pos}");
            let read = match conn.read(&mut response[response_pos..]) {
                Ok(bytes) => bytes,
                Err(err) => match err.kind() {
                    ErrorKind::WouldBlock => 0,
                    _ => return Err(err.into()),
                }
            };
            //eprintln!("4 {read}");
            response_pos += read;

            /*if written > 0 || read > 0 {
                eprintln!("Wrote {written} and read {read}, total written {request_pos}/{} and read {response_pos}/{}", request.len(), response.len());
                std::thread::sleep(std::time::Duration::from_millis(100));
            }*/

        }

        let mut test = [0u8; 1];
        if let Ok(_) = conn.read_exact(&mut test) {
            panic!("Got too much data!");
        }

        //conn.write_all(request.as_bytes())?;
        //conn.flush()?;
        eprintln!("Wrote {}", request.len());

        //conn.read_exact(&mut response)?;
        eprintln!("Read {} bytes", response.len());
        response_tx.send(response)?;*/
    }
}
