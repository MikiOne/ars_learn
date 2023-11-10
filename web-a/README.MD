# 使用Ntex开发WEB应用的demo

### 使用JWT作为授权
  - 登录返回Bearer JWT 
  - 使用ntex的middleware技术拦截request header中的Bearer Token
  - 使用FromRequest使其WEB入库方法自动注入从JWT中解析出的登录用户对象
### 使用DIESEL作为ORM，r2d2 dp pool
  - 熟悉diesel_cli操作PG
  - diesel_cli 常用操作
    ```bash
    # github: https://github.com/diesel-rs/diesel/tree/master/diesel_cli#installation
    # 安装 CLI 工具: diesel_cli
    cargo install diesel_cli --no-default-features --features "postgres sqlite mysql"
    ## 如果需要postgres特性，本机必须安装pg且配置path环境变量
    cargo install diesel_cli --no-default-features --features sqlite,postgres
    # 使用PG
    cargo install diesel_cli --no-default-features --features postgres
    
    # set DATABASE_URL .env
    echo DATABASE_URL=postgres://postgres:password@localhost/test > .env
    
    # 生成migrations结构
    diesel setup
    # 生成down.sql up.sql
    diesel migration generate pg_table
    # 在up.sql中编写create table；在down.sql中编写drop tableSQL脚本
    # 根据migrations中的SQL脚本在数据库中创建表，以及生成schemas.rs
    diesel migration run
    
    # 如果表已经存在，生成diesel::table!表信息
    # -o 指定表名
    diesel print-schema -o hf_dict > src/schema.rs
    ```
  - 熟悉 diesel中查询的filter用法，对应`where name=a and age=3`
    ```rust
    pub fn find_with_login(
        bo: LoginBO,
        conn: &mut PgConnection,
    ) -> Result<Option<models::HfDict>, diesel::result::Error> {
        use crate::schema::hf_dict::dsl::*;
        let res = hf_dict
            .filter(name.eq(bo.name).and(code.eq(bo.code)))
            .first::<models::HfDict>(conn)
            .optional()?;
        Ok(res)
    }
    ```
### 使用PostgreSQL数据库