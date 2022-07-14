# Spring Helper - Spring Web 開發輔助工具

一個簡單的Rust程式，可以幫忙處理開發Spring Web專案中的一些工作。

## 功能

- `init`：從[Spring Initalizr](https://start.spring.io)下載包含Spring WebFlux、Spring Data R2DBC與PostgreSQL Driver的新專案.

> 範例：產生Package Name為`tw.mingchang.project`、使用JDK 18及編譯出JAR的Spring Boot Project，並存於名為`mingchang.zip`的壓縮檔.
>
> macOS指令：
> ```
> ./spring-helper init tw.mingchang.project JAR 18 maven mingchang.zip
> ```
>
> Windows指令：
> ```
> spring-helper.exe init tw.mingchang.project JAR 18 maven mingchang.zip
> ```

- `model`：提供JSON，產生相對應的Model。
> 範例：為Package Name`tw.mingchang.project`的專案產生一個可產出以下JSON的Model，並取名為`Test`。
> > 注意：請將Java Type放置於JSON的Value中，如為JSON Array也請直接轉為`List<>`。
> ```
> {"a": "String", "b": "Integer"}
> ```
>
> macOS指令：
> ```
> ./spring-helper model Test tw.mingchang.project
> ```
>
> Windows指令：
> ```
> spring-helper.exe model Test tw.mingchang.project
> ```
- `quick-start`：從PostgreSQL中讀取指定Schema的Table與Column，並自動產生空的Controller、Service、Model與Repository。
> 範例：利用`postgresql://root:root@localhost:5432/postgres`連接PostgreSQL資料庫，為`tw.mingchang.project`專案產生Schema中所有Table的Controller、Service、Model與Repository。
>
> macOS指令：
> ```
> ./spring-helper quick-start postgresql://root:root@localhost:5432/postgres testdb tw.mingchang.project
> ```
>
> Windows指令：
> ```
> spring-helper.exe quick-start postgresql://root:root@localhost:5432/postgres testdb tw.mingchang.project
> ```
