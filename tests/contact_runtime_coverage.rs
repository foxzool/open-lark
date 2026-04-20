//! openlark 通讯录运行时覆盖测试入口。

#![cfg(feature = "communication")]

#[path = "unit/contact/batch_operations_tests.rs"]
mod batch_operations_tests;
#[path = "unit/contact/department_tests.rs"]
mod department_tests;
