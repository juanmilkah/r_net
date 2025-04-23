use std::{
    env,
    io::{Read, Result as IOResult, Write},
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs, UdpSocket},
};

fn run_tcp_client<T: ToSocketAddrs>(addr: T) -> IOResult<()> {
    let mut stream = TcpStream::connect(addr)?;

    let mut buf = Vec::with_capacity(1024);
    stream.read_to_end(&mut buf)?;

    let message = String::from_utf8_lossy(&buf);
    println!("RECEIVED: {:?}", message);
    Ok(())
}

fn run_udp_client<T: ToSocketAddrs>(server_addr: T, client_addr: T) -> IOResult<()> {
    let socket = UdpSocket::bind(client_addr)?;

    let message = "Hello, from udp client".as_bytes();
    socket.send_to(&message, server_addr)?;

    let mut buf = [0u8; 1504];
    let (len, src) = socket.recv_from(&mut buf)?;
    println!("Received {} bytes from {}", len, src);

    let message = String::from_utf8_lossy(&buf[..len]);
    println!("MESSAGE: {}", message);
    Ok(())
}

fn run_tcp_server<T: ToSocketAddrs>(addr: T) -> IOResult<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let message = "Hello, from server".as_bytes();
        stream.write_all(message)?;
    }
    Ok(())
}

fn run_udp_server<T: ToSocketAddrs>(addr: T) -> IOResult<()> {
    let socket = UdpSocket::bind(addr)?;
    println!("Server listening on port 8080");

    let mut data = [0u8; 1024];
    loop {
        let (len, src) = socket.recv_from(&mut data)?;
        println!("Received {} bytes from {}", len, src);

        let message = "Hello, from udp server".as_bytes();
        let sent = socket.send_to(&message, src)?;
        println!("Sent {} bytes back to {}", sent, src);
    }
}

fn main() -> IOResult<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        eprintln!("Provide a Client or Server!");
        return Ok(());
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let udp_client = SocketAddr::from(([127, 0, 0, 1], 9090));

    match args[1].as_str() {
        "client" => match args[2].as_str() {
            "tcp" => run_tcp_client(addr)?,
            "udp" => run_udp_client(addr, udp_client)?,
            _ => eprintln!("Invalid client type"),
        },
        "server" => match args[2].as_str() {
            "tcp" => run_tcp_server(addr)?,
            "udp" => run_udp_server(addr)?,
            _ => eprintln!("Invalid server type"),
        },
        _ => eprintln!("Invalid argument"),
    };

    Ok(())
}
