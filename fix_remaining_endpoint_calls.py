#!/usr/bin/env python3
"""
Final script to fix all remaining incomplete EndpointBuilder calls in the hire module.
This script maps incomplete calls to correct endpoint constants based on context analysis.
"""

import os
import re
import sys

def get_endpoint_mapping():
    """Map context patterns to endpoint constants"""
    return {
        # referral_account module mappings
        'get_balance': 'HIRE_REFERRAL_ACCOUNT_BALANCE',
        'approve_withdrawal': 'HIRE_REFERRAL_WITHDRAWAL_APPROVE',
        'enable_account': 'HIRE_REFERRAL_ACCOUNT_ENABLE',
        'disable_account': 'HIRE_REFERRAL_ACCOUNT_DISABLE',

        # website module mappings
        'unpublish_job_from_website': 'HIRE_V1_WEBSITE_JOB_UNPUBLISH',
        'convert_website_application': 'HIRE_V1_WEBSITE_APPLICATION_CONVERT',

        # job module mappings
        'delete_job': 'HIRE_V1_JOB_DELETE',
        'get_job_detail': 'HIRE_V1_JOB_GET_DETAIL',

        # job_process module mappings
        'delete_job_process': 'HIRE_V1_JOB_PROCESS_DELETE',
        'get_job_process_detail': 'HIRE_V1_JOB_PROCESS_GET',
        'update_job_process': 'HIRE_V1_JOB_PROCESS_UPDATE',

        # agency module mappings
        'delete_agency_recommendation': 'HIRE_V1_AGENCY_RECOMMENDATION_DELETE',
        'update_agency_recommendation': 'HIRE_V1_AGENCY_RECOMMENDATION_UPDATE',

        # offer_settings module mappings
        'delete_offer_setting': 'HIRE_V1_OFFER_SETTING_DELETE',
        'get_offer_setting_detail': 'HIRE_V1_OFFER_SETTING_GET',
        'update_offer_setting': 'HIRE_V1_OFFER_SETTING_UPDATE',

        # job_requirement module mappings
        'delete_job_requirement': 'HIRE_V1_JOB_REQUIREMENT_DELETE',
        'get_job_requirement_detail': 'HIRE_V1_JOB_REQUIREMENT_GET',
        'update_job_requirement': 'HIRE_V1_JOB_REQUIREMENT_UPDATE',

        # subject module mappings
        'delete_subject': 'HIRE_V1_SUBJECT_DELETE',
        'update_subject': 'HIRE_V1_SUBJECT_UPDATE',

        # exam module mappings
        'delete_exam_paper': 'HIRE_V1_EXAM_PAPER_DELETE',
        'get_exam_record_detail': 'HIRE_V1_EXAM_RECORD_GET',
        'update_exam_record': 'HIRE_V1_EXAM_RECORD_UPDATE',

        # interview_settings module mappings
        'delete_interview_setting': 'HIRE_V1_INTERVIEW_SETTING_DELETE',
        'get_interview_setting_detail': 'HIRE_V1_INTERVIEW_SETTING_GET',
        'update_interview_setting': 'HIRE_V1_INTERVIEW_SETTING_UPDATE',

        # referral module mappings
        'get_referral_detail': 'HIRE_V1_REFERRAL_GET',
        'get_referral_account': 'HIRE_V1_REFERRAL_ACCOUNT_GET',
        'grant_referral_reward': 'HIRE_V1_REFERRAL_GRANT_REWARD',

        # external_system module mappings
        'delete_external_system': 'HIRE_V1_EXTERNAL_SYSTEM_DELETE',
        'convert_external_candidate': 'HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES_CONVERT',

        # background_check module mappings
        'cancel_background_check': 'HIRE_V1_BACKGROUND_CHECK_ORDER_CANCEL',
        'get_background_check_report': 'HIRE_V1_BACKGROUND_CHECK_ORDER_REPORT',
        'update_background_check_order': 'HIRE_V1_BACKGROUND_CHECK_ORDER_UPDATE',

        # application module mappings
        'get_application_detail': 'HIRE_V1_APPLICATION_GET',
        'reject_application': 'HIRE_V1_APPLICATION_REJECT',
        'arrange_interview': 'HIRE_V1_APPLICATION_INTERVIEWS',
        'send_offer': 'HIRE_V1_APPLICATION_OFFER',
        'advance_application': 'HIRE_V1_APPLICATION_ADVANCE',

        # talent_pool module mappings
        'delete_talent_pool': 'HIRE_V1_TALENT_POOL_DELETE',
        'update_talent_pool': 'HIRE_V1_TALENT_POOL_UPDATE',

        # interview module mappings
        'cancel_interview': 'HIRE_V1_INTERVIEW_CANCEL',
        'reschedule_interview': 'HIRE_V1_INTERVIEW_RESCHEDULE',
        'update_interview_arrangement': 'HIRE_V1_INTERVIEW_ARRANGEMENT_UPDATE',
    }

