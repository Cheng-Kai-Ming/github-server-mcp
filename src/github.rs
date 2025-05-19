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

/// GitHub CLI命令结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

/// 仓库信息请求参数
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RepoParam {
    pub owner: String,
    pub repo: String,
}

/// 创建Issue请求参数
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateIssueParam {
    pub title: String,
    pub body: Option<String>,
    pub repo: Option<String>,
}

/// 创建PR请求参数
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreatePRParam {
    pub title: String,
    pub body: Option<String>,
    pub base: String,
    pub head: String,
    pub repo: Option<String>,
}

/// 克隆仓库参数
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CloneRepoParam {
    pub repo: String,
    pub directory: Option<String>,
}

/// GitHub MCP服务
#[derive(Clone)]
pub struct GitHubService {
    last_result: Arc<Mutex<Option<CommandResult>>>,
}

/// 运行GitHub CLI命令并返回结果
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
            error: Some(format!("执行命令失败: {}", e)),
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

    /// 列出当前用户的仓库
    #[tool(description = "列出当前用户的仓库")]
    async fn list_repos(&self) -> Result<CallToolResult, McpError> {
        let args = vec!["repo".to_string(), "list".to_string(), "--json".to_string(), "name,description,url".to_string()];
        let result = run_gh_command(args);
        
        let mut last_result = self.last_result.lock().await;
        *last_result = Some(result.clone());
        
        if result.success {
            Ok(CallToolResult::success(vec![Content::text(result.output)]))
        } else {
            Err(McpError::internal_error(
                "获取仓库列表失败",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// 获取仓库信息
    #[tool(description = "获取指定仓库的信息")]
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
                "获取仓库信息失败",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// 列出指定仓库的Issues
    #[tool(description = "列出指定仓库的Issues")]
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
                "获取Issues列表失败",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// 创建Issue
    #[tool(description = "在指定仓库中创建Issue")]
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
                "创建Issue失败",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// 列出指定仓库的PR
    #[tool(description = "列出指定仓库的Pull Requests")]
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
                "获取Pull Requests列表失败",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// 创建PR
    #[tool(description = "创建Pull Request")]
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
                "创建Pull Request失败",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// 克隆仓库
    #[tool(description = "克隆GitHub仓库")]
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
                "克隆仓库失败",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// 运行任意GitHub CLI命令
    #[tool(description = "运行任意GitHub CLI命令")]
    async fn run_command(
        &self,
        #[tool(param)]
        #[schemars(description = "GitHub CLI命令，不含gh前缀")]
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
                "执行命令失败",
                Some(json!({"error": result.error.unwrap_or_default()})),
            ))
        }
    }

    /// 获取GitHub登录状态
    #[tool(description = "检查GitHub CLI登录状态")]
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
            instructions: Some("这是一个GitHub CLI封装服务器，提供GitHub操作相关的工具。使用前请确保已安装GitHub CLI并登录。使用auth_status检查登录状态，list_repos列出仓库，repo_view查看仓库信息，list_issues和list_prs查看问题和PR，create_issue和create_pr创建问题和PR，clone_repo克隆仓库，run_command运行任意GitHub CLI命令。".to_string()),
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
