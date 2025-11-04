#!/usr/bin/env python3
"""
Script to analyze open-lark project and extract all API services and methods.
This script scans the src/service/ directory to identify:
- Service modules
- API versions (v1, v3, v4, etc.)
- Async API methods that return SDKResult
"""

import os
import re
import ast
from pathlib import Path
from typing import Dict, List, Set, Tuple

class APIMethodExtractor:
    def __init__(self, base_path: str):
        self.base_path = Path(base_path)
        self.services_dir = self.base_path / "src" / "service"
        self.results = {}

    def scan_all_services(self) -> Dict:
        """Scan all services and extract API methods"""
        print("🔍 Scanning open-lark services for API methods...")

        # Get all service directories
        service_dirs = [d for d in self.services_dir.iterdir()
                       if d.is_dir() and not d.name.startswith('.')]

        print(f"📁 Found {len(service_dirs)} service directories")

        for service_dir in sorted(service_dirs):
            service_name = service_dir.name
            print(f"\n🔍 Analyzing service: {service_name}")

            service_info = self.analyze_service(service_dir)
            if service_info['has_api']:
                self.results[service_name] = service_info

        return self.results

    def analyze_service(self, service_dir: Path) -> Dict:
        """Analyze a single service directory"""
        service_name = service_dir.name
        service_info = {
            'name': service_name,
            'path': str(service_dir),
            'versions': {},
            'has_api': False,
            'main_module': None,
            'version_modules': []
        }

        # Check for main mod.rs
        main_mod = service_dir / "mod.rs"
        if main_mod.exists():
            service_info['main_module'] = str(main_mod)
            print(f"  📄 Found main module: {main_mod}")

        # Find version directories (v1, v3, v4, etc.)
        version_pattern = re.compile(r'^v\d+$')
        version_dirs = [d for d in service_dir.iterdir()
                       if d.is_dir() and version_pattern.match(d.name)]

        # Also check for subdirectories that might contain API methods
        subdirs = [d for d in service_dir.iterdir()
                  if d.is_dir() and not version_pattern.match(d.name)
                  and not d.name.startswith('.') and d.name != 'models']

        all_api_dirs = version_dirs + subdirs

        for version_dir in sorted(all_api_dirs):
            version_name = version_dir.name
            print(f"  📂 Analyzing version/module: {version_name}")

            api_methods = self.extract_api_methods_from_dir(version_dir)
            if api_methods:
                service_info['versions'][version_name] = {
                    'path': str(version_dir),
                    'methods': api_methods
                }
                service_info['has_api'] = True
                service_info['version_modules'].append(version_name)
                print(f"    ✅ Found {len(api_methods)} API methods")

        # If no version dirs found, check the service directory itself
        if not all_api_dirs:
            api_methods = self.extract_api_methods_from_dir(service_dir)
            if api_methods:
                service_info['versions']['root'] = {
                    'path': str(service_dir),
                    'methods': api_methods
                }
                service_info['has_api'] = True
                service_info['version_modules'].append('root')
                print(f"    ✅ Found {len(api_methods)} API methods in root")

        return service_info

    def extract_api_methods_from_dir(self, dir_path: Path) -> List[Dict]:
        """Extract API methods from a directory"""
        methods = []

        # Find all Rust files
        rust_files = list(dir_path.rglob("*.rs"))

        for rust_file in rust_files:
            file_methods = self.extract_api_methods_from_file(rust_file)
            methods.extend(file_methods)

        return methods

    def extract_api_methods_from_file(self, file_path: Path) -> List[Dict]:
        """Extract API methods from a single Rust file"""
        methods = []

        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()

            # Look for async fn methods that return SDKResult
            pattern = r'pub\s+async\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*SDKResult[^{]*'
            matches = re.finditer(pattern, content, re.MULTILINE | re.DOTALL)

            for match in matches:
                method_name = match.group(1)

                # Skip common non-API methods
                skip_methods = {'new', 'build', 'execute', 'send', 'request', 'from', 'clone'}
                if method_name.lower() in skip_methods:
                    continue

                # Get the full method signature
                start_pos = match.start()
                # Find the end of the method signature
                sig_end = content.find('{', start_pos)
                if sig_end == -1:
                    sig_end = content.find(';', start_pos)

                if sig_end != -1:
                    signature = content[start_pos:sig_end].strip()

                    # Extract parameter info
                    param_match = re.search(r'\(([^)]+)\)', signature)
                    params = param_match.group(1) if param_match else ""

                    methods.append({
                        'name': method_name,
                        'signature': signature,
                        'params': params,
                        'file': str(file_path.relative_to(self.base_path)),
                        'line_number': content[:start_pos].count('\n') + 1
                    })

        except Exception as e:
            print(f"    ⚠️  Error reading {file_path}: {e}")

        return methods

    def print_results(self):
        """Print the analysis results"""
        print("\n" + "="*80)
        print("🎯 OPEN-LARK API ANALYSIS RESULTS")
        print("="*80)

        total_services = len(self.results)
        total_versions = sum(len(info['versions']) for info in self.results.values())
        total_methods = sum(
            len(version_info['methods'])
            for service_info in self.results.values()
            for version_info in service_info['versions'].values()
        )

        print(f"📊 Summary:")
        print(f"   Services with APIs: {total_services}")
        print(f"   API versions/modules: {total_versions}")
        print(f"   Total API methods: {total_methods}")

        print(f"\n📋 Detailed API Methods:")
        print("-"*80)

        for service_name, service_info in sorted(self.results.items()):
            print(f"\n🔸 Service: {service_name}")

            for version_name, version_info in sorted(service_info['versions'].items()):
                methods = version_info['methods']
                if not methods:
                    continue

                print(f"   📂 Version/Module: {version_name} ({len(methods)} methods)")

                for method in sorted(methods, key=lambda x: x['name']):
                    print(f"      • {method['name']}() - {method['file']}")

    def export_to_markdown(self, output_file: str):
        """Export results to markdown file"""
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write("# Open-Lark API Methods Analysis\n\n")
            f.write("This document contains a comprehensive list of all API services and methods in the open-lark project.\n\n")

            # Summary
            total_services = len(self.results)
            total_versions = sum(len(info['versions']) for info in self.results.values())
            total_methods = sum(
                len(version_info['methods'])
                for service_info in self.results.values()
                for version_info in service_info['versions'].values()
            )

            f.write("## Summary\n\n")
            f.write(f"- **Services with APIs**: {total_services}\n")
            f.write(f"- **API versions/modules**: {total_versions}\n")
            f.write(f"- **Total API methods**: {total_methods}\n\n")

            # Detailed list
            f.write("## API Methods by Service\n\n")

            for service_name, service_info in sorted(self.results.items()):
                f.write(f"### {service_name}\n\n")

                for version_name, version_info in sorted(service_info['versions'].items()):
                    methods = version_info['methods']
                    if not methods:
                        continue

                    f.write(f"#### {version_name}\n\n")
                    f.write("| Method | File | Line |\n")
                    f.write("|--------|------|------|\n")

                    for method in sorted(methods, key=lambda x: x['name']):
                        f.write(f"| `{method['name']}()` | `{method['file']}` | {method['line_number']} |\n")

                    f.write("\n")

        print(f"📝 Results exported to: {output_file}")

def main():
    """Main function"""
    base_path = Path(__file__).parent
    extractor = APIMethodExtractor(base_path)

    # Scan all services
    results = extractor.scan_all_services()

    # Print results
    extractor.print_results()

    # Export to markdown
    output_file = base_path / "api_methods_analysis.md"
    extractor.export_to_markdown(output_file)

    print(f"\n✅ Analysis complete!")

if __name__ == "__main__":
    main()