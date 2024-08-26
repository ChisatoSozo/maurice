use futures::stream::Stream;
use log::{debug, error, info, warn};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::pin::Pin;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

pub struct PythonProcess {
    child: std::process::Child,
    stdin: std::process::ChildStdin,
    stdout: BufReader<std::process::ChildStdout>,
}

impl PythonProcess {
    fn new() -> std::io::Result<Self> {
        debug!("Creating new PythonProcess");
        let mut child = Command::new("python")
            .arg("../functions/python/index.py")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take().unwrap();
        let mut stdout = BufReader::new(child.stdout.take().unwrap());

        // Wait for the "Ready" message
        let mut ready_line = String::new();
        loop {
            stdout.read_line(&mut ready_line)?;
            if ready_line.trim() == "Ready" {
                info!("Python process is ready");
                break;
            }
            ready_line.clear();
        }

        info!("PythonProcess created successfully");
        Ok(PythonProcess {
            child,
            stdin,
            stdout,
        })
    }

    fn send_command(&mut self, name: &str, args: Value, caller_id: &str) -> std::io::Result<()> {
        let command = json!({
            "name": name,
            "args": args,
            "caller_id": caller_id
        });

        debug!("Sending command: {}", command);
        writeln!(self.stdin, "{}", command.to_string())?;
        self.stdin.flush()?;
        debug!("Command sent successfully");

        Ok(())
    }

    fn read_response(&mut self) -> std::io::Result<Value> {
        debug!("Reading response from Python process");
        loop {
            let mut line = String::new();
            self.stdout.read_line(&mut line)?;
            line = line.trim().to_string();

            if line.starts_with("{") {
                match serde_json::from_str(&line) {
                    Ok(response) => {
                        debug!("JSON response received: {:?}", response);
                        return Ok(response);
                    }
                    Err(e) => {
                        warn!(
                            "Failed to parse JSON from line that starts with '{{': {:?}. Line: {}",
                            e, line
                        );
                        // Continue to the next iteration to read another line
                    }
                }
            } else {
                // Log non-JSON lines and continue reading
                if line.len() > 0 {
                    info!("Non-JSON output from Python process: {}", line);
                }

                // Continue to the next iteration to read another line
            }
        }
    }
}

pub struct PythonInterface {
    process: Arc<Mutex<PythonProcess>>,
}

impl PythonInterface {
    pub fn new() -> std::io::Result<Self> {
        info!("Initializing PythonInterface");
        let process = Arc::new(Mutex::new(PythonProcess::new()?));

        info!("PythonInterface initialized successfully");
        Ok(PythonInterface { process })
    }

    pub async fn execute(&self, name: &str, args: Value) -> impl Stream<Item = Value> {
        let caller_id = Uuid::new_v4().to_string();
        debug!("Executing command '{}' with caller_id: {}", name, caller_id);

        let command = json!({
            "name": name,
            "args": args,
            "caller_id": caller_id.clone()
        });

        if let Err(e) =
            self.process
                .lock()
                .await
                .send_command(name, command["args"].clone(), &caller_id)
        {
            error!("Failed to send command: {:?}", e);
        }

        let (tx, rx) = mpsc::channel(10);
        let process_clone = Arc::clone(&self.process);

        tokio::spawn(async move {
            loop {
                match process_clone.lock().await.read_response() {
                    Ok(response) => {
                        if response["caller_id"].as_str() == Some(&caller_id) {
                            if let Err(e) = tx.send(response.clone()).await {
                                error!("Failed to send response to stream: {:?}", e);
                                break;
                            }
                            if response["done"].as_bool() == Some(true) {
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error reading response: {:?}", e);
                        break;
                    }
                }
            }
        });

        ResponseStream { rx }
    }
}

struct ResponseStream {
    rx: mpsc::Receiver<Value>,
}

impl Stream for ResponseStream {
    type Item = Value;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}
