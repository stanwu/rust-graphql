# Rust GraphQL API 伺服器

此專案是一個使用 Rust 實作的簡單 GraphQL API 伺服器，採用 Actix-web 框架與 Juniper 函式庫。伺服器包含一個 `echo` 查詢，可回傳輸入的訊息。

## 功能
- **GraphQL API**：使用 Juniper 定義與處理 GraphQL 查詢。
- **Actix-web 整合**：使用 Actix-web 處理 HTTP 請求與路由。
- **Echo 查詢**：一個簡單的查詢，回傳輸入的訊息。

## 設置與運行

### 先決條件
- Rust（最新穩定版本）
- Cargo（Rust 的套件管理工具）

### 步驟
1. 複製此專案：
   ```bash
   git clone <repository-url>
   cd rust-graphql
   ```
2. 建置專案：
   ```bash
   cargo build
   ```
3. 運行伺服器：
   ```bash
   cargo run
   ```
4. 訪問 GraphQL 端點：
   ```
   http://127.0.0.1:8080/graphql
   ```

### 範例查詢
您可以使用 `curl` 測試 `echo` 查詢：
```bash
curl -X POST http://127.0.0.1:8080/graphql \
-H "Content-Type: application/json" \
-d '{"query": "{ echo(message: \"Hello, world!\") }"}'
```
預期回應：
```json
{
  "data": {
    "echo": "Hello, world!"
  }
}
```

## 開發過程摘要

在此專案的開發過程中，遇到了以下挑戰並成功解決：

1. **依賴版本衝突**：
   - 由於 `juniper_actix` 的依賴，`juniper` crate 存在版本衝突。
   - 通過統一所有依賴項使用 `juniper v0.16.1` 解決。

2. **類型不匹配問題**：
   - `post_graphql_handler` 函數需要特定的上下文類型。
   - 將 `EmptyMutation` 和 `EmptySubscription` 更新為使用 `()` 作為上下文類型。

3. **線程安全**：
   - 使用 `Arc` 確保 GraphQL 架構的線程安全共享所有權。

4. **錯誤處理**：
   - 解決與缺少 `Cargo.toml` 相關的錯誤，確保正確的工作目錄。

5. **詳細文件**：
   - 添加了全面的程式碼註解，以便更好地理解和維護。

## 未來改進
- 增加對變更（mutations）和訂閱（subscriptions）的支援。
- 實作身份驗證與授權。
- 增強錯誤處理與日誌記錄。

## 授權
此專案採用 MIT 授權。