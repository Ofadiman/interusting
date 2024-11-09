use std::{error::Error, net::IpAddr};

use cidr::IpCidr;
use clap::Parser;
use tokio::runtime::Runtime;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long)]
    cidr: IpCidr,

    #[arg(short = 's', long, default_value = "0")]
    port_start: u16,

    #[arg(short = 'e', long, default_value = "1024")]
    port_end: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    if args.port_start > args.port_end {
        panic!("port_start cannot be greater than port_end")
    }

    let (sender, mut receiver) = tokio::sync::mpsc::channel::<(IpAddr, u16)>(10);

    let runtime = Runtime::new()?;

    let mut tasks: Vec<tokio::task::JoinHandle<()>> = vec![];
    runtime.block_on(async {
        for address in args.cidr.iter().map(|ip_inet| return ip_inet.address()) {
            for port in args.port_start..=args.port_end {
                let cloned_sender = sender.clone();

                let task = tokio::spawn(async move {
                    let connection = tokio::net::TcpStream::connect((address, port)).await;
                    if let Ok(_open) = connection {
                        cloned_sender
                            .send((address, port))
                            .await
                            .expect("sending message to channel must not fail");
                    };
                });

                tasks.push(task);
            }
        }

        for task in tasks {
            task.await.unwrap();
        }
    });

    drop(sender);

    while let Ok((ip, port)) = receiver.try_recv() {
        println!("ip {ip} has open port {port}")
    }

    Ok(())
}
