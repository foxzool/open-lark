#!/usr/bin/env python3
"""
Manual fixes for the remaining incomplete EndpointBuilder calls.
This script handles the specific remaining issues that couldn't be auto-detected.
"""

import os
import re

def add_missing_endpoint_constants():
    """Add missing endpoint constants to the hire endpoints file"""
    endpoints_file = "/Users/zool/RustroverProjects/open-lark/crates/open-lark-core/src/endpoints/hire.rs"

    try:
        with open(endpoints_file, 'r', encoding='utf-8') as f:
            content = f.read()

        # Add missing endpoint constants at the end before the last line
        missing_endpoints = [
            "",
            "/// Additional endpoints that might be missing",
            "pub const HIRE_V1_JOB_DELETE: &str = \"/open-apis/hire/v1/jobs/{job_id}\";",
            "pub const HIRE_V1_JOB_PROCESS_DELETE: &str = \"/open-apis/hire/v1/job_processes/{job_process_id}\";",
            "pub const HIRE_V1_JOB_PROCESS_UPDATE: &str = \"/open-apis/hire/v1/job_processes/{job_process_id}\";",
            "pub const HIRE_V1_JOB_REQUIREMENT_DELETE: &str = \"/open-apis/hire/v1/job_requirements/{job_requirement_id}\";",
            "pub const HIRE_V1_JOB_REQUIREMENT_UPDATE: &str = \"/open-apis/hire/v1/job_requirements/{job_requirement_id}\";",
            "pub const HIRE_V1_SUBJECT_DELETE: &str = \"/open-apis/hire/v1/subjects/{subject_id}\";",
            "pub const HIRE_V1_SUBJECT_UPDATE: &str = \"/open-apis/hire/v1/subjects/{subject_id}\";",
            "pub const HIRE_V1_EXAM_PAPER_DELETE: &str = \"/open-apis/hire/v1/exam_papers/{exam_paper_id}\";",
            "pub const HIRE_V1_EXAM_RECORD_UPDATE: &str = \"/open-apis/hire/v1/exam_records/{exam_record_id}\";",
            "pub const HIRE_V1_INTERVIEW_SETTING_DELETE: &str = \"/open-apis/hire/v1/interview_settings/{interview_setting_id}\";",
            "pub const HIRE_V1_INTERVIEW_SETTING_UPDATE: &str = \"/open-apis/hire/v1/interview_settings/{interview_setting_id}\";",
            "pub const HIRE_V1_OFFER_SETTING_DELETE: &str = \"/open-apis/hire/v1/offer_settings/{offer_setting_id}\";",
            "pub const HIRE_V1_OFFER_SETTING_UPDATE: &str = \"/open-apis/hire/v1/offer_settings/{offer_setting_id}\";",
            "pub const HIRE_V1_TALENT_POOL_DELETE: &str = \"/open-apis/hire/v1/talent_pools/{talent_pool_id}\";",
            "pub const HIRE_V1_TALENT_POOL_UPDATE: &str = \"/open-apis/hire/v1/talent_pools/{talent_pool_id}\";",
            "pub const HIRE_V1_INTERVIEW_ARRANGEMENT_UPDATE: &str = \"/open-apis/hire/v1/interview_arrangements/{interview_id}\";",
            "pub const HIRE_V1_AGENCY_RECOMMENDATION_DELETE: &str = \"/open-apis/hire/v1/agency_recommendations/{recommendation_id}\";",
            "pub const HIRE_V1_AGENCY_RECOMMENDATION_UPDATE: &str = \"/open-apis/hire/v1/agency_recommendations/{recommendation_id}\";",
            "pub const HIRE_V1_EXTERNAL_SYSTEM_DELETE: &str = \"/open-apis/hire/v1/external_systems/{system_id}\";",
            "pub const HIRE_V1_BACKGROUND_CHECK_ORDER_UPDATE: &str = \"/open-apis/hire/v1/background_check_orders/{order_id}\";",
            "pub const HIRE_V1_APPLICATION_OFFER: &str = \"/open-apis/hire/v1/applications/{application_id}/offer\";",
            "pub const HIRE_V1_APPLICATION_EVALUATIONS_ADD: &str = \"/open-apis/hire/v1/applications/{application_id}/evaluations\";",
            "pub const HIRE_V1_TALENT_POOL_ADD_TALENT: &str = \"/open-apis/hire/v1/talent_pools/{talent_pool_id}/talents\";",
            "pub const HIRE_V1_TALENT_POOL_REMOVE_TALENT: &str = \"/open-apis/hire/v1/talent_pools/{talent_pool_id}/talents/{talent_id}\";",
        ]

        # Check if these endpoints already exist
        for endpoint in missing_endpoints:
            if endpoint and "pub const" in endpoint and endpoint not in content:
                content += "\n" + endpoint

        with open(endpoints_file, 'w', encoding='utf-8') as f:
            f.write(content)

        print("Added missing endpoint constants")
        return True

    except Exception as e:
        print(f"Error updating endpoints file: {e}")
        return False

