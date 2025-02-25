use std::thread::sleep;
use std::time::Duration;

use osafe::io::posix_print::Print;
use osafe::io::Printable;
use osafe::ipc::Communicate;
use osafe::multiprocessing::posix_process::Process;
use osafe::ipc::posix_udp::UdpSocket;

const TEST_DATA:i32 = 4444;
const PORT:u16 = 8080;

fn subprocess()
{
    let udp_recvr = UdpSocket::bind(PORT+1).unwrap();
    let start = udp_recvr.recv::<i32>().unwrap();
    Print::printstrln(&format!("Recvd Test Data: {}", start)).unwrap();
    let udp_sender = UdpSocket::new("127.0.0.1".to_string(), PORT).unwrap();
    udp_sender.send(TEST_DATA).unwrap();
}

#[test]
fn test_ipc()
{
    let _subprocess = match Process::run(subprocess).unwrap()
    {
        Some(subprocess) => subprocess,
        None => return,
    };
    sleep(Duration::from_millis(100));
    let udp_sender = UdpSocket::new("127.0.0.1".to_string(), PORT+1).unwrap();
    let udp_recvr = UdpSocket::bind(PORT).unwrap();
    udp_sender.send(TEST_DATA).unwrap();
    let resp = udp_recvr.recv::<i32>().unwrap();
    assert_eq!(resp, TEST_DATA);
}
