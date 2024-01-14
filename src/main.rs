use std::{env, net::{IpAddr, TcpStream}, str::FromStr, u16, process::exit, sync::{mpsc::{channel, Sender}, Arc, atomic::AtomicUsize}, thread, time::Duration, io::{self, Write}};
use std::sync::atomic::Ordering;
use std::time::Instant;
use rayon::prelude::*;

const MAX: u16 = 65535;

#[allow(dead_code)]
struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}


impl Arguments{
    fn new(args:&[String]) -> Result<Arguments,&'static str>{
        if args.len()<2{
            return Err("not enough arguments!");
        }else if args.len()>4{
            return Err("too much arguments!");
        }
        let f= args[1].clone();
        if let Ok(ipaddr)=IpAddr::from_str(&f){
            return Ok(Arguments{flag:String::from(""),ipaddr,threads:2000});
        }else{
            let flag=args[1].clone();
            if flag.contains("-h")||flag.contains("-help")&&args.len()==2{
                println!("Usage: -j to select how many threads you want \r\n-h or -help to show this help message");
                return  Err("help")
            }else if flag.contains("-h")||flag.contains("-help"){
                return  Err("too many arguments!")
            }else if flag.contains("-j"){
                let ipaddr =match IpAddr::from_str(&args[3]){
                    Ok(s)=>s,
                    Err(_)=>return Err("not a vaild IPADDR")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s)=>s,
                    Err(_)=> return Err("failed to parse thread number!")

                };
                return Ok(Arguments{threads,flag,ipaddr});
            }else{
                return Err("invaild syntax");
            }
        }

}
}
fn scan(tx: Sender<u16>, scanned_ports: Arc<AtomicUsize>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port;
    loop {
        let socket_addr = format!("{}:{}", addr, port);
        if let Ok(_) = TcpStream::connect_timeout(&socket_addr.parse().unwrap(), Duration::new(1, 0)) {
            tx.send(port).unwrap();

        }
        scanned_ports.fetch_add(1, Ordering::SeqCst);
        if MAX - port <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    let start = Instant::now();
    let scanned_ports = Arc::new(AtomicUsize::new(0));
    let total_ports = MAX as usize;
    let progress_tracker = {
        let scanned_ports = Arc::clone(&scanned_ports);
        thread::spawn(move || {
            while scanned_ports.load(Ordering::SeqCst) < total_ports {
                let progress = 100 * scanned_ports.load(Ordering::SeqCst) / total_ports;
                let progress_bar_length = 50;
                let arrow_pos = progress * progress_bar_length / 100;
                let progress_bar = format!(
                    "{:=<arrow_pos$}>{:->progress_bar_remaining$}",
                    "", "",
                    arrow_pos = arrow_pos,
                    progress_bar_remaining = progress_bar_length - arrow_pos - 1
                );

                print!("\rProgress: [{progress_bar}] {}%", progress);
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_secs(1));
            }
            println!();
        })
    };
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, err);
            exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();

    (0..num_threads).into_par_iter().for_each(|i| {
        let tx = tx.clone();
        let addr_clone = addr;
        let scanned_ports_clone = Arc::clone(&scanned_ports); // 克隆 Arc
        thread::spawn(move || {
            scan(tx, scanned_ports_clone, i, addr_clone, num_threads); // 传递 scanned_ports_clone
        });
    });
    progress_tracker.join().unwrap();
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}