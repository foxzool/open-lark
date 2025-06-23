# Drive服务Builder Execute方法添加完成

## 概述

已成功为Drive服务的所有Builder类添加了`execute()`和`execute_with_options()`方法，提供更便捷的API调用方式。

## 已修改的文件

### 1. `/src/service/cloud_docs/drive/v1/media.rs`

**修改的Builder类:**
- `UploadMediaRequestBuilder`
- `UploadPartRequestBuilder`

**新增方法:**
- `execute()` - 直接执行请求
- `execute_with_options()` - 带选项执行请求

**使用示例:**
```rust
// 使用execute方法
let result = UploadMediaRequest::builder()
    .file_name("test.jpg")
    .parent_token("parent_token")
    .size(1024)
    .file(file_data)
    .execute(&media_service)
    .await?;

// 使用execute_with_options方法
let result = UploadMediaRequest::builder()
    .file_name("test.jpg")
    .parent_token("parent_token")
    .size(1024)
    .file(file_data)
    .execute_with_options(&media_service, request_option)
    .await?;
```

### 2. `/src/service/cloud_docs/drive/v1/permissions.rs`

**修改的Builder类:**
- `GetPermissionRequestBuilder`
- `PatchPermissionRequestBuilder`

**新增方法:**
- `execute()` - 直接执行获取/更新权限请求
- `execute_with_options()` - 带选项执行权限请求

**使用示例:**
```rust
// 获取权限
let result = GetPermissionRequest::builder()
    .token("file_token")
    .r#type("doc")
    .execute(&permissions_service)
    .await?;

// 更新权限
let result = PatchPermissionRequest::builder()
    .token("file_token")
    .r#type("doc")
    .external_access_entity("open")
    .execute(&permissions_service)
    .await?;
```

### 3. `/src/service/cloud_docs/drive/v1/file.rs`

**修改的Builder类:**
- `FileUploadPartRequestBuilder`

**新增方法:**
- `execute()` - 直接执行文件分片上传请求
- `execute_with_options()` - 带选项执行文件分片上传请求

**使用示例:**
```rust
let result = FileUploadPartRequest::builder()
    .upload_id("upload_id")
    .seq(1)
    .size(1024)
    .file_chunk(chunk_data)
    .execute(&file_service)
    .await?;
```

### 4. `/src/service/cloud_docs/drive/v2/explorer.rs`

**修改的Builder类:**
- `CreateFolderRequestBuilder`
- `ListFolderRequestBuilder`

**新增方法:**
- `execute()` - 直接执行文件夹操作请求
- `execute_with_options()` - 带选项执行文件夹操作请求

**使用示例:**
```rust
// 创建文件夹
let result = CreateFolderRequest::builder()
    .name("新建文件夹")
    .folder_token("parent_folder_token")
    .execute(&explorer_service)
    .await?;

// 列出文件夹内容
let result = ListFolderRequest::builder()
    .folder_token("folder_token")
    .page_size(50)
    .order_by("EditedTime")
    .direction("DESC")
    .execute(&explorer_service)
    .await?;
```

## 新增示例文件

创建了示例文件 `examples/api/drive_builder_execute_demo.rs`，展示如何使用新增的execute方法。

## 一致性特点

1. **统一的API模式**: 所有execute方法都遵循相同的签名模式
2. **类型安全**: 返回正确的响应类型，保持类型安全
3. **中文文档**: 所有方法都有完整的中文文档注释
4. **便捷性**: 简化了从构建到执行的流程
5. **选项支持**: 支持带RequestOption的执行方式

## 优势

1. **减少样板代码**: 不需要手动调用build()再调用service方法
2. **链式调用**: 支持完整的链式调用模式
3. **一致性**: 与项目中其他Builder的模式保持一致
4. **向后兼容**: 不影响现有的build()方法使用

## 验证

- ✅ 代码格式化通过 (`cargo fmt`)
- ✅ 语法检查通过 (`cargo check --all-features`)
- ✅ 完整构建通过 (`cargo build --all-features`)
- ✅ 示例代码编译通过 (`cargo check --example drive_builder_execute_demo`)

所有Drive服务的Builder类现在都支持便捷的execute方法调用！