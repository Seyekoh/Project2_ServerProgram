use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{self, Read, Write};
use std::fs::{self, File};
use std::path::Path;
use server_program::decode_from_base64;

fn main() -> io::Result<()> {
    // Create a data folder if it doesn't exist
    let data_dir = Path::new("data");
    if !data_dir.exists() {
        fs::create_dir(data_dir)?;
        println!("Created 'data' directory.");
    }

    // Start listening for connections on the specified IP and port
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080");

    // Handle incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected: {:?}", stream.peer_addr());
                // Spawn a thread to handle the client connection
                thread::spawn(move || {
                    if let Err(error) = handle_client(stream) {
                        eprintln!("Error handling client: {}", error);
                    }
                });
            }
            Err(error) => eprintln!("Connection failed: {}", error),
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    // Buffer to store incoming data
    let mut buffer = [0; 1024];
    
    // Read the branch code message
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No data received"));
    }

    let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received: {}", received_data);
    

    // Parse branch code from incoming message
    if let Some(branch_code) = received_data.split('~').nth(1) {
        println!("Received branch code: {}", branch_code.trim());

        // Create a folder for the branch if it doesn't exist
        let branch_path = Path::new("data/").join(branch_code.trim());
        if !branch_path.exists() {
            fs::create_dir(&branch_path)?;
            println!("Created directory for branch: {}", branch_code);
        }

        // Send OK to client
        stream.write_all(b"OK")?;
        println!("Sent OK to client");

        // Read the next message (base64 encoded data)
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No data received for encoded data"));
        }

        let encoded_data = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received encoded data: {}", encoded_data);

        // Remove the ~ chars and decode base64 data
        let clean_data = encoded_data.trim_matches('~');
        let decoded_content = decode_from_base64(clean_data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        // Save the decoded content to a file in the branch folder
        let file_path = branch_path.join("branch_weekly_sales.txt");
        let mut file = File::create(file_path)?;
        file.write_all(decoded_content.as_bytes())?;
        println!("Saved decoded content to file");

        // Send final OK to the client to indicate successfull receipt
        stream.write_all(b"OK")?;
        println!("Sent final OK to client");

    } else {
        eprintln!("Failed to parse branch code");
    }

    Ok(())
}
