use std::io;
use std::process::exit;
use tokio::time::{sleep, Duration};
use tokio::net::UdpSocket;
use std::net::{SocketAddr, ToSocketAddrs};
use std::process::Output;
use minifb::{Key, Window, WindowOptions, Scale, ScaleMode};
use image::{ImageBuffer, DynamicImage, Rgba, GenericImageView};
use steganography::encoder::Encoder;
use steganography::decoder::Decoder;
use serde::{Serialize, Deserialize};
use std::fs;
use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::env;
use toml;
use std::collections::HashMap;
use std::sync::Arc;
extern crate csv;
use csv::Writer;
use csv::ReaderBuilder;
use std::error::Error;

use std::io::prelude::*;

struct Picture {
    access_count: usize,
    image_data: Vec<u8>,
    encrypted_data: Vec<u8>
}
impl Picture {
    fn new(image_data: Vec<u8>, access_count: usize, encrypted_data: Vec<u8>) -> Self {
        Picture { access_count, image_data,encrypted_data }
    }
}


async fn send_message_with_retransmission(
    message: &str,
    destination: SocketAddr,
    max_retransmissions: usize,
    timeout: Duration,
    socket: &UdpSocket
) -> Result<(), Box<dyn std::error::Error>> {
    //let socket = UdpSocket::bind("127.0.0.1:1001").await?;

    let mut retransmissions = 0;
    let mut flag =0;
    loop {
       
    
        // Send the message
        socket.send_to(message.as_bytes(), destination).await?;
        println!("Message sent to {}", destination);
      
        // Wait for a response or a timeout
        let mut buf = [0; 1024];
        let result = tokio::select! {
            res = socket.recv_from(&mut buf) => res,
            _ = sleep(timeout) => Ok((0, SocketAddr::from(([0, 0, 0, 0], 0)))),
        };

        match result {
            Ok((num_bytes, sender)) if num_bytes > 0 => {
                println!("Received response from {}: {:?}", sender, String::from_utf8_lossy(&buf[..num_bytes]));
                println!(" entered");
                let mut buf1 = [0; 1024];
                let (num_bytes2, sender) = socket.recv_from(&mut buf1).await.expect("Failed to receive data");
                let mut buf2 = [0; 1024];
                let (num_bytes2, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");
                let data = &buf2[..num_bytes2];
               // let data_string = String::from_utf8_lossy(data).to_string(); // Convert the data to a String
                println!("Received response from {}: {:?}", sender, String::from_utf8_lossy(&buf2[..num_bytes2]));
                if String::from_utf8_lossy(&buf2[..num_bytes2]).trim().to_string() == "Name Registered".to_string() {
                    flag = 1;

                } else {
                    let destination =sender;
                    println!("Enter Your Name ");
                    let mut name = String::new();
                    io::stdin().read_line(&mut name)?;
                    socket.send_to(name.trim().as_bytes(), destination).await?;
                }
                break Ok(());
            }
            _ => {
                println!("No response, retransmitting...");
                retransmissions += 1;
                if retransmissions >= max_retransmissions {
                    break Err("Max retransmissions reached".into());

                    //break Err("Max retransmissions reached".into());
                }
            }
        }
    }
}

static mut imagcount:i32 = 0;
async fn send_message_with_retransmission2(
    message: &str,
    destination: SocketAddr,
    max_retransmissions: usize,
    timeout: Duration,
    socket: &UdpSocket
) -> Result<(), Box<dyn std::error::Error>> {
    //let socket = UdpSocket::bind("127.0.0.1:1001").await?;
    
    let mut retransmissions = 0;
    let mut flag =0;
    loop {
        // Send the message
        socket.send_to(message.as_bytes(), destination).await?;
        println!("Message sent to {}", destination);

        // Wait for a response or a timeout
        let mut buf = [0; 1024];
        let result = tokio::select! {
            res = socket.recv_from(&mut buf) => res,
            _ = sleep(timeout) => Ok((0, SocketAddr::from(([0, 0, 0, 0], 0)))),
        };

        match result {
            Ok((num_bytes, sender)) if num_bytes > 0 => {
                let mut flag = 0;
                while flag == 0 {
                
                    let mut buf2 = [0; 1024];
                    let (num_bytes, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");
                    // Receive the expected image size
                    //let (bytes_received, _) = socket.recv_from(&mut buffer).await.expect("Failed to receive data");
                   // let expected_image_size = u64::from_ne_bytes(buf2[..num_bytes].try_into().unwrap());
                    
                    println!("Received Ack from {}", sender);
                    println!("Enter Receiver's Name ");
                    let mut message = String::new();
                    io::stdin().read_line(&mut message)?;
                    let destination = sender;
                    socket.send_to(message.trim().as_bytes(), destination).await?;
                    // println!("Message sent to {}: {}", correct_add, message);

                    let mut buffer = [0; 1024];
                    let (num_bytes, src_addr) = socket.recv_from(&mut buffer).await?;
                    let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
                    println!("You received a message '{}'", received_message);

                    if received_message.to_string() == "Name not found".to_string() {
                        println!("Name not found, Try another name");
                    } else {
                        let mut add = received_message.to_string();
                        let dest_addr = add.as_str();
                        println!("Name found at address '{}' ", dest_addr);
                        socket.send_to("request".trim().as_bytes(), dest_addr).await?;
                  
                        let mut buf = [0; 1024];
                        let (num_bytes2, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
                        let imagnum = u64::from_ne_bytes(buf[..num_bytes2].try_into().unwrap());
            
                        //println!("Enter Message ");
                        for i in 0..imagnum
                        {
                            let mut received_data2 = Vec::new(); // Store received fragments
                            let mut buf = [0; 1024];
                            let (num_bytes2, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
                            // Receive the expected image size
                            //let (bytes_received, _) = socket.recv_from(&mut buffer).await.expect("Failed to receive data");
                            let expected_image_size = u64::from_ne_bytes(buf[..num_bytes2].try_into().unwrap());
                            
                      
                        
                            println!("Expected image size: {} bytes", expected_image_size);
                            let mut buffer = vec![0; 1024]; 
                            while let Ok(bytes) = socket.recv(&mut buffer).await {
                                // Process the received fragment
                                received_data2.extend_from_slice(&buffer[..bytes]);
                                       
                                     
                                        println!("Received size {} ", received_data2.len());
                                        // Check if the entire image has been received
                                        if received_data2.len() as u64 == expected_image_size {
                                            println!("Received encrypted image");
                                           unsafe{ let encrypted_image_name = format!("received/received_image{}.jpg", imagcount);
                                            fs::write(&encrypted_image_name, &received_data2)?;
                    
                                            
                                            let mut buf = [0; 1024];
                                            let (num_bytes2, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
                                           
                                            let access = u64::from_ne_bytes(buf[..num_bytes2].try_into().unwrap());
                                
                                            println!("Received  image and saved it as '{}' with number of access {} ", encrypted_image_name,access);
                                        
                                            
                                        }
                                            break; // Exit the loop once the entire image is received
                                        }
                                 
                
                                    }
                        }
                        //send_image_over_udp( dest_addr,socket).await?;
                        /*let mut m = String::new();
                        io::stdin().read_line(&mut m)?;
                        socket.send_to(m.trim().as_bytes(), dest_addr).await?;*/
                        flag = 1;
                    }
                }
                break Ok(());
            }
            _ => {
                println!("No response, retransmitting...");
                retransmissions += 1;
                if retransmissions >= max_retransmissions {
                    break Err("Max retransmissions reached".into());
                    //break Err("Max retransmissions reached".into());
                }
            }
        }
    }
}




async fn send_message_with_retransmission3(
    message: &str,
    destination: SocketAddr,
    max_retransmissions: usize,
    timeout: Duration,
    socket: &UdpSocket,
    image_path: &String,
    imageCounter: u16
) -> Result<(), Box<dyn std::error::Error>> {

  
    //let socket = UdpSocket::bind("127.0.0.1:1001").await?;

    let mut retransmissions = 0;
    let mut flag =0;
    loop {
        socket.send_to(message.as_bytes(), destination).await?;
        println!("Message sent to {}", destination);

        // Wait for a response or a timeout
        let mut buf = [0; 1024];
        let result = tokio::select! {
            res = socket.recv_from(&mut buf) => res,
            _ = sleep(timeout) => Ok((0, SocketAddr::from(([0, 0, 0, 0], 0)))),
        };

        match result {
            Ok((num_bytes, sender)) if num_bytes > 0 => {
                let data = &buf[..num_bytes];
                let data_string = String::from_utf8_lossy(data).to_string(); // Convert the data to a String

                let mut buf2 = [0; 1024];
                let (num_bytes, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");
                // Receive the expected image size
                //let (bytes_received, _) = socket.recv_from(&mut buffer).await.expect("Failed to receive data");
                //let expected_image_size = u64::from_ne_bytes(buf2[..num_bytes].try_into().unwrap());
                
        println!("received from: {} : {}", sender, data_string);
        let dir_path = "pictures/";
        let mut images: Vec<(String)> = Vec::new();
        // Read the directory
        if let Ok(entries) = fs::read_dir(dir_path) {
            // Iterate over each entry
            for entry in entries {
                if let Ok(entry) = entry {
                    // Get the path of the entry and convert it to a string
                    let image_path = entry.path().display().to_string();

                    // Define the access rights for the image
                    let access_rights = 5; // Replace this with the actual access rights

                    // Add the image path and its access rights to the vector
                    images.push((image_path));
                }
            }
        }
        // Display the vector of images and their access rights
        println!("Images");
        for (index, (image_path)) in images.iter().enumerate() {
            println!("Index: {}, Image Path: {}", index, image_path);
        }

        // Ask the user to select an image
        println!("Enter the index of the image you want to send:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let index: usize = input.trim().parse().expect("Please type a number!");
        let destination = sender;
        // Check if the index is valid
        if index < images.len() {
            let (image_path) = &images[index];
            let image_data = fs::read(image_path)?;
        
            let expected_image_size = image_data.len() as u64;

            
            // Send the expected image size to the server
            socket.send_to(&expected_image_size.to_ne_bytes(), destination).await?;

            // Define the maximum size of each fragment
            let max_fragment_size = 1024; // Adjust this according to your needs

            // Split the image data into smaller fragments and send them
            for chunk in image_data.chunks(max_fragment_size) {
                //println!("sent");
                socket.send_to(chunk, destination).await?;
            }

            println!("Image sent successfully!");
           
        
        } else {
            println!("Invalid index");
        }

        let mut buffer = vec![0; 1024]; // Adjust the buffer size to match the maximum fragment size
            
                let mut received_data = Vec::new(); // Store received fragments
            
                // Receive the expected image size
                //let (bytes_received, _) = socket.recv_from(&mut buffer).await?;
               // let expected_image_size = u64::from_ne_bytes(buffer[..bytes_received].try_into().unwrap());
                let mut buf2 = [0; 1024];
                let (num_bytes, crypt_addr) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");
                // Receive the expected image size
                //let (bytes_received, _) = socket.recv_from(&mut buffer).await.expect("Failed to receive data");
                let expected_image_size = u64::from_ne_bytes(buf2[..num_bytes].try_into().unwrap());
                println!("Expected image size: {} bytes", expected_image_size);
            
                while let Ok(bytes) = socket.recv(&mut buffer).await {
                    // Process the received fragment
                    received_data.extend_from_slice(&buffer[..bytes]);
                   
                    // Check if the entire image has been received
                    if received_data.len() as u64 == expected_image_size {
                      
                        let combined_string = "encryption/".to_string() + "received_image" + index.to_string().as_str() +  ".jpg" ;
                        // Save the received data as an image file
                          fs::write(combined_string, &received_data)?;
                    
                                println!("Received image and saved it as 'received_image.jpg'");
                        break; // Exit the loop once the entire image is received
                    }
                   /* if(received_data.len() as u64 >=  expected_image_size - 10000)
                    {
                        fs::write("received_image.jpg", &received_data)?;
            
                        println!("Received most of the image and saved it as 'received_image.jpg'");
                        break; // Exit the loop once the entire image is received

                    } */
                } 
                break Ok(());
    }
    _ => {
        println!("No response, retransmitting...");
        retransmissions += 1;
        if retransmissions >= max_retransmissions {
            // break Ok(());
            break Err("Max retransmissions reached".into());

        }
    }
}
break Ok(());
    }
}
fn bytes_to_image(bytes: Vec<u8>) -> DynamicImage {
    image::load_from_memory(&bytes).unwrap()
}

async fn send_image_over_udp(
    server_ip: &str,
    socket: &UdpSocket,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = "encryption/";
    let mut images: Vec<(String)> = Vec::new();
    // Read the directory
    if let Ok(entries) = fs::read_dir(dir_path) {
        // Iterate over each entry
        for entry in entries {
            if let Ok(entry) = entry {
                // Get the path of the entry and convert it to a string
                let image_path = entry.path().display().to_string();

                // Define the access rights for the image


                // Add the image path and its access rights to the vector
                images.push((image_path));
            }
        }
    }
    // Display the vector of images and their access rights
    println!("Images and their access rights:");
    for (index, (image_path)) in images.iter().enumerate() {
        println!("Index: {}, Image Path: {}", index, image_path);
    }

    // Ask the user to select an image
    println!("Enter the index of the image you want to send:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let index: usize = input.trim().parse().expect("Please type a number!");

    // Check if the index is valid
    if index < images.len() {
        let (image_path) = &images[index];
        let image_data = fs::read(image_path)?;

        // Get the size of the image
        let expected_image_size = image_data.len() as u64;

        // Send the expected image size to the server
        socket.send_to(&expected_image_size.to_ne_bytes(), server_ip).await?;

        // Define the maximum size of each fragment
        let max_fragment_size = 1024; // Adjust this according to your needs

        // Split the image data into smaller fragments and send them
        for chunk in image_data.chunks(max_fragment_size) {
           // println!("sent");
            socket.send_to(chunk, server_ip).await?;
        }

        println!("Image sent successfully!");
        println!("Enter number of times allowed to access ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let access: usize = input.trim().parse().expect("Please type a number!");
        socket.send_to(&access.to_ne_bytes(), server_ip).await?;
        // Display the selected image
        // Here you can add the code to display the image based on its path
        // println!("Sending image at path: {}", image_path);
    } else {
        println!("Invalid index");
    }
    // Read the image file asynchronously


    Ok(())
}

fn read_file(user_data: &mut HashMap<String, i32> ) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("access.csv")?;
    let mut read = ReaderBuilder::new().from_reader(file);

    for result in read.records() {
        let record = result?;
        if record.len() >= 2 {
            let image = record[0].to_string();
            let access = record[1].parse();
            //println!(" Address {}  Name {} ", add,name);
            // Parse the SocketAddr from the CSV data.


            user_data.insert(image, access.unwrap());
        }
    }
    // mem::drop(&file);
    Ok(())
}


fn write_file( image: &str, access: &i32) -> Result<(), Box<dyn Error>> {
    // Open the CSV file in append mode
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("access.csv")?;

    let mut write = Writer::from_writer(file);

    // Write the new entry to the CSV file
    write.write_record(&[image, &access.to_string()])?;
    write.flush()?;

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a UDP socket bound to a local address
    let socket = UdpSocket::bind("127.0.0.1:1101").await?;
    let flag2 = 0;
    let mut images: Vec<(String, i32)> = Vec::new();
    let mut imageCounter = 0;
    let mut encryptioncounter=0;
    let mut recieved_red = 0;
    let mut image_data:HashMap<String, i32> = HashMap::new();
    // Destination address (IP and port) of the receiver
    let direct_add: [&str; 3] = ["127.0.0.1:1002", "127.0.0.1:1012", "127.0.0.1:1022"];
    let ask_add: [&str; 3] = ["127.0.0.1:1003", "127.0.0.1:1013", "127.0.0.1:1023"];
    let crypt_add: [&str; 3] = ["127.0.0.1:1006", "127.0.0.1:1016", "127.0.0.1:1026"];
    //let direct_addr[3] = "127.0.0.1:1002";
    //let ask_addr = "127.0.0.1:1003";
    let rec_addr = "127.0.0.1:1004";
    // let crypt_addr = "127.0.0.1:1006";
    let mut flag3 = 1;

    // socket.send_to("register".as_bytes(), direct_addr).await?;
    let destination: SocketAddr = "127.0.0.1:1002".parse()?;
    let message = "Register";
    let max_retransmissions = 0;
    let timeout = Duration::from_secs(1); // Set the timeout to 5 seconds
    match send_message_with_retransmission(
        message,
        "127.0.0.1:1008".parse::<SocketAddr>().unwrap(),
        max_retransmissions,
        timeout,
        &socket,
    ).await {
        Ok(()) => {
            // Handle the success case
            println!("Message sent successfully through {}", direct_add[0]);
        }
        Err(err) => {
            // Handle the error case
            match send_message_with_retransmission(
                message,
                "127.0.0.1:1018".parse::<SocketAddr>().unwrap(),
                max_retransmissions,
                timeout,
                &socket,
            ).await {
                Ok(()) => {
                    // Handle the success case
                    println!("Message sent successfully through {}", direct_add[1]);
                }
                Err(err) => {
                    // Handle the error case
                    match send_message_with_retransmission(
                        message,
                        "127.0.0.1:1028".parse::<SocketAddr>().unwrap(),
                        max_retransmissions,
                        timeout,
                        &socket,
                    ).await {
                        Ok(()) => {
                            // Handle the success case
                            println!("Message sent successfully through {}", direct_add[2]);
                        }
                        Err(err) => {
                            println!("Did not recieve any reply");
                            exit(0);
                        }
                    }
                }
            }
        }
    }
    /*send_message_with_retransmission(
        message,
        direct_add[0].parse().unwrap(),
        max_retransmissions,
        timeout,
        &socket
    ).await?;
    send_message_with_retransmission(
     message,
     direct_add[1].parse().unwrap(),
     max_retransmissions,
     timeout,
     &socket
 ).await?;
 send_message_with_retransmission(
     message,
     direct_add[2].parse().unwrap(),
     max_retransmissions,
     timeout,
     &socket
 ).await?;
 */
    /*
        while flag3 == 0 {

            let mut buffer = [0; 1024];
            let (num_bytes, src_addr) = socket.recv_from(&mut buffer).await?;
            let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
            println!("You received a message '{}'", received_message);

            if received_message.trim().to_string() == "Name Registered".to_string() {
                flag3 = 1;
                break;
            } else {
                println!("Enter Your Name ");
                let mut name = String::new();
                io::stdin().read_line(&mut name)?;
                socket.send_to(name.trim().as_bytes(), direct_addr).await?;
            }
        }*/

    loop {
        println!("Choose to 1. Send \n 2. Receive \n 3.Encrypt \n 4.View Picture \n 5.Exit");
        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice)?;
        let c: i32 = choice.trim().parse()?;
        if c == 1 {
            /*let mut flag = 0;
            while flag == 0 {
                println!("Enter Receiver's Name ");
                let mut message = String::new();
                io::stdin().read_line(&mut message)?;
                socket.send_to(message.trim().as_bytes(), ask_add[0]).await?;
                socket.send_to(message.trim().as_bytes(), ask_add[1]).await?;
                socket.send_to(message.trim().as_bytes(), ask_add[2]).await?;

               // println!("Message sent to {}: {}", correct_add, message);

                let mut buffer = [0; 1024];
                let (num_bytes, src_addr) = socket.recv_from(&mut buffer).await?;
                let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
                println!("You received a message '{}'", received_message);

                if received_message.to_string() == "Name not found".to_string() {
                    println!("Name not found, Try another name");
                } else {
                    let mut add = received_message.to_string();
                    let dest_addr = add.as_str();
                    println!("Name found at address '{}' ", dest_addr);
                    //println!("Enter Message ");
                    send_image_over_udp("received_image.jpg", dest_addr, &socket).await?;
                    /*let mut m = String::new();
                    io::stdin().read_line(&mut m)?;
                    socket.send_to(m.trim().as_bytes(), dest_addr).await?;*/
                    flag = 1;
                }
            }*/
            let message="Ask";
            match send_message_with_retransmission2(
                message,
                "127.0.0.1:1008".parse::<SocketAddr>().unwrap(),
                max_retransmissions,
                timeout,
                &socket,
            ).await {
                Ok(()) => {
                    // Handle the success case
                    println!("Message sent successfully through {}", direct_add[0]);
                }
                Err(err) => {
                    // Handle the error case
                    match send_message_with_retransmission2(
                        message,
                        "127.0.0.1:1018".parse::<SocketAddr>().unwrap(),
                        max_retransmissions,
                        timeout,
                        &socket,
                    ).await {
                        Ok(()) => {
                            // Handle the success case
                            println!("Message sent successfully through {}", direct_add[1]);
                        }
                        Err(err) => {
                            // Handle the error case
                            match send_message_with_retransmission2(
                                message,
                                "127.0.0.1:1028".parse::<SocketAddr>().unwrap(),
                                max_retransmissions,
                                timeout,
                                &socket,
                            ).await {
                                Ok(()) => {
                                    // Handle the success case
                                    println!("Message sent successfully through {}", direct_add[2]);
                                }
                                Err(err) => {
                                    println!("Did not recieve any reply");
                                    exit(0);
                                }
                            }
                        }
                    }
                }
            }

        } else if c == 2 {
            loop {


                // Deserialize the TOML content into a Rust data structure

                // Print the deserialized data
                // println!("{:?}", deserialized_data);

                
                let mut buffer = [0; 1024];
                let (num_bytes, src_addr) = socket.recv_from(&mut buffer).await?;
                let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
                println!("You received a message '{}'", received_message);
                if received_message == "request"
                {
                    let dir_path = "encryption/";
                    let mut images: Vec<(String)> = Vec::new();
                    // Read the directory
                    if let Ok(entries) = fs::read_dir(dir_path) {
                        // Iterate over each entry
                        for entry in entries {
                            if let Ok(entry) = entry {
                                // Get the path of the entry and convert it to a string
                                let image_path = entry.path().display().to_string();
            
                                // Define the access rights for the image
                                let access_rights = 5; // Replace this with the actual access rights
            
                                // Add the image path and its access rights to the vector
                                images.push((image_path));
                            }
                        }
                    }
                    // Display the vector of images and their access rights
                    println!("Images");
                    for (index, (image_path)) in images.iter().enumerate() {
                        println!("Index: {}, Image Path: {}", index, image_path);
                    }
                 
                    println!("how many pictures do you want to send?");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    let num: usize = input.trim().parse().expect("Please type a number!");
                    socket.send_to(&num.to_ne_bytes(), src_addr).await?;
                    // Ask the user to select an image
                    for j in 0..num{
                    println!("Enter the index of the image you want to send:");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    let index: usize = input.trim().parse().expect("Please type a number!");
                    let destination = src_addr;
                    // Check if the index is valid
                    if index < images.len() {
                       
                        println!("Enter number of accesses allowed");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let access: usize = input.trim().parse().expect("Please type a number!");
                        let (image_path) = &images[index];
                        let image_data = fs::read(image_path)?;
                        //socket.send_to("Encrypt".as_bytes(), destination).await?;
                        // Get the size of the image
                        let expected_image_size = image_data.len() as u64;
            
            
                        
                        // Send the expected image size to the server
                        socket.send_to(&expected_image_size.to_ne_bytes(), destination).await?;
            
                        // Define the maximum size of each fragment
                        let max_fragment_size = 1024; // Adjust this according to your needs
            
                        // Split the image data into smaller fragments and send them
                        for chunk in image_data.chunks(max_fragment_size) {
                            //println!("sent");
                            socket.send_to(chunk, destination).await?;
                        }
            
                      socket.send_to(&access.to_ne_bytes(), destination).await?;
                         println!("Image sent succesfully");
                    } 
                    else {
                        println!("Invalid index");
                        
                    }
                }
                }
            
            
            }

        }
        else if c == 3 {

            // Read the image file into a byte vector
            let image_data = fs::read("src/cover.jpg")?;
            let expected_image_size = image_data.len() as u64; // Get the size of the image

            // Create a UDP socket

            let message = "Encrypt";
            match send_message_with_retransmission3(
                message,
                "127.0.0.1:1008".parse::<SocketAddr>().unwrap(),
                max_retransmissions,
                timeout,
                &socket,
                &"src/cover.jpg".to_string(),
                imageCounter
            ).await {
                Ok(()) => {
                    // Handle the success case
                    println!("Message sent successfully through {}", direct_add[0]);
                }
                Err(err) => {
                    // Handle the error case
                    match send_message_with_retransmission3(
                        message,
                        "127.0.0.1:1018".parse::<SocketAddr>().unwrap(),
                        max_retransmissions,
                        timeout,
                        &socket,
                        &"src/cover.jpg".to_string(),
                        imageCounter
                    ).await {
                        Ok(()) => {
                            // Handle the success case
                            println!("Message sent successfully through {}", direct_add[1]);
                        }
                        Err(err) => {
                            // Handle the error case
                            match send_message_with_retransmission3(
                                message,
                                "127.0.0.1:1028".parse::<SocketAddr>().unwrap(),
                                max_retransmissions,
                                timeout,
                                &socket,
                                &"src/cover.jpg".to_string(),
                                imageCounter
                            ).await {
                                Ok(()) => {
                                    // Handle the success case
                                    println!("Message sent successfully through {}", direct_add[2]);
                                }
                                Err(err) => {
                                    println!("Did not recieve any reply");
                                    exit(0);
                                }
                            }
                        }
                    }
                }
            }

            // Send the expected image size to the server
            /*socket.send_to(&expected_image_size.to_ne_bytes(), crypt_add[0]).await?;
            socket.send_to(&expected_image_size.to_ne_bytes(), crypt_add[1]).await?;
            socket.send_to(&expected_image_size.to_ne_bytes(), crypt_add[2]).await?;

            let mut buffer = [0; 1024];
            let (num_bytes, src_addr) = socket.recv_from(&mut buffer).await?;
            let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
            println!("You received a message '{}'", received_message);
            // Define the maximum size of each fragment
            let max_fragment_size = 1024; // Adjust this according to your needs

            // Split the image data into smaller fragments and send them
            for chunk in image_data.chunks(max_fragment_size) {
                socket.send_to(chunk,  src_addr).await?;
            }

            println!("Image sent successfully!");


            let mut buffer = vec![0; 1024]; // Adjust the buffer size to match the maximum fragment size

            let mut received_data = Vec::new(); // Store received fragments

            // Receive the expected image size
            //let (bytes_received, _) = socket.recv_from(&mut buffer).await?;
           // let expected_image_size = u64::from_ne_bytes(buffer[..bytes_received].try_into().unwrap());
            let mut buf2 = [0; 1024];
            let (num_bytes, crypt_addr) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");
            // Receive the expected image size
            //let (bytes_received, _) = socket.recv_from(&mut buffer).await.expect("Failed to receive data");
            let expected_image_size = u64::from_ne_bytes(buf2[..num_bytes].try_into().unwrap());
            println!("Expected image size: {} bytes", expected_image_size);

            while let Ok(bytes) = socket.recv(&mut buffer).await {
                // Process the received fragment
                received_data.extend_from_slice(&buffer[..bytes]);

                // Check if the entire image has been received
                if received_data.len() as u64 == expected_image_size {

                    // Save the received data as an image file
                    fs::write("received_image.jpg", &received_data)?;

                    println!("Received image and saved it as 'received_image.jpg'");
                    break; // Exit the loop once the entire image is received
                }
            } */
        }
        else if c==4
        {

            // The path to the directory containing the images
            let dir_path = "received/";

            // Read the directory
            if(recieved_red == 0){
                if let Ok(entries) = fs::read_dir(dir_path) {
                    // Iterate over each entry
                    for entry in entries {
                        if let Ok(entry) = entry {
                            // Get the path of the entry and convert it to a string
                            let image_path = entry.path().display().to_string();

                            // Define the access rights for the image
                            let access_rights = 2; // Replace this with the actual access rights

                            // Add the image path and its access rights to the vector
                            images.push((image_path, access_rights));
                        }
                    }
                }
                recieved_red = 1;
            }
            // Display the vector of images and their access rights
            println!("Images and their access rights:");
            for (index, (image_path, access_rights)) in images.iter().enumerate() {
                println!("Index: {}, Image Path: {}, Access Rights: {}", index, image_path, access_rights);
            }

            // Ask the user to select an image
            println!("Enter the index of the image you want to display:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let index: usize = input.trim().parse().expect("Please type a number!");

            // Check if the index is valid
            if index < images.len() {
                let mut new_rights:i32 = 0;
                let (image_path, access_rights) =  &mut images[index];
                // Display the selected image
                // Here you can add the code to display the image based on its path
                let path= image_path;
                if  *access_rights > 0 {
                    let encrypted = image::open(path.clone()).expect("Failed to open image");
                    let encoded_image = steganography::util::file_as_image_buffer(path.as_str().to_string());
                    let decoder = Decoder::new(encoded_image);

                    let decodedImageBytes = decoder.decode_alpha();

                    let decodedImage = bytes_to_image(decodedImageBytes);
                    //let image = image::open("decodedImage.png").expect("Failed to open image");
                    let (width, height) = decodedImage.dimensions();
                    let u32_image_datanew: Vec<u32> = decodedImage
                        .to_rgba8()  // Convert to RGBA format
                        .into_raw()
                        .chunks_exact(4)
                        .map(|chunk| {
                            let r = chunk[0] as u32;
                            let g = chunk[1] as u32;
                            let b = chunk[2] as u32;
                            let a = chunk[3] as u32;
                            (a << 24) | (r << 16) | (g << 8) | b
                        })
                        .collect();
                    // Create a window for displaying the image
                    println!("access {}", access_rights);
                    new_rights = access_rights.clone();
                    new_rights -=1;
                    //images[index] = ( path.as_str().to_string(),new_rights);
                    let mut window = Window::new(
                        "Image Viewer",
                        width as usize,
                        height as usize,
                        WindowOptions {
                            resize: true,
                            scale: Scale::X2,
                            scale_mode: ScaleMode::AspectRatioStretch,
                            ..WindowOptions::default()
                        },
                    )
                        .unwrap_or_else(|e| {
                            panic!("{}", e);
                        });

                    while window.is_open() && !window.is_key_down(Key::Escape) {
                        window
                            .update_with_buffer(&u32_image_datanew, width as usize, height as usize)
                            .unwrap_or_else(|e| {
                                println!("Error updating window: {}", e);
                            });
                    }
                }else{

                    let image = image::open(path).expect("Failed to open image");
                    let (width, height) = image.dimensions();
                    let u32_image_datanew: Vec<u32> = image
                        .to_rgba8()  // Convert to RGBA format
                        .into_raw()
                        .chunks_exact(4)
                        .map(|chunk| {
                            let r = chunk[0] as u32;
                            let g = chunk[1] as u32;
                            let b = chunk[2] as u32;
                            let a = chunk[3] as u32;
                            (a << 24) | (r << 16) | (g << 8) | b
                        })
                        .collect();
                    let mut window = Window::new(
                        "Image Viewer",
                        width as usize,
                        height as usize,
                        WindowOptions {
                            resize: true,
                            scale: Scale::X2,
                            scale_mode: ScaleMode::AspectRatioStretch,
                            ..WindowOptions::default()
                        },
                    )
                        .unwrap_or_else(|e| {
                            panic!("{}", e);
                        });

                    while window.is_open() && !window.is_key_down(Key::Escape) {
                        window
                            .update_with_buffer(&u32_image_datanew, width as usize, height as usize)
                            .unwrap_or_else(|e| {
                                println!("Error updating window: {}", e);
                            });

                    }}
                // println!("Displaying image at path: {}", *path);
                let (image_path2, _) =   &images[index];
                images[index] = (image_path2.to_string(),new_rights);
            } else {
                println!("Invalid index");
            }



            //decodedImage.save("decodedImage.png").unwrap();



        }
        else if c == 5 {
            exit(1);
        }
    }
}