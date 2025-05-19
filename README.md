# MCP GitHub CLI 服务器

这是一个使用Rust编写的Model Context Protocol (MCP) GitHub CLI封装服务器，提供了GitHub CLI工具的MCP包装，让你可以通过MCP客户端（如Claude Desktop）执行GitHub操作。

## 功能

- 获取GitHub登录状态
- 列出用户仓库
- 查看仓库信息
- 列出仓库Issues和Pull Requests
- 创建Issue和Pull Request
- 克隆仓库
- 执行任意GitHub CLI命令

## 前提条件

1. 安装GitHub CLI
   - macOS：`brew install gh`
   - Windows：`winget install --id GitHub.cli`
   - Linux：请参考GitHub CLI官方文档

2. 登录GitHub CLI
   ```bash
   gh auth login
   ```

## 构建和运行

### 构建

```bash
cargo build --release
```

### 运行

```bash
cargo run --release
```

## 在Claude Desktop中使用

1. 构建服务器

```bash
cargo build --release
```

2. 在Claude Desktop的配置文件中添加服务器配置

```json
{
  "mcpServers": {
    "github": {
      "command": "PATH-TO/mcp-github-server/target/release/mcp-github-server",
      "args": []
    }
  }
}
```

3. 重启Claude Desktop

4. 使用以下工具与GitHub CLI服务器交互：

- `github.auth_status` - 检查GitHub CLI登录状态
- `github.list_repos` - 列出当前用户的仓库
- `github.repo_view` - 查看仓库信息，参数: `{"owner": "仓库拥有者", "repo": "仓库名"}`
- `github.list_issues` - 列出仓库Issues，参数: `{"owner": "仓库拥有者", "repo": "仓库名"}`
- `github.create_issue` - 创建Issue，参数: `{"title": "Issue标题", "body": "Issue内容", "repo": "所有者/仓库名"}`
- `github.list_prs` - 列出仓库Pull Requests，参数: `{"owner": "仓库拥有者", "repo": "仓库名"}`
- `github.create_pr` - 创建Pull Request，参数: `{"title": "PR标题", "body": "PR描述", "base": "目标分支", "head": "源分支", "repo": "所有者/仓库名"}`
- `github.clone_repo` - 克隆仓库，参数: `{"repo": "所有者/仓库名", "directory": "克隆的目标目录"}`
- `github.run_command` - 运行任意GitHub CLI命令，参数: `"repo list --limit 5"`（注意不要包含前缀'gh'）

## 示例

```
// 获取GitHub登录状态
github.auth_status

// 列出当前用户的仓库
github.list_repos

// 查看仓库信息
github.repo_view {"owner": "modelcontextprotocol", "repo": "rust-sdk"}

// 列出仓库Issues
github.list_issues {"owner": "modelcontextprotocol", "repo": "rust-sdk"}

// 创建Issue
github.create_issue {"title": "Bug报告", "body": "描述bug的详细信息", "repo": "我的用户名/我的仓库"}

// 克隆仓库
github.clone_repo {"repo": "modelcontextprotocol/rust-sdk"}

// 执行任意GitHub CLI命令
github.run_command "repo view modelcontextprotocol/rust-sdk --json name,description"
```

## 技术栈

- Rust
- tokio - 异步运行时
- rmcp - Model Context Protocol (MCP) SDK
- GitHub CLI - GitHub命令行工具

## 注意事项

- 请确保已安装并登录GitHub CLI
- 对于部分命令，需要有相应的GitHub权限 
