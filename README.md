# MCP GitHub CLI Server

This is a Model Context Protocol (MCP) GitHub CLI wrapper server written in Rust, providing MCP wrappers for GitHub CLI tools that allow you to perform GitHub operations through MCP clients (like Claude Desktop).

## Features

- Get GitHub login status
- List user repositories 
- View repository information
- List repository Issues and Pull Requests
- Create Issues and Pull Requests
- Clone repositories
- Execute arbitrary GitHub CLI commands

## Prerequisites

1. Install GitHub CLI
   - macOS: `brew install gh`
   - Windows: `winget install --id GitHub.cli`
   - Linux: Please refer to GitHub CLI official documentation

2. Login to GitHub CLI
   ```bash
   gh auth login
   ```

## Build and Run

### Build
