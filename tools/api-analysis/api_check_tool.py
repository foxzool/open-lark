import csv
import os

TARGET_TAGS = {'ccm', 'base', 'baike', 'minutes'}
TARGET_PROJECT = None

def check_apis():
    csv_path = '../../api_list_export.csv'
    base_src_path = 'src'
    
    total_count = 0
    found_count = 0
    missing_apis = []

    print(f"Checking APIs from {csv_path}...")
    
    found_apis = []
    
    with open(csv_path, 'r', encoding='utf-8') as f:
        reader = csv.reader(f)
        header = next(reader)
        # map header to index
        header_map = {name: i for i, name in enumerate(header)}
        
        for row in reader:
            biz_tag = row[header_map['bizTag']]
            project = row[header_map['meta.Project']]
            
            if biz_tag in TARGET_TAGS and (TARGET_PROJECT is None or project == TARGET_PROJECT):
                version = row[header_map['meta.Version']]
                resource = row[header_map['meta.Resource']]
                name = row[header_map['meta.Name']]
                
                # Function to convert camelCase to snake_case
                def to_snake(s):
                    import re
                    return re.sub(r'(?<!^)(?=[A-Z])', '_', s).lower()
                
                # Handling resource path (split by .)
                resource_path = resource.replace('.', '/')

                # Normalize name path
                # Split by /
                segments = name.split('/')
                norm_segments = []
                for seg in segments:
                    if not seg:
                        continue
                    if seg.startswith(':'):
                        # Parameter: :folderToken -> _folder_token
                        param_name = seg[1:]
                        norm_segments.append(f"_{to_snake(param_name)}")
                    else:
                        # Normalize all segments to snake_case to match Rust conventions
                        norm_segments.append(to_snake(seg))
                
                norm_name_path = "/".join(norm_segments).replace('#', '_')

                # Construct file path
                # src/bizTag/project/version/resource/name.rs
                # Note: name might be "v2/folder/:token/meta"
                # If resource is "default", we include it in path.
                
                file_rel_path = os.path.join(
                    biz_tag,
                    project,
                    version,
                    resource_path,
                    f"{norm_name_path}.rs"
                )
                
                full_path = os.path.join(base_src_path, file_rel_path)
                
                total_count += 1
                if os.path.exists(full_path):
                    found_count += 1
                    found_apis.append(full_path)
                else:
                    missing_apis.append({
                        'bizTag': biz_tag,
                        'project': project,
                        'version': version,
                        'resource': resource,
                        'name': name,
                        'path': full_path,
                        'row_data': row
                    })

    print(f"Total Target APIs: {total_count}")
    print(f"Found Implementations: {found_count}")
    
    if found_apis:
        print("\nFirst 5 Found APIs (Reference):")
        for p in found_apis[:5]:
            print(f"- {p}")
            
    print(f"Missing Implementations: {len(missing_apis)}")
    
    if missing_apis:
        print("\nFirst 10 Missing APIs:")
        for api in missing_apis[:10]:
            print(f"- {api['path']}")
            
        # Group missing by project to match prompt counts
        from collections import Counter
        missing_by_project = Counter(api['project'] for api in missing_apis)
        print("\nMissing by Project:")
        for proj, count in missing_by_project.items():
            print(f"{proj}: {count}")

if __name__ == '__main__':
    check_apis()