def extract_function_name(content, line_num):
    """Extract the function name that contains the incomplete EndpointBuilder call"""
    lines = content.split('\n')

    # Search backwards from the error line to find the function definition
    for i in range(line_num - 1, max(0, line_num - 50), -1):
        line = lines[i].strip()
        if line.startswith('pub async fn '):
            # Extract function name
            match = re.search(r'pub async fn (\w+)', line)
            if match:
                return match.group(1)
        elif line.startswith('async fn '):
            # Extract function name
            match = re.search(r'async fn (\w+)', line)
            if match:
                return match.group(1)

    return None

def fix_incomplete_endpoint_calls(file_path):
    """Fix incomplete EndpointBuilder calls in a file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content
        lines = content.split('\n')
        endpoint_mapping = get_endpoint_mapping()

        # Find all incomplete EndpointBuilder calls
        for line_num, line in enumerate(lines, 1):
            if 'EndpointBuilder::replace_param();' in line:
                # Extract function name to determine correct endpoint
                function_name = extract_function_name(content, line_num)

                if function_name and function_name in endpoint_mapping:
                    endpoint_const = endpoint_mapping[function_name]

                    # Determine if this is a DELETE/GET/POST request based on context
                    method = 'GET'  # default
                    for i in range(max(0, line_num - 5), line_num):
                        if 'set_http_method(Method::' in lines[i]:
                            method_match = re.search(r'Method::(\w+)', lines[i])
                            if method_match:
                                method = method_match.group(1)
                            break

                    # Replace the incomplete call
                    if method in ['DELETE', 'GET'] and '{' in endpoint_const:
                        # For endpoints with path parameters
                        old_line = line.strip()
                        new_line = f'        api_req.set_api_path({endpoint_const}.to_string());'
                        content = content.replace(old_line, new_line)
                        print(f"Fixed {function_name} in {os.path.basename(file_path)} -> {endpoint_const}")
                    else:
                        # For simple endpoints
                        old_line = line.strip()
                        new_line = f'        api_req.set_api_path({endpoint_const}.to_string());'
                        content = content.replace(old_line, new_line)
                        print(f"Fixed {function_name} in {os.path.basename(file_path)} -> {endpoint_const}")
                else:
                    print(f"Could not determine endpoint for function '{function_name}' in {os.path.basename(file_path)}")

        # Fix any remaining double semicolons
        content = re.sub(r';;\s*$', ';', content, flags=re.MULTILINE)

        # Write back if changed
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return True

        return False

    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    """Main function to fix all incomplete EndpointBuilder calls"""
    hire_dir = "/Users/zool/RustroverProjects/open-lark/crates/open-lark-hr-suite/src/hire"

    if not os.path.exists(hire_dir):
        print(f"Directory not found: {hire_dir}")
        return 1

    fixed_files = 0

    # Process all Rust files in the hire directory
    for root, dirs, files in os.walk(hire_dir):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                if fix_incomplete_endpoint_calls(file_path):
                    fixed_files += 1

    print(f"\nFixed incomplete EndpointBuilder calls in {fixed_files} files")

    # Check for any remaining issues
    print("\nChecking for remaining issues...")
    remaining_issues = 0

    for root, dirs, files in os.walk(hire_dir):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        content = f.read()

                    if 'EndpointBuilder::replace_param();' in content:
                        print(f"Remaining issue in: {file_path}")
                        remaining_issues += 1

                except Exception as e:
                    print(f"Error checking {file_path}: {e}")

    if remaining_issues == 0:
        print("✅ All incomplete EndpointBuilder calls have been fixed!")
    else:
        print(f"❌ {remaining_issues} files still have incomplete EndpointBuilder calls")

    return 0 if remaining_issues == 0 else 1

if __name__ == "__main__":
    sys.exit(main())