def fix_specific_endpoint_calls():
    """Fix specific incomplete endpoint calls in files"""

    # Mapping of file path to list of (function_name, endpoint_constant, line_pattern) tuples
    fixes = [
        # background_check module
        ("/Users/zool/RustroverProjects/open-lark/crates/open-lark-hr-suite/src/hire/ecological_docking/background_check/mod.rs", [
            ("get_order_detail", "HIRE_V1_BACKGROUND_CHECK_ORDER_GET"),
            ("cancel_order", "HIRE_V1_BACKGROUND_CHECK_ORDER_CANCEL"),
            ("get_report", "HIRE_V1_BACKGROUND_CHECK_ORDER_REPORT"),
        ]),

        # exam module
        ("/Users/zool/RustroverProjects/open-lark/crates/open-lark-hr-suite/src/hire/ecological_docking/exam/mod.rs", [
            ("get_record_detail", "HIRE_V1_EXAM_RECORD_GET"),
            ("cancel_exam", "HIRE_V1_EXAM_RECORD_CANCEL"),
            ("reschedule_exam", "HIRE_V1_EXAM_RECORD_RESCHEDULE"),
        ]),

        # talent_pool module
        ("/Users/zool/RustroverProjects/open-lark/crates/open-lark-hr-suite/src/hire/candidate_management/talent_pool/mod.rs", [
            ("add_talent_to_pool", "HIRE_V1_TALENT_POOL_ADD_TALENT"),
            ("remove_talent_from_pool", "HIRE_V1_TALENT_POOL_REMOVE_TALENT"),
        ]),

        # offer_settings module
        ("/Users/zool/RustroverProjects/open-lark/crates/open-lark-hr-suite/src/hire/recruitment_config/offer_settings/mod.rs", [
            ("get_settings_detail", "HIRE_V1_OFFER_SETTING_GET"),
            ("update_settings", "HIRE_V1_OFFER_SETTING_UPDATE"),
            ("delete_settings", "HIRE_V1_OFFER_SETTING_DELETE"),
        ]),

        # job_process module
        ("/Users/zool/RustroverProjects/open-lark/crates/open-lark-hr-suite/src/hire/recruitment_config/job_process/mod.rs", [
            ("get_process_detail", "HIRE_V1_JOB_PROCESS_GET"),
            ("update_process", "HIRE_V1_JOB_PROCESS_UPDATE"),
            ("delete_process", "HIRE_V1_JOB_PROCESS_DELETE"),
        ]),

        # job_requirement module
        ("/Users/zool/RustroverProjects/open-lark/crates/open-lark-hr-suite/src/hire/recruitment_config/job_requirement/mod.rs", [
            ("get_requirement_detail", "HIRE_V1_JOB_REQUIREMENT_GET"),
            ("update_requirement", "HIRE_V1_JOB_REQUIREMENT_UPDATE"),
            ("delete_requirement", "HIRE_V1_JOB_REQUIREMENT_DELETE"),
        ]),

        # subject module
        ("/Users/zool/RustroverProjects/open-lark/crates/open-lark-hr-suite/src/hire/recruitment_config/subject/mod.rs", [
            ("enable_subject", "HIRE_V1_SUBJECT_ENABLE"),
            ("disable_subject", "HIRE_V1_SUBJECT_DISABLE"),
        ]),

        # interview_settings module
        ("/Users/zool/RustroverProjects/open-lark/crates/open-lark-hr-suite/src/hire/recruitment_config/interview_settings/mod.rs", [
            ("get_settings_detail", "HIRE_V1_INTERVIEW_SETTING_GET"),
            ("update_settings", "HIRE_V1_INTERVIEW_SETTING_UPDATE"),
            ("delete_settings", "HIRE_V1_INTERVIEW_SETTING_DELETE"),
        ]),

        # job module
        ("/Users/zool/RustroverProjects/open-lark/crates/open-lark-hr-suite/src/hire/recruitment_config/job/mod.rs", [
            ("update_job", "HIRE_V1_JOB_COMBINED_UPDATE"),
        ]),

        # agency module
        ("/Users/zool/RustroverProjects/open-lark/crates/open-lark-hr-suite/src/hire/get_candidates/agency/mod.rs", [
            ("confirm_recommendation", "HIRE_V1_AGENCY_RECOMMENDATION_CONFIRM"),
            ("reject_recommendation", "HIRE_V1_AGENCY_RECOMMENDATION_REJECT"),
        ]),
    ]

    fixed_count = 0

    for file_path, function_fixes in fixes:
        if not os.path.exists(file_path):
            continue

        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()

            original_content = content

            for function_name, endpoint_const in function_fixes:
                # Find the function and fix its endpoint call
                pattern = r'(pub async fn ' + function_name + r'\([^)]+\)[^{]*\{[^}]*api_req\.set_api_path\()EndpointBuilder::replace_param\(\);([^}]*})'

                def replace_func(match):
                    return match.group(1) + endpoint_const + '.to_string());' + match.group(2)

                content = re.sub(pattern, replace_func, content, flags=re.DOTALL)

            # Also fix some incorrect mappings from previous script
            # Fix referral_account incorrect mappings
            if "referral_account" in file_path:
                content = content.replace(
                    "api_req.set_api_path(HIRE_REFERRAL_ACCOUNT_BALANCE.to_string());",
                    "api_req.set_api_path(HIRE_REFERRAL_WITHDRAWAL_APPROVE.to_string());",
                    1  # Only replace first occurrence
                )
                content = content.replace(
                    "api_req.set_api_path(HIRE_REFERRAL_ACCOUNT_BALANCE.to_string());",
                    "api_req.set_api_path(HIRE_REFERRAL_ACCOUNT_ENABLE.to_string());",
                    1  # Only replace first occurrence
                )
                content = content.replace(
                    "api_req.set_api_path(HIRE_REFERRAL_ACCOUNT_BALANCE.to_string());",
                    "api_req.set_api_path(HIRE_REFERRAL_ACCOUNT_DISABLE.to_string());",
                    1  # Only replace first occurrence
                )

            # Fix referral module incorrect mappings
            if "get_candidates/referral" in file_path:
                content = content.replace(
                    "api_req.set_api_path(HIRE_V1_REFERRAL_GET.to_string());",
                    "api_req.set_api_path(HIRE_V1_REFERRAL_ACCOUNT_GET.to_string());",
                    1  # Only replace first occurrence
                )
                content = content.replace(
                    "api_req.set_api_path(HIRE_V1_REFERRAL_GET.to_string());",
                    "api_req.set_api_path(HIRE_V1_REFERRAL_GRANT_REWARD.to_string());",
                    1  # Only replace first occurrence
                )

            # Fix website module incorrect mapping
            if "get_candidates/website" in file_path:
                content = content.replace(
                    "api_req.set_api_path(HIRE_V1_WEBSITE_JOB_UNPUBLISH.to_string());",
                    "api_req.set_api_path(HIRE_V1_WEBSITE_APPLICATION_CONVERT.to_string());",
                    1  # Only replace first occurrence
                )

            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixed_count += 1
                print(f"Fixed endpoint calls in {os.path.basename(file_path)}")

        except Exception as e:
            print(f"Error processing {file_path}: {e}")

    return fixed_count

def main():
    """Main function"""
    print("Adding missing endpoint constants...")
    add_missing_endpoint_constants()

    print("\nFixing specific endpoint calls...")
    fixed_files = fix_specific_endpoint_calls()

    print(f"\nFixed endpoint calls in {fixed_files} files")

    # Final check
    print("\nFinal check for remaining issues...")
    remaining = 0
    hire_dir = "/Users/zool/RustroverProjects/open-lark/crates/open-lark-hr-suite/src/hire"

    for root, dirs, files in os.walk(hire_dir):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        content = f.read()

                    if 'EndpointBuilder::replace_param();' in content:
                        print(f"Remaining issue: {os.path.relpath(file_path, hire_dir)}")
                        remaining += 1

                except Exception as e:
                    print(f"Error checking {file_path}: {e}")

    if remaining == 0:
        print("✅ All EndpointBuilder calls have been fixed!")
        return 0
    else:
        print(f"❌ {remaining} files still have issues")
        return 1

if __name__ == "__main__":
    import sys
    sys.exit(main())