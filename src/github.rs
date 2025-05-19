use std::sync::Arc;
use std::process::Command;

use rmcp::{
    Error as McpError, RoleServer, ServerHandler, model::*, 
    service::RequestContext, tool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;
use anyhow::Result;

/// GitHub CLI command result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

/// Repository info request parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RepoParam {
    pub owner: String,
    pub repo: String,
}

/// Create issue request parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateIssueParam {
    pub title: String,
    pub body: Option<String>,
    pub repo: Option<String>,
}

/// Create PR request parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreatePRParam {
    pub title: String,
    pub body: Option<String>,
    pub base: String,
    pub head: String,
    pub repo: Option<String>,
}

/// Clone repository parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CloneRepoParam {
    pub repo: String,
    pub directory: Option<String>,
}

/// GitHub MCP Service
#[derive(Clone)]
pub struct GitHubService {
    last_result: Arc<Mutex<Option<CommandResult>>>,
}

/// Run GitHub CLI command and return result
fn run_gh_command(args: Vec<String>) -> CommandResult {
    let output = Command::new("gh")
        .args(&args)
        .output();
    
    match output {
        Ok(output) => {
            let success = output.status.success();
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            
            CommandResult {
                success,
                output: stdout,
                error: if !success { Some(stderr) } else { None },
            }
        },
        Err(e) => CommandResult {
            success: false,
            output: String::new(),
            error: Some(format!("Failed to execute command: {}", e)),
        },
    }
}

#[tool(tool_box)]
impl GitHubService {
    pub fn new() -> Self {
        Self {
            last_result: Arc::new(Mutex::new(None)),
        }
    }

    /// List repositories of current user
    #[tool(description = "List repositories of current user")]
    async fn list_repos(&self) -> Result<CallToolResult, McpError> {
        let args = vec!["repo".to_string(), "list".to_string(), "--json".to_string(), "name,description,url".to_string()];
        let result = run_gh_command(args);
        
        let mut last_result = self.last_result.lock().await;
        *last_result = Some(result.clone());
        
        if result.success {
            Ok(CallToolResult::success(vec![Content::text(result.output)]))
        } else {
            Err(McpError::internal_error(
                "Failed to get repository list",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// Get repository information
    #[tool(description = "Get information of specified repository")]
    async fn repo_view(
        &self,
        #[tool(aggr)] param: RepoParam,
    ) -> Result<CallToolResult, McpError> {
        let repo = format!("{}/{}", param.owner, param.repo);
        let args = vec!["repo".to_string(), "view".to_string(), repo, "--json".to_string(), "name,description,url,stars,forks,watchers".to_string()];
        let result = run_gh_command(args);
        
        let mut last_result = self.last_result.lock().await;
        *last_result = Some(result.clone());
        
        if result.success {
            Ok(CallToolResult::success(vec![Content::text(result.output)]))
        } else {
            Err(McpError::internal_error(
                "Failed to get repository information",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// List issues of specified repository
    #[tool(description = "List issues of specified repository")]
    async fn list_issues(
        &self,
        #[tool(aggr)] param: RepoParam,
    ) -> Result<CallToolResult, McpError> {
        let repo = format!("{}/{}", param.owner, param.repo);
        let args = vec!["issue".to_string(), "list".to_string(), "--repo".to_string(), repo, "--json".to_string(), "number,title,state,url".to_string()];
        let result = run_gh_command(args);
        
        let mut last_result = self.last_result.lock().await;
        *last_result = Some(result.clone());
        
        if result.success {
            Ok(CallToolResult::success(vec![Content::text(result.output)]))
        } else {
            Err(McpError::internal_error(
                "Failed to get issues list",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// Create issue
    #[tool(description = "Create issue in specified repository")]
    async fn create_issue(
        &self,
        #[tool(aggr)] param: CreateIssueParam,
    ) -> Result<CallToolResult, McpError> {
        let mut args = vec!["issue".to_string(), "create".to_string()];
        
        if let Some(repo) = param.repo {
            args.push("--repo".to_string());
            args.push(repo);
        }
        
        args.push("--title".to_string());
        args.push(param.title);
        
        if let Some(body) = param.body {
            args.push("--body".to_string());
            args.push(body);
        }
        
        let result = run_gh_command(args);
        
        let mut last_result = self.last_result.lock().await;
        *last_result = Some(result.clone());
        
        if result.success {
            Ok(CallToolResult::success(vec![Content::text(result.output)]))
        } else {
            Err(McpError::internal_error(
                "Failed to create issue",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// List pull requests of specified repository
    #[tool(description = "List pull requests of specified repository")]
    async fn list_prs(
        &self,
        #[tool(aggr)] param: RepoParam,
    ) -> Result<CallToolResult, McpError> {
        let repo = format!("{}/{}", param.owner, param.repo);
        let args = vec!["pr".to_string(), "list".to_string(), "--repo".to_string(), repo, "--json".to_string(), "number,title,state,url".to_string()];
        let result = run_gh_command(args);
        
        let mut last_result = self.last_result.lock().await;
        *last_result = Some(result.clone());
        
        if result.success {
            Ok(CallToolResult::success(vec![Content::text(result.output)]))
        } else {
            Err(McpError::internal_error(
                "Failed to get pull requests list",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// Create pull request
    #[tool(description = "Create pull request")]
    async fn create_pr(
        &self,
        #[tool(aggr)] param: CreatePRParam,
    ) -> Result<CallToolResult, McpError> {
        let mut args = vec!["pr".to_string(), "create".to_string()];
        
        if let Some(repo) = param.repo {
            args.push("--repo".to_string());
            args.push(repo);
        }
        
        args.push("--title".to_string());
        args.push(param.title);
        
        if let Some(body) = param.body {
            args.push("--body".to_string());
            args.push(body);
        }
        
        args.push("--base".to_string());
        args.push(param.base);
        
        args.push("--head".to_string());
        args.push(param.head);
        
        let result = run_gh_command(args);
        
        let mut last_result = self.last_result.lock().await;
        *last_result = Some(result.clone());
        
        if result.success {
            Ok(CallToolResult::success(vec![Content::text(result.output)]))
        } else {
            Err(McpError::internal_error(
                "Failed to create pull request",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// Clone repository
    #[tool(description = "Clone GitHub repository")]
    async fn clone_repo(
        &self,
        #[tool(aggr)] param: CloneRepoParam,
    ) -> Result<CallToolResult, McpError> {
        let mut args = vec!["repo".to_string(), "clone".to_string(), param.repo];
        
        if let Some(dir) = param.directory {
            args.push(dir);
        }
        
        let result = run_gh_command(args);
        
        let mut last_result = self.last_result.lock().await;
        *last_result = Some(result.clone());
        
        if result.success {
            Ok(CallToolResult::success(vec![Content::text(result.output)]))
        } else {
            Err(McpError::internal_error(
                "Failed to clone repository",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// Run any GitHub CLI command
    #[tool(description = "Run any GitHub CLI command")]
    async fn run_command(
        &self,
        #[tool(param)]
        #[schemars(description = "GitHub CLI command without gh prefix")]
        command: String,
    ) -> Result<CallToolResult, McpError> {
        let args: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();
        let result = run_gh_command(args);
        
        let mut last_result = self.last_result.lock().await;
        *last_result = Some(result.clone());
        
        if result.success {
            Ok(CallToolResult::success(vec![Content::text(result.output)]))
        } else {
            Err(McpError::internal_error(
                "Failed to execute command",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// Get GitHub authentication status
    #[tool(description = "Check GitHub CLI authentication status")]
    async fn auth_status(&self) -> Result<CallToolResult, McpError> {
        let args = vec!["auth".to_string(), "status".to_string()];
        let result = run_gh_command(args);
        
        let mut last_result = self.last_result.lock().await;
        *last_result = Some(result.clone());
        
        Ok(CallToolResult::success(vec![Content::text(result.output)]))
    }
}

#[tool(tool_box)]
impl ServerHandler for GitHubService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This is a GitHub CLI wrapper server that provides GitHub operation tools. Please ensure GitHub CLI is installed and logged in before use. Use auth_status to check login status, list_repos to list repositories, repo_view to view repository information, list_issues and list_prs to view issues and PRs, create_issue and create_pr to create issues and PRs, clone_repo to clone repositories, and run_command to run any GitHub CLI command.".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
} 
