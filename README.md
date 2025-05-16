# zkVM Nexus MCP Server

This MCP server enables Claude to create zkVM proofs by sending Rust code to a local zkVM HTTP server.


## Configuring Claude for Desktop

Install [claude desktop](https://claude.ai/download).

Add the following to your Claude for Desktop configuration file (located at `~/Library/Application Support/Claude/claude_desktop_config.json`):

```json
{
    "mcpServers": {
        "zkvm-nexus-server": {
            "command": "python3",
            "args": [
                "INSERT_PATH"
            ]
        }
    }
}
```

## Setup

1. Install the required dependencies:
   ```
   pip3 install -r requirements.txt
   ```

2. Run the local proof server
    ```
    cargo run -r -- server
    ```

## Usage

Once configured, you can use Claude desktop to create a zkVM proof by providing valid Rust code. For example:

"Can you create a nexus zkVM proof that 3252^4 = 111,841,284,854,016?"

Claude will then use the MCP server to send the code to your local zkVM HTTP server and return the result. 

Run `cargo run -r -- verify` to check the proof is correct
