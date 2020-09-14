/////////////////////////////////////////////////////////////
// rust_comm_processing::test2.rs - test send/recv         //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 12 Sep 2020  //
/////////////////////////////////////////////////////////////

#![allow(unused_imports)]
use std::net::{TcpListener, TcpStream};

use rust_traits::*;
use rust_comm_processing::*;
use rust_message::*;
use rust_comm_logger::*;
use std::sync::*;
use std::io::*;

type Log = MuteLog;

fn handle_client(stream: &TcpStream) -> std::io::Result<()> {
    let mut buf_writer = BufWriter::new(stream.try_clone()?);
    let mut buf_reader = BufReader::new(stream.try_clone()?);

    let mut msg = Message::new(MSG_SIZE);
    let rslt:Result<()> = CommProcessing::<Log>::buf_recv_message(&mut buf_reader, &mut msg);
    if rslt.is_err() {
        print!("\n  recv_message error");
        let err = std::io::Error::new(ErrorKind::Other, "recv error");
        return Err(err);
    }
    else {
        print!("\n  receiver received msg");
        msg.show_message(8);
        CommProcessing::<Log>::buf_send_message(&msg, &mut buf_writer)?;
    }
    Ok(())
}
fn start_listener(end_point: &str) -> std::io::Result<()> {
    let tcpl = TcpListener::bind(end_point)?;
    for stream in tcpl.incoming() {
        print!("\n  listener accepted connection");
        let rslt = handle_client(&stream?);
        if rslt.is_err() {
            print!("\n  error in handle_client");
            let _ = std::io::stdout().flush();
        }
        break;  // only one connection for testing
    }
    Ok(())
}
fn construction(addr: &'static str) -> Result<()> {
    let mut msg = Message::new(MSG_SIZE);
    msg.set_content_str("test message");
    let _cp = CommProcessing::<VerboseLog>::new();
    let addr_copy = addr;
    let handle = std::thread::spawn(move || {
        let rslt = start_listener(addr_copy);
        if rslt.is_err() {
            print!("\n  failed to start listener on {:?}",addr);
            let _ = std::io::stdout().flush();
            // let error = std::io::Error::new(ErrorKind::Other, "start failed");
            // return Err(error);
        }
        rslt
    });

    Log::write("\n  sending msg");
    msg.set_type(MessageType::FLUSH as u8);
    let stream = TcpStream::connect(addr)?;
    let mut buf_writer = BufWriter::new(stream.try_clone()?);
    let mut buf_reader = BufReader::new(stream.try_clone()?);
    CommProcessing::<Log>::buf_send_message(&msg, &mut buf_writer)?;
    Log::write(&format!("\n  sent message with len: {:?}",msg.len()));
    let _ = std::io::stdout().flush();

    CommProcessing::<Log>::buf_recv_message(&mut buf_reader, &mut msg)?;
    print!("\n  connector received reply msg");
    let _ = std::io::stdout().flush();

    msg.show_message(8);
    let s = msg.get_content_str().unwrap();
    print!("\n  message content: {:?}",s);

    msg.set_type(MessageType::QUIT as u8);
    CommProcessing::<Log>::buf_send_message(&msg, &mut buf_writer)?;
    CommProcessing::<Log>::buf_recv_message(&mut buf_reader, &mut msg)?;
    msg.show_message(8);
    let _ = handle.join();
    Ok(())
}

fn main() {

    print!("\n  -- test2 : FixedSizeMsg - ver2\n");
    show_msg_size();

    let addr: &'static str = "127.0.0.1:8083";
    let rslt = construction(addr);
    if rslt.is_err() {
        print!("\n  listener start on {:?} failed", addr);
    }

    print!("\n  That's all Folks!\n\n");
}