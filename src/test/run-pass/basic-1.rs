// -*- rust -*-

use std;
import std::comm::chan;
import std::comm::port;
import std::comm::send;
import std::comm::recv;
import std::task;

fn a(c: chan<int>) { send(c, 10); }

fn main() {
    let p = port();
    task::spawn(chan(p), a);
    task::spawn(chan(p), a);
    let n: int = 0;
    n = recv(p);
    n = recv(p);
    //    log "Finished.";
}

fn b(c: chan<int>) {
    //    log "task b0";
    //    log "task b1";
    //    log "task b2";
    //    log "task b3";
    //    log "task b4";
    //    log "task b5";
    send(c, 10);
}
