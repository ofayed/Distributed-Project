use std::net::UdpSocket;
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Define the addresses of the three servers
    let server_addresses = [
        "127.0.0.1:1004",
        "127.0.0.1:1014",
        "127.0.0.1:1024"
    ];
    let mut server_num = 1;

    // Create a UDP socket for the client.
    let socket = UdpSocket::bind("127.0.0.1:4000")?;

    // Message to send to the servers.
    //let data = 20;
    let mut buffer_size = 0;
    let mut requests_handled = 0;
    let mut turn_to_fail = 0;
    let mut server_down = 0;
    let mut flag=0;
loop {
    println!("Choose Server:");

    server_num = turn_to_fail;

    println!("3. read_buffer_size:");
    println!("4. count_handled_requests:");
    println!("5. pew pew:");


    let mut send_flag_counter = 3;
    
        while (flag==0){
        turn_to_fail = turn_to_fail % 3;
        let mut send_flag = send_flag_counter;
        let mut message = "";
      
         if send_flag == 3 {
            message = "read_buffer_size";
        }
        else if send_flag == 4 {
            message = "count_handled_requests";
        }
        else if send_flag == 5{
            if buffer_size == 0 && requests_handled > 0{
                message = "pew pew";
                turn_to_fail += 1;
                turn_to_fail %= 3;
                server_down = 1;
                
            }
            else{
                println!("You can't pew pew now");
                if buffer_size != 0{
                    println!("Buffer is NOT empty");
                }
                else if requests_handled == 0{
                    println!("Server has not handled any requests yet");
                }
                break;
            }
           
        }


        // Send the message to each server.
        socket.send_to(message.as_bytes(), server_addresses[server_num])?;
        println!("Message sent to server {}: {}", server_addresses[server_num], message);

        let mut buffer = [0u8; 1024];

        if send_flag == 3 || send_flag == 4{
        match socket.recv_from(&mut buffer) {
            Ok((n, sender_address)) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                // functionality: received a request from a PEER SERVER
                
                let message_int = message.parse::<i32>().unwrap();
                if send_flag == 3 {
                    println!("Received the buffer size from {}: {}", sender_address, message);
                    buffer_size = message_int;
                }
                else if send_flag == 4 {
                    println!("Received the requests number from {}: {}", sender_address, message);
                    requests_handled = message_int;
                }
                if buffer_size == 0 && requests_handled > 0{
                    println!("You can pew pew now {}", sender_address);
                }
            }
            Err(e) => {
                eprintln!("Fault tolerance handler: Error receiving the buffer size: {}", e);
            }
            
        }
        }
        if server_down == 1{
            println!("Server is down");
             flag=1;
            let mut send_flag = send_flag_counter;
            let sleep_duration = Duration::from_secs(30);
            thread::sleep(sleep_duration);
            server_down = 0;
            break;;
        }
        send_flag_counter += 1;
        if send_flag_counter == 6{
            break;
        }
        turn_to_fail+=1;
    }

    let sleep_duration = Duration::from_secs(5);
    thread::sleep(sleep_duration);
    }
               
    Ok(())
}
