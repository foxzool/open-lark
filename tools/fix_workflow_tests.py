#!/usr/bin/env python3
"""Fix test module imports in openlark-workflow crate."""

import os
import re

def fix_test_imports(file_path):
    """Fix the test module imports in a single file."""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # Pattern to match the test module with empty body
    pattern = r'#\[cfg\(test\)\]\nmod tests \{\s*\n\n    #(\[test\])'
    replacement = '''#[cfg(test)]
mod tests {
    use openlark_core::config::Config;
    use std::sync::Arc;

    use super::*;

    #[test]'''

    if re.search(pattern, content, re.DOTALL):
        content = re.sub(pattern, replacement, content, flags=re.DOTALL)
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        return True
    return False

def main():
    # Files that need fixing (from clippy output)
    files = [
        "crates/openlark-workflow/src/v1/task/collaborator/create.rs",
        "crates/openlark-workflow/src/v1/task/comment/create.rs",
        "crates/openlark-workflow/src/v1/task/comment/update.rs",
        "crates/openlark-workflow/src/v1/task/complete.rs",
        "crates/openlark-workflow/src/v1/task/create.rs",
        "crates/openlark-workflow/src/v1/task/follower/batch_delete.rs",
        "crates/openlark-workflow/src/v1/task/follower/create.rs",
        "crates/openlark-workflow/src/v1/task/get.rs",
        "crates/openlark-workflow/src/v1/task/list.rs",
        "crates/openlark-workflow/src/v1/task/patch.rs",
        "crates/openlark-workflow/src/v1/task/reminder/create.rs",
        "crates/openlark-workflow/src/v1/task/collaborator/batch_delete.rs",
        "crates/openlark-workflow/src/v2/attachment/get.rs",
        "crates/openlark-workflow/src/v2/attachment/list.rs",
        "crates/openlark-workflow/src/v2/attachment/upload.rs",
        "crates/openlark-workflow/src/v2/custom_field/option/create.rs",
        "crates/openlark-workflow/src/v2/custom_field/option/patch.rs",
        "crates/openlark-workflow/src/v2/task/add_dependencies.rs",
        "crates/openlark-workflow/src/v2/task/add_members.rs",
        "crates/openlark-workflow/src/v2/task/add_reminders.rs",
        "crates/openlark-workflow/src/v2/task/add_tasklist.rs",
        "crates/openlark-workflow/src/v2/task/create.rs",
        "crates/openlark-workflow/src/v2/task/list.rs",
        "crates/openlark-workflow/src/v2/task/remove_dependencies.rs",
        "crates/openlark-workflow/src/v2/task/remove_members.rs",
        "crates/openlark-workflow/src/v2/task/remove_reminders.rs",
        "crates/openlark-workflow/src/v2/task/remove_tasklist.rs",
        "crates/openlark-workflow/src/v2/task/subtask/create.rs",
        "crates/openlark-workflow/src/v2/task/subtask/list.rs",
        "crates/openlark-workflow/src/v2/task/tasklists.rs",
        "crates/openlark-workflow/src/v2/task/update.rs",
        "crates/openlark-workflow/src/v2/tasklist/activity_subscription/create.rs",
        "crates/openlark-workflow/src/v2/tasklist/activity_subscription/list.rs",
        "crates/openlark-workflow/src/v2/tasklist/activity_subscription/patch.rs",
        "crates/openlark-workflow/src/v2/tasklist/add_members.rs",
        "crates/openlark-workflow/src/v2/tasklist/remove_members.rs",
        "crates/openlark-workflow/src/v2/tasklist/tasks.rs",
    ]

    # Additional files that appeared in the second batch of errors
    additional_files = [
        "crates/openlark-workflow/src/v1/task/delete.rs",
        "crates/openlark-workflow/src/v1/task/uncomplete.rs",
        "crates/openlark-workflow/src/v2/attachment/delete.rs",
        "crates/openlark-workflow/src/v2/comment/create.rs",
        "crates/openlark-workflow/src/v2/comment/delete.rs",
        "crates/openlark-workflow/src/v2/comment/get.rs",
        "crates/openlark-workflow/src/v2/comment/list.rs",
        "crates/openlark-workflow/src/v2/comment/update.rs",
        "crates/openlark-workflow/src/v2/custom_field/add.rs",
        "crates/openlark-workflow/src/v2/custom_field/create.rs",
        "crates/openlark-workflow/src/v2/custom_field/delete.rs",
        "crates/openlark-workflow/src/v2/custom_field/get.rs",
        "crates/openlark-workflow/src/v2/custom_field/list.rs",
        "crates/openlark-workflow/src/v2/custom_field/remove.rs",
        "crates/openlark-workflow/src/v2/custom_field/update.rs",
        "crates/openlark-workflow/src/v2/section/create.rs",
        "crates/openlark-workflow/src/v2/section/delete.rs",
        "crates/openlark-workflow/src/v2/section/get.rs",
        "crates/openlark-workflow/src/v2/section/list.rs",
        "crates/openlark-workflow/src/v2/section/update.rs",
        "crates/openlark-workflow/src/v2/task/complete.rs",
        "crates/openlark-workflow/src/v2/task/delete.rs",
        "crates/openlark-workflow/src/v2/task/get.rs",
        "crates/openlark-workflow/src/v2/task/tasklists.rs",
        "crates/openlark-workflow/src/v2/task/uncomplete.rs",
        "crates/openlark-workflow/src/v2/tasklist/activity_subscription/delete.rs",
        "crates/openlark-workflow/src/v2/tasklist/activity_subscription/get.rs",
        "crates/openlark-workflow/src/v2/tasklist/activity_subscription/list.rs",
        "crates/openlark-workflow/src/v2/tasklist/create.rs",
        "crates/openlark-workflow/src/v2/tasklist/delete.rs",
        "crates/openlark-workflow/src/v2/tasklist/get.rs",
        "crates/openlark-workflow/src/v2/tasklist/list.rs",
        "crates/openlark-workflow/src/v2/tasklist/update.rs",
    ]

    all_files = files + additional_files

    # Remove duplicates while preserving order
    seen = set()
    unique_files = []
    for f in all_files:
        if f not in seen:
            seen.add(f)
            unique_files.append(f)

    fixed_count = 0
    for file_path in unique_files:
        if os.path.exists(file_path):
            if fix_test_imports(file_path):
                print(f"Fixed: {file_path}")
                fixed_count += 1
            else:
                print(f"Skipped (no match): {file_path}")
        else:
            print(f"Not found: {file_path}")

    print(f"\nTotal fixed: {fixed_count}")

if __name__ == "__main__":
    main()
