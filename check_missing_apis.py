import csv
import os
import re

# Config
CSV_FILE = "api_list_export.csv"
SRC_DIR = "crates/openlark-security/src"
TARGET_BIZ_TAGS = {"auth", "passport", "acs", "security_and_compliance"}

# Mapping expectations based on User Request:
# src/biztag/project/version/resource/name.rs
# resource can have dots -> subdirs.

def normalize_resource(resource):
    return resource.replace(".", "/")

def check_missing():
    if not os.path.exists(CSV_FILE):
        print(f"Error: {CSV_FILE} not found")
        return

    missing_count = 0
    total_target = 0
    
    with open(CSV_FILE, 'r', encoding='utf-8') as f:
        reader = csv.DictReader(f)
        for row in reader:
            biz_tag = row['bizTag']
            if biz_tag not in TARGET_BIZ_TAGS:
                continue
            
            project = row['meta.Project']
            version = row['meta.Version']
            resource = row['meta.Resource']
            name = row['meta.Name']
            
            # Special handling for "old" version? User didn't specify, but some rows have "old". 
            # Assuming standard structure as requested: src/biztag/project/version/resource/name.rs
            # If version is empty or weird, we might need adjustments, but let's stick to the rule first.
            
            # Construct path
            resource_path = normalize_resource(resource)
            
            # Construct expected file path
            # src / biz_tag / project / version / resource_path / name.rs
            # Note: project might need snake_case or specific mapping if not matching directory.
            # Assuming project name in CSV matches directory name for now.
            
            # The User said: "目录按照src,bizTag, meta.project, meta.version, meta.resource, meta.name进行组织"
            # So: src/{bizTag}/{project}/{version}/{resource}/{name}.rs
            
            rel_path = os.path.join(biz_tag, project, version, resource_path, f"{name}.rs")
            full_path = os.path.join(SRC_DIR, rel_path)
            
            total_target += 1
            
            if not os.path.exists(full_path):
                # Also check if it exists but is a mod.rs? Unlikely for "name.rs".
                # User says: name.rs
                print(f"MISSING: {rel_path} | {row['id']} | {row['name']}")
                missing_count += 1

    print(f"Total Target APIs: {total_target}")
    print(f"Missing APIs: {missing_count}")

if __name__ == "__main__":
    check_missing()
