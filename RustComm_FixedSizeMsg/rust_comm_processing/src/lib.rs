/////////////////////////////////////////////////////////////
// rust_comm_processing::lib.rs - Application Specific     //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 19 Jul 2020  //
/////////////////////////////////////////////////////////////
/*
   CommProcessing<L>:
   - defines send_message, recv_message, and process_message
   - each of these needs to be tailored to the specifics of
     the Message class
*/

#![allow(unused_imports)]
#![allow(dead_code)]

/*-- RustComm facilities --*/
use rust_traits::*;
use rust_message::*;
//use rust_blocking_queue::*;
use rust_comm_logger::*;

/*-- std library facilities --*/
use std::fmt::*;
use std::net::{TcpStream};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Write};

type M = Message;

/*---------------------------------------------------------
  CommProcessing<L> 
  - defines application specific processing for the
    appliczation's message type
  - L is a logger type the must implement the Logger trait
*/
#[derive(Debug, Copy, Clone, Default)]
pub struct CommProcessing<L>
where L: Logger + Debug + Copy + Clone + Default {
    log: L,
}
impl<L> CommProcessing<L>
where L: Logger + Debug + Copy + Clone + Default
{
    pub fn new() -> CommProcessing<L> {
        CommProcessing {
            log: L::default(),
        }
    }
}
impl<M,L> Sndr<M> for CommProcessing<L>
where 
    M: Msg + Clone + Send + Default,
    L: Logger + Debug + Copy + Clone + Default
{
    fn send_message(msg: &M, stream: &mut TcpStream) -> std::io::Result<()>
    {
        L::write(&format!("\n  msg.len(): {}", msg.len()));
        stream.write(&msg.get_ref())?;
        Ok(())
    }
    fn buf_send_message(msg: &M, stream: &mut BufWriter<TcpStream>) -> std::io::Result<()>
    {
        L::write(&format!("\n  msg.len(): {}", msg.len()));
        stream.write(&msg.get_ref())?;
        let msg_type = msg.get_type(); 
        if msg_type == MessageType::FLUSH as u8 
            || msg_type == MessageType::END as u8 
            || msg_type == MessageType::QUIT as u8 
        {
            L::write("\n  flushing stream");
            let _ = stream.flush();
        }
        Ok(())
    }
}
impl<M,L> Rcvr<M> for CommProcessing<L>
where 
    M: Msg + Clone + Send + Default,
    L: Logger + Debug + Copy + Clone + Default
{
    /*-- reads message and enques in supplied BlockingQueue<M> --*/
    fn recv_message(stream: &mut TcpStream, msg_ref: &mut M) -> std::io::Result<()> 
    {
        L::write("\n  attempting to receive msg in commProc");
        L::write(&format!("\n  msg.len() in commProc: {}", msg_ref.len()));
        stream.read_exact(&mut msg_ref.get_mut_ref())?;
        Ok(())
    }
    /*-- same as above but uses buffered reader --*/
    fn buf_recv_message(stream: &mut BufReader<TcpStream>, msg_ref: &mut M) -> std::io::Result<()> 
    {
        L::write("\n  attempting to receive msg in commProc");
        L::write(&format!("\n  msg.len() in commProc: {}", msg_ref.len()));
        stream.read_exact(&mut msg_ref.get_mut_ref())?;
        Ok(())
    }
}
/*---------------------------------------------------------
  Process<M> handles processing of each message on 
  Listener<P,L>
*/
impl<M,L> Process<M> for CommProcessing<L>
where 
    M: Msg + Clone + Send + Default,
    L: Logger + Debug + Copy + Clone + Default
{
    fn process_message(msg: &mut M) 
    {
        L::write("\n--entered process_message--");
        let msg_type = msg.get_type();
        if msg_type != MessageType::FLUSH as u8 
            && msg_type != MessageType::END as u8 
            && msg_type != MessageType::QUIT as u8 
        {
            msg.set_type(MessageType::REPLY as u8);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construction() {
        let msg = Message::new(MSG_SIZE);
        let _cp = CommProcessing::<MuteLog>::default();
        let addr = "127.0.0.1:8080";
        let _lstnr = std::net::TcpListener::bind(addr);
        let mut stream = std::net::TcpStream::connect(addr).unwrap();
        let _ = CommProcessing::<MuteLog>::send_message(&msg, &mut stream);
        assert_eq!(2 + 2, 4);
    }
}
