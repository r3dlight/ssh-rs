use ssh_rs::ssh;
use std::net::{TcpStream, ToSocketAddrs};

fn main() {
    ssh::enable_log();

    let bio = MyProxy::new("127.0.0.1:22");

    let mut session = ssh::create_session()
        .username("ubuntu")
        .password("password")
        .private_key_path("./id_rsa")
        .connect_bio(bio)
        .unwrap()
        .run_local();
    let exec = session.open_exec().unwrap();
    let vec: Vec<u8> = exec.send_command("ls -all").unwrap();
    println!("{}", String::from_utf8(vec).unwrap());
    // Close session.
    session.close();
}

// Use a real ssh server since I don't wanna implement a ssh-server in the example codes
struct MyProxy {
    server: TcpStream,
}

impl MyProxy {
    fn new<A>(addr: A) -> Self
    where
        A: ToSocketAddrs,
    {
        Self {
            server: TcpStream::connect(addr).unwrap(),
        }
    }
}

impl std::io::Read for MyProxy {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        println!("bio log: read {} bytes", buf.len());
        self.server.read(buf)
    }
}

impl std::io::Write for MyProxy {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        println!("bio log: write {} bytes", buf.len());
        self.server.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.server.flush()
    }
}
