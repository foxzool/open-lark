#!/usr/bin/env python3
"""Fix test module imports in openlark-workflow crate - pass 2."""

import os
import re

def fix_test_imports(file_path):
    """Fix the test module imports in a single file."""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # Check if test module exists
    if '#[cfg(test)]' not in content:
        return False

    # Pattern 1: Test module with empty body and use crate:: models
    # Need to add Arc import and use super::*
    pattern1 = r'(#\[cfg\(test\)\]\nmod tests \{\n)(\s+)(use crate::[a-z_/]+::models::\{[a-zA-Z_, \}]+\};)'
    match1 = re.search(pattern1, content, re.DOTALL)
    if match1:
        # Check if Arc is not imported
        if 'use std::sync::Arc;' not in content:
            # Check if we need to add Arc import
            if 'let config = Arc::new(' in content:
                # Insert Arc import before the first use statement in test module
                insertion = '\n    use std::sync::Arc;'
                insertion2 = '\n    use super::*;'
                content = content.replace(match1.group(3), match1.group(3) + insertion + insertion2)
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                return True

    # Pattern 2: Test module with use super::*; but without Arc import
    if 'use super::*;' in content and 'use std::sync::Arc;' not in content:
        if 'let config = Arc::new(' in content:
            # Find the #[cfg(test)] block and add Arc import after mod tests {
            pattern = r'(#\[cfg\(test\)\]\nmod tests \{)'
            match = re.search(pattern, content)
            if match:
                insertion = '\n    use std::sync::Arc;'
                content = content.replace(match.group(1), match.group(1) + insertion)
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                return True

    # Pattern 3: Test module with empty body but missing imports
    # Match: #[cfg(test)]\nmod tests {\n\n    (no use statements before first #[test])
    pattern3 = r'#\[cfg\(test\)\]\nmod tests \{\n\n    #\[test\]'
    if re.search(pattern3, content, re.DOTALL):
        # This is the case where I already added the imports but need to remove Config
        # Check if we have use openlark_core::config::Config; and use super::*;
        if 'use openlark_core::config::Config;' in content and 'use super::*;' in content:
            # Remove the unused Config import
            content = content.replace('    use openlark_core::config::Config;\n', '')
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return True

    return False

def main():
    # All files in workflow tests
    files = [
        "crates/openlark-workflow/src/v1/task/collaborator/create.rs",
        "crates/openlark-workflow/src/v1/task/comment/create.rs",
        "crates/openlark-workflow/src/v1/task/comment/update.rs",
        "crates/openlark-workflow/src/v1/task/complete.rs",
        "crates/openlark-workflow/src/v1/task/create.rs",
        "crates/openlark-workflow/src/v1/task/delete.rs",
        "crates/openlark-workflow/src/v1/task/follower/batch_delete.rs",
        "crates/openlark-workflow/src/v1/task/follower/create.rs",
        "crates/openlark-workflow/src/v1/task/get.rs",
        "crates/openlark-workflow/src/v1/task/list.rs",
        "crates/openlark-workflow/src/v1/task/patch.rs",
        "crates/openlark-workflow/src/v1/task/reminder/create.rs",
        "crates/openlark-workflow/src/v1/task/uncomplete.rs",
        "crates/openlark-workflow/src/v2/attachment/delete.rs",
        "crates/openlark-workflow/src/v2/attachment/get.rs",
        "crates/openlark-workflow/src/v2/attachment/list.rs",
        "crates/openlark-workflow/src/v2/attachment/upload.rs",
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
        "crates/openlark-workflow/src/v2/custom_field/option/create.rs",
        "crates/openlark-workflow/src/v2/custom_field/option/patch.rs",
        "crates/openlark-workflow/src/v2/custom_field/remove.rs",
        "crates/openlark-workflow/src/v2/custom_field/update.rs",
        "crates/openlark-workflow/src/v2/section/create.rs",
        "crates/openlark-workflow/src/v2/section/delete.rs",
        "crates/openlark-workflow/src/v2/section/get.rs",
        "crates/openlark-workflow/src/v2/section/list.rs",
        "crates/openlark-workflow/src/v2/section/update.rs",
        "crates/openlark-workflow/src/v2/task/add_dependencies.rs",
        "crates/openlark-workflow/src/v2/task/add_members.rs",
        "crates/openlark-workflow/src/v2/task/add_reminders.rs",
        "crates/openlark-workflow/src/v2/task/add_tasklist.rs",
        "crates/openlark-workflow/src/v2/task/complete.rs",
        "crates/openlark-workflow/src/v2/task/create.rs",
        "crates/openlark-workflow/src/v2/task/delete.rs",
        "crates/openlark-workflow/src/v2/task/get.rs",
        "crates/openlark-workflow/src/v2/task/list.rs",
        "crates/openlark-workflow/src/v2/task/remove_dependencies.rs",
        "crates/openlark-workflow/src/v2/task/remove_members.rs",
        "crates/openlark-workflow/src/v2/task/remove_reminders.rs",
        "crates/openlark-workflow/src/v2/task/remove_tasklist.rs",
        "crates/openlark-workflow/src/v2/task/subtask/create.rs",
        "crates/openlark-workflow/src/v2/task/subtask/list.rs",
        "crates/openlark-workflow/src/v2/task/tasklists.rs",
        "crates/openlark-workflow/src/v2/task/update.rs",
        "crates/openlark-workflow/src/v2/task/uncomplete.rs",
        "crates/openlark-workflow/src/v2/tasklist/activity_subscription/create.rs",
        "crates/openlark-workflow/src/v2/tasklist/activity_subscription/delete.rs",
        "crates/openlark-workflow/src/v2/tasklist/activity_subscription/get.rs",
        "crates/openlark-workflow/src/v2/tasklist/activity_subscription/list.rs",
        "crates/openlark-workflow/src/v2/tasklist/activity_subscription/patch.rs",
        "crates/openlark-workflow/src/v2/tasklist/add_members.rs",
        "crates/openlark-workflow/src/v2/tasklist/create.rs",
        "crates/openlark-workflow/src/v2/tasklist/delete.rs",
        "crates/openlark-workflow/src/v2/tasklist/get.rs",
        "crates/openlark-workflow/src/v2/tasklist/list.rs",
        "crates/openlark-workflow/src/v2/tasklist/remove_members.rs",
        "crates/openlark-workflow/src/v2/tasklist/tasks.rs",
        "crates/openlark-workflow/src/v2/tasklist/update.rs",
    ]

    fixed_count = 0
    for file_path in files:
        if os.path.exists(file_path):
            if fix_test_imports(file_path):
                print(f"Fixed: {file_path}")
                fixed_count += 1

    print(f"\nTotal fixed: {fixed_count}")

if __name__ == "__main__":
    main()
