#!/usr/bin/env python3
from typing import Any
import httpx
from mcp.server.fastmcp import FastMCP

# Initialize FastMCP server
mcp = FastMCP("zkvm-nexus-server")

# Constants
ZKVM_API_ENDPOINT = "http://127.0.0.1:8080/package"


@mcp.tool()
async def create_zkvm_proof(code: str) -> str:
    """Create a Nexus zkVM proof from Rust code to verify correct execution.
    
    The Nexus zkVM (Zero-Knowledge Virtual Machine) provides cryptographic proofs that
    a given Rust code snippet executes correctly, without revealing the execution details.
    
    Requirements:
    - Code must be no_std Rust compatible
    - Do not use any imports
    - The snippet will be placed inside a main function, so write code accordingly
    - Use an assert statement to verify the result of the code
    - Do NOT use any file system operations, networking, environment variables or any other operations that are part of the standard library. We are in a no_std environment.
    - Do NOT pass in a function, just the snippet of code that should be inserted into a main function.
    - Do NOT use any print statements, we are in a no_std environment.
    
    ```rust
    let x:u128 = 1322;
    let result:u128 = x * x * x * x;
    assert_eq!(result, 3054399363856);
    ```
    
    Args:
        code: Valid no_std Rust code to be processed by the zkVM. Include all imports in the snippet.
    
    Returns:
        Response from the zkVM server, including proof details or error messages.
    """
    headers = {
        "Content-Type": "text/plain"
    }
    
    async with httpx.AsyncClient() as client:
        try:
            response = await client.post(
                ZKVM_API_ENDPOINT, 
                content=code, 
                headers=headers,
                timeout=60.0
            )
            response.raise_for_status()
            return f"Successfully created zkVM proof. Response: {response.text}"
        except httpx.HTTPStatusError as e:
            return f"Error creating zkVM proof: HTTP {e.response.status_code} - {e.response.text}"
        except httpx.RequestError as e:
            return f"Error creating zkVM proof: {str(e)}"
        except Exception as e:
            return f"Unexpected error creating zkVM proof: {str(e)}"


if __name__ == "__main__":
    # Initialize and run the server
    mcp.run(transport='stdio') 