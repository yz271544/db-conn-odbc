# Connect Database by ODBC

一个基于 Rust 的 ODBC 数据库客户端，提供简单的命令行接口来连接和查询数据库。

## 功能特点

- 支持通过命令行参数配置数据库连接
- 支持执行 SQL 查询并显示结果
- 支持多种 ODBC 驱动程序
- 错误处理和友好的错误提示

## 系统要求

- Rust 1.56 或更高版本
- ODBC 驱动程序管理器（Linux 上使用 unixODBC）
- 目标数据库的 ODBC 驱动程序

## 安装

1. 安装 ODBC 驱动程序管理器：

   ```bash
   # Ubuntu/Debian
   sudo apt-get install unixodbc unixodbc-dev odbcinst
   
   # CentOS/RHEL
   sudo yum install unixODBC unixODBC-devel
   ```

2. 安装目标数据库的 ODBC 驱动程序（例如：达梦数据库、MySQL、PostgreSQL 等）

3. 克隆并构建项目：

   ```bash
   git clone <repository-url>
   cd db-conn-odbc
   cargo build --release
   ```

## 使用方法

程序支持以下命令行参数：

| 参数 | 短选项 | 长选项 | 说明 |
|------|--------|--------|------|
| 驱动程序 | -D | --driver | ODBC 驱动程序名称 |
| 服务器 | -s | --server | 服务器地址和端口 |
| 数据库 | -b | --database | 数据库名称 |
| 用户名 | -u | --uid | 数据库用户名 |
| 密码 | -p | --pwd | 数据库密码 |
| SQL查询 | -q | --query | 要执行的 SQL 查询语句 |

### 示例

1. 连接达梦数据库：

   ```bash
   cargo run -- -D "DM8 ODBC DRIVER" \
                -s "172.16.117.71:15236" \
                -b "JWAB" \
                -u "JWAB" \
                -p "jhy123456" \
                -q "SELECT * FROM dual"
   ```

2. 连接 MySQL：

   ```bash
   cargo run -- -D "MySQL ODBC 8.0 Driver" \
                -s "localhost:3306" \
                -b "testdb" \
                -u "root" \
                -p "password" \
                -q "SELECT * FROM users"
   ```

3. 连接 PostgreSQL：

   ```bash
   cargo run -- -D "PostgreSQL ANSI" \
                -s "localhost" \     
                -b "rustboot" \
                -u "postgres" \
                -p "password" \
                -q "select current_timestamp(0)"
   ```

4. 查看帮助信息：

   ```bash
   cargo run -- --help
   ```

## 常见问题

1. 找不到 ODBC 驱动程序
   - 确保已正确安装 ODBC 驱动程序
   - 检查驱动程序名称是否正确
   - 在 Linux 上，可以使用 `odbcinst -q -d` 命令查看已安装的驱动程序

2. 连接失败
   - 检查服务器地址和端口是否正确
   - 验证用户名和密码
   - 确认数据库名称是否正确
   - 检查网络连接是否正常

## 开发

1. 添加新的依赖：

   ```bash
   cargo add <package-name>
   ```

2. 运行测试：

   ```bash
   cargo test
   ```

3. 代码格式化：

   ```bash
   cargo fmt
   ```

## 许可证

MIT
