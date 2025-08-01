use anyhow::Result;
use lsp_types::{
    request::{DocumentSymbolRequest, Initialize},
    DocumentSymbolParams, InitializeParams, TextDocumentIdentifier, Url,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    process::{Child, ChildStdin, ChildStdout, Command},
};
use std::process::Stdio;

/// Configuration for an LSP server
#[derive(Debug)]
pub struct LspConfig {
    pub command: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
}

/// Client for communicating with an LSP server
pub struct LspClient {
    process: Child,
    writer: BufWriter<ChildStdin>,
    reader: BufReader<ChildStdout>,
    id_counter: Arc<Mutex<u64>>,
    capabilities: Option<Value>,
}

impl LspClient {
    /// Creates a new LSP client with the given configuration
    pub async fn new(configs: &HashMap<String, LspConfig>) -> Result<Self> {
        // For simplicity, we'll use the first configured LSP server
        // In a real implementation, we'd support multiple servers
        let config = configs
            .values()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No LSP configuration found"))?;

        // Start the LSP server process
        let mut command = Command::new(&config.command);
        command.args(&config.args);
        for (key, value) in &config.env {
            command.env(key, value);
        }

        let mut process = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let writer = BufWriter::new(process.stdin.take().unwrap());
        let reader = BufReader::new(process.stdout.take().unwrap());

        let mut client = Self {
            process,
            writer,
            reader,
            id_counter: Arc::new(Mutex::new(0)),
            capabilities: None,
        };

        // Initialize the LSP server
        client.initialize().await?;

        Ok(client)
    }

    /// Initializes the LSP server
    async fn initialize(&mut self) -> Result<()> {
        let params = InitializeParams {
            process_id: Some(std::process::id().into()),
            capabilities: Default::default(),
            ..Default::default()
        };

        let response = self
            .send_request::<Initialize>("initialize", params)
            .await?;
        self.capabilities = Some(serde_json::to_value(response)?);
        self.notify("initialized", Value::Null).await?;
        Ok(())
    }

    /// Gets context for a user request
    pub async fn get_context(&mut self, _request: &str) -> Result<String> {
        // In a real implementation, we'd analyze the request to determine
        // which files to get symbols from. For now, we'll just return an empty string.
        // This is a placeholder for actual LSP integration.
        Ok(String::new())
    }

    /// Gets document symbols for a file
    pub async fn get_document_symbols(&mut self, file_path: &Path) -> Result<Value> {
        let uri = Url::from_file_path(file_path).map_err(|_| {
            anyhow::anyhow!("Failed to convert path to URI: {}", file_path.display())
        })?;

        let params = DocumentSymbolParams {
            text_document: TextDocumentIdentifier { uri },
            partial_result_params: Default::default(),
            work_done_progress_params: Default::default(),
        };

        let response = self.send_request::<DocumentSymbolRequest>("textDocument/documentSymbol", params)
            .await?;
        Ok(serde_json::to_value(response)?)
    }

    /// Sends a request to the LSP server and returns the response
    async fn send_request<R: lsp_types::request::Request>(
        &mut self,
        method: &str,
        params: R::Params,
    ) -> Result<R::Result>
    where
        R::Params: serde::Serialize,
        R::Result: serde::de::DeserializeOwned,
    {
        let id = {
            let mut counter = self.id_counter.lock().unwrap();
            *counter += 1;
            *counter
        };

        let request = jsonrpc::Request {
            jsonrpc: Some("2.0".to_string()),
            method: method.to_string(),
            params: Some(serde_json::to_value(params)?),
            id: Some(id.into()),
        };

        self.send_message(&request).await?;
        self.receive_response(id).await
    }

    /// Sends a notification to the LSP server
    async fn notify(&mut self, method: &str, params: Value) -> Result<()> {
        let notification = jsonrpc::Notification {
            jsonrpc: Some("2.0".to_string()),
            method: method.to_string(),
            params: Some(params),
        };

        self.send_message(&notification).await
    }

    /// Sends a JSON-RPC message to the LSP server
    async fn send_message<T: serde::Serialize>(&mut self, message: &T) -> Result<()> {
        let content = serde_json::to_string(message)?;
        let content_length = content.len();

        self.writer
            .write_all(format!("Content-Length: {content_length}\r\n\r\n").as_bytes())
            .await?;
        self.writer.write_all(content.as_bytes()).await?;
        self.writer.flush().await?;

        Ok(())
    }

    /// Receives a response from the LSP server
    async fn receive_response<R: serde::de::DeserializeOwned>(
        &mut self,
        expected_id: u64,
    ) -> Result<R> {
        loop {
            let message = self.receive_message().await?;
            if let Some(response) = message.as_response() {
                if response.id == Some(expected_id.into()) {
                    let result_value = response.result.as_ref().cloned().unwrap_or(Value::Null);
                    return serde_json::from_value(result_value)
                        .map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e));
                }
            }
        }
    }

    /// Receives a message from the LSP server
    async fn receive_message(&mut self) -> Result<jsonrpc::Message> {
        let mut content_length = 0;
        let mut headers = String::new();

        // Read headers
        loop {
            let line = self.reader.read_line(&mut headers).await?;
            if line == 0 || headers.ends_with("\r\n\r\n") {
                break;
            }
        }

        // Parse content length
        for line in headers.lines() {
            if line.to_lowercase().starts_with("content-length:") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() > 1 {
                    content_length = parts[1].trim().parse()?;
                }
            }
        }

        // Read content
        let mut content = vec![0; content_length];
        self.reader.read_exact(&mut content).await?;
        let content = String::from_utf8(content)?;

        // Parse JSON-RPC message
        serde_json::from_str(&content).map_err(|e| anyhow::anyhow!("Failed to parse message: {}", e))
    }
}

impl Drop for LspClient {
    fn drop(&mut self) {
        // Start shutdown of the LSP server
        let _ = self.process.start_kill();
    }
}

/// JSON-RPC message types
mod jsonrpc {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Message {
        Request(Request),
        Response(Response),
        Notification(Notification),
    }

    impl Message {
        pub fn as_response(&self) -> Option<&Response> {
            match self {
                Message::Response(r) => Some(r),
                _ => None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Request {
        pub jsonrpc: Option<String>,
        pub method: String,
        pub params: Option<Value>,
        pub id: Option<Value>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Response {
        pub jsonrpc: Option<String>,
        pub result: Option<Value>,
        pub error: Option<Value>,
        pub id: Option<Value>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Notification {
        pub jsonrpc: Option<String>,
        pub method: String,
        pub params: Option<Value>,
    }
}
