use std::thread::sleep;
use std::time::{Duration, SystemTime};
use chrono::{DateTime, Utc, Timelike};
use async_osc::{OscSocket, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let socket = OscSocket::bind("127.0.0.1:0").await?;
    socket.connect("127.0.0.1:9000").await?;

    loop {
        let now = SystemTime::now();

        let dt:DateTime<Utc> = now.clone().into();
        println!("sending {}", dt.format("%H:%M:%S"));
        
        socket.send(("/avatar/parameters/Hour", (dt.hour() as i32,))).await?;
        socket.send(("/avatar/parameters/Minute", (dt.minute() as i32,))).await?;
        socket.send(("/avatar/parameters/Second", (dt.second() as i32,))).await?;

        sleep(Duration::from_secs(1));
    }
}

/*
async fn send_data(dt: DateTime<Utc>) -> Result<(), Error> {
    let socket = OscSocket::bind("127.0.0.1:0").await?;
    socket.connect("127.0.0.1:9000").await?;
    socket.send(("/avatar/parameters/Hour", (dt.hour() as i32,))).await?;
    socket.send(("/avatar/parameters/Minute", (dt.minute() as i32,))).await?;
    socket.send(("/avatar/parameters/Second", (dt.second() as i32,))).await?;
    Ok::<(), Error>(())
}*/