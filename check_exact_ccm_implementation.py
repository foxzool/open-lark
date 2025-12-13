import os
import csv
from collections import defaultdict

# 读取API列表
ccm_apis = []
with open('/Users/zool/RustroverProjects/open-lark/api_list_export.csv', 'r', encoding='utf-8') as f:
    reader = csv.DictReader(f)
    for row in reader:
        if row['bizTag'] == 'ccm':
            ccm_apis.append(row)

# 按project分组
project_stats = defaultdict(list)
for api in ccm_apis:
    project = api['meta.Project']
    project_stats[project].append(api)

# 定义API到文件名的映射
api_file_mapping = {
    # ccm_doc (6个API)
    'create': 'create',
    'get_document_meta': 'meta',
    'get_sheet_meta': 'sheet_meta',
    'get_raw_content': 'raw_content',
    'get_content': 'content',
    'batch_update': 'batch_update',

    # ccm_docs (2个API)
    'search_object': 'search_object',
    'get_meta': 'meta',

    # ccm_drive_explorer (8个API)
    'get_root_folder_meta': 'root_folder_meta',
    'get_folder_meta': 'folder_meta',
    'get_folder_children': 'folder_children',
    'create_folder': 'folder',
    'create_file': 'file',
    'delete_spreadsheets': 'file/spreadsheets',
    'copy_files': 'file/copy',
    'delete_docs': 'file/docs',

    # ccm_drive_permission (3个API)
    'member_permitted': 'member/permitted',
    'member_transfer': 'member/transfer',
    'get_public_v2': 'public',

    # ccm_sheet (33个API)
    'sheets_batch_update': 'sheets_batch_update',
    'update_sheet_properties': 'sheets_batch_update',
    'add_dimension_range': 'adddimensionrange',
    'insert_dimension_range': 'insertdimensionrange',
    'update_dimension_range': 'updatedimensionrange',
    'delete_dimension_range': 'deletedimensionrange',
    'merge_cells': 'mergecells',
    'unmerge_cells': 'unmergecells',
    'set_style': 'setstyle',
    'styles_batch_update': 'batchsetstyle',
    'insert_values': 'insertvalues',
    'append_values': 'appendvalues',
    'values_prepend': 'values_prepend',
    'values_append': 'values_append',
    'values_image': 'values_image',
    'values_batch_get': 'values_batch_get',
    'values_batch_update': 'values_batch_update',
    'add_protected_range': 'addprotectedrange',
    'update_protected_range': 'updateprotectedrange',
    'get_protected_range': 'getprotectedrange',
    'delete_protected_range': 'deleteprotectedrange',
    'protected_dimension': 'protected_dimension',
    'set_dropdown': 'setdropdown',
    'update_dropdown': 'updatedropdown',
    'get_dropdown': 'getdropdown',
    'delete_dropdown': 'deletedropdown',
    'data_validation': 'dataValidation',
    'create_condition_format': 'createconditionformat',
    'update_condition_format': 'updateconditionformat',
    'delete_condition_format': 'deleteconditionformat',
    'get_condition_format': 'getconditionformat',
    'condition_formats': 'condition_formats',
    'metainfo': 'metainfo',
    'get_spreadsheet_meta': 'getspreadsheetmeta',
    'update_spreadsheet_properties': 'updatespreadsheetproperties',
    'import': 'import',
    'get_import_result': 'getimportresult',
    'write_image': 'writeimage',
    'read_single_range': 'readsinglerange',
    'read_multiple_ranges': 'readmultipleranges',
    'write_single_range': 'writesinglerange',
    'write_ranges': 'batchwriteranges',
    'operate_sheets': 'operatesheets',

    # docs (1个API)
    'content': 'content',

    # docx (19个API)
    'chat_announcement_get': 'chat/announcement/get',
    'chat_announcement_blocks_list': 'chat/announcement/blocks/list',
    'chat_announcement_block_children_create': 'chat/announcement/block/children/create',
    'chat_announcement_block_batch_update': 'chat/announcement/block/batch_update',
    'chat_announcement_block_get': 'chat/announcement/block/get',
    'chat_announcement_block_children_get': 'chat/announcement/block/children/get',
    'chat_announcement_block_children_batch_delete': 'chat/announcement/block/children/batch_delete',
    'document_create': 'document',
    'document_get': 'document/get',
    'document_raw_content': 'document/raw_content',
    'document_blocks_list': 'document/blocks/list',
    'document_block_children_create': 'document/block/children/create',
    'document_block_descendant_create': 'document/block/descendant/create',
    'document_block_update': 'document/block/update',
    'document_block_get': 'document/block/get',
    'document_block_batch_update': 'document/block/batch_update',
    'document_block_children_get': 'document/block/children/get',
    'document_block_children_batch_delete': 'document/block/children/batch_delete',
    'document_blocks_convert': 'document/blocks/convert',

    # drive (59个API)
    'file_list': 'file/list',
    'file_create_folder': 'file/create_folder',
    'file_task_check': 'file/task_check',
    'meta_batch_query': 'meta/batch_query',
    'file_statistics': 'file/statistics',
    'file_view_records': 'file/view_records',
    'file_copy': 'file/copy',
    'file_move': 'file/move',
    'file_delete': 'file/delete',
    'file_create_shortcut': 'file/create_shortcut',
    'file_upload_all': 'file/upload_all',
    'file_upload_prepare': 'file/upload_prepare',
    'file_upload_part': 'file/upload_part',
    'file_upload_finish': 'file/upload_finish',
    'file_download': 'file/download',
    'import_task_create': 'import_task/create',
    'import_task_get': 'import_task/get',
    'export_task_create': 'export_task/create',
    'export_task_get': 'export_task/get',
    'export_task_file_download': 'export_task/file/download',
    'media_upload_all': 'media/upload_all',
    'media_upload_prepare': 'media/upload_prepare',
    'media_upload_part': 'media/upload_part',
    'media_upload_finish': 'media/upload_finish',
    'media_download': 'media/download',
    'media_batch_get_tmp_download_url': 'media/batch_get_tmp_download_url',
    'file_versions': 'file/versions',
    'file_version_get': 'file/version/get',
    'file_version_delete': 'file/version/delete',
    'file_likes': 'file/likes',
    'file_subscribe': 'file/subscribe',
    'file_get_subscribe': 'file/get_subscribe',
    'file_delete_subscribe': 'file/delete_subscribe',
    'permission_members_add': 'permission/members/add',
    'permission_members_batch_create': 'permission/members/batch_create',
    'permission_members_update': 'permission/members/update',
    'permission_members_get': 'permission/members/get',
    'permission_members_delete': 'permission/members/delete',
    'permission_members_transfer_owner': 'permission/members/transfer_owner',
    'permission_members_auth': 'permission/members/auth',
    'permission_public_update_v2': 'permission/v2/public/update',
    'permission_public_get_v2': 'permission/v2/public/get',
    'permission_public_password': 'permission/public/password',
    'permission_public_password_refresh': 'permission/public/password/refresh',
    'permission_public_password_delete': 'permission/public/password/delete',
    'file_comments': 'file/comments',
    'file_comments_batch_query': 'file/comments/batch_query',
    'file_comments_resolve': 'file/comments/resolve',
    'file_comments_add': 'file/comments/add',
    'file_comment_get': 'file/comment/get',
    'file_comment_replies_get': 'file/comment/replies/get',
    'file_comment_replies_update': 'file/comment/replies/update',
    'file_comment_replies_delete': 'file/comment/replies/delete',
    'file_subscriptions_get': 'file/subscriptions/get',
    'file_subscriptions_create': 'file/subscriptions/create',
    'file_subscriptions_update': 'file/subscriptions/update',
    'permission_public_update_v1': 'permission/public/update',
    'permission_public_get_v1': 'permission/public/get',

    # wiki (16个API)
    'spaces': 'spaces',
    'spaces_get': 'spaces/get',
    'spaces_create': 'spaces/create',
    'spaces_members': 'spaces/members',
    'spaces_members_add': 'spaces/members/add',
    'spaces_members_delete': 'spaces/members/delete',
    'spaces_setting_update': 'spaces/setting/update',
    'spaces_nodes': 'spaces/nodes',
    'spaces_nodes_get': 'spaces/get_node',
    'spaces_nodes_move': 'spaces/nodes/move',
    'spaces_nodes_update_title': 'spaces/nodes/update_title',
    'spaces_nodes_copy': 'spaces/nodes/copy',
    'spaces_nodes_move_docs_to_wiki': 'spaces/nodes/move_docs_to_wiki',
    'tasks_get': 'tasks/get',
    'nodes_search': 'nodes/search',

    # sheets (27个API)
    'spreadsheets': 'spreadsheets',
    'spreadsheets_update': 'spreadsheets/update',
    'spreadsheets_get': 'spreadsheets/get',
    'sheets_query': 'sheets/query',
    'sheets_get': 'sheets/get',
    'sheets_move_dimension': 'sheets/move_dimension',
    'sheets_filter': 'sheets/filter',
    'sheets_filter_update': 'sheets/filter/update',
    'sheets_filter_get': 'sheets/filter/get',
    'sheets_filter_delete': 'sheets/filter/delete',
    'sheets_filter_views': 'sheets/filter_views',
    'sheets_filter_views_update': 'sheets/filter_views/update',
    'sheets_filter_views_query': 'sheets/filter_views/query',
    'sheets_filter_views_get': 'sheets/filter_views/get',
    'sheets_filter_views_delete': 'sheets/filter_views/delete',
    'sheets_filter_views_conditions': 'sheets/filter_views/conditions',
    'sheets_filter_views_conditions_update': 'sheets/filter_views/conditions/update',
    'sheets_filter_views_conditions_get': 'sheets/filter_views/conditions/get',
    'sheets_filter_views_conditions_delete': 'sheets/filter_views/conditions/delete',
    'sheets_float_images': 'sheets/float_images',
    'sheets_float_images_update': 'sheets/float_images/update',
    'sheets_float_images_get': 'sheets/float_images/get',
    'sheets_float_images_query': 'sheets/float_images/query',
    'sheets_float_images_delete': 'sheets/float_images/delete',
}

# 检查现有实现
base_path = '/Users/zool/RustroverProjects/open-lark/crates/openlark-docs/src/ccm'

def extract_api_name_from_url(url):
    """从URL提取API操作名称"""
    method = url.split(':')[0].upper()
    path = url.split(':')[1] if ':' in url else ''

    # 移除开头的 /
    if path.startswith('/'):
        path = path[1:]

    # 替换动态参数
    path = path.replace(':file_token', '')
    path = path.replace(':folderToken', '')
    path = path.replace(':document_id', '')
    path = path.replace(':chat_id', '')
    path = path.replace(':block_id', '')
    path = path.replace(':space_id', '')
    path = path.replace(':member_id', '')
    path = path.replace(':node_token', '')
    path = path.replace(':sheet_id', '')
    path = path.replace(':spreadsheet_token', '')
    path = path.replace(':spreadsheetToken', '')
    path = path.replace(':comment_id', '')
    path = path.replace(':reply_id', '')
    path = path.replace(':subscription_id', '')
    path = path.replace(':version_id', '')
    path = path.replace(':filter_view_id', '')
    path = path.replace(':condition_id', '')
    path = path.replace(':dataValidationId', '')
    path = path.replace(':sheetId', '')
    path = path.replace(':task_id', '')
    path = path.replace(':ticket', '')
    path = path.replace(':file_token/download', 'download')

    # 移除末尾的 /
    if path.endswith('/'):
        path = path[:-1]

    # 分割路径
    parts = path.split('/')
    if len(parts) > 0:
        return parts[-1]
    return ""

def check_api_exists_in_mod_file(project, api_name):
    """检查API是否在mod.rs中定义"""
    project_path = os.path.join(base_path, project)

    # 查找所有的mod.rs文件
    for root, dirs, files in os.walk(project_path):
        if 'mod.rs' in files:
            mod_file = os.path.join(root, 'mod.rs')
            try:
                with open(mod_file, 'r', encoding='utf-8') as f:
                    content = f.read()
                    # 检查是否有对应的pub fn定义
                    if f'pub fn {api_name}(' in content or f'pub fn {api_name.replace("_", "")}(' in content:
                        return True
            except:
                pass
    return False

def check_implementation(project, api_name, url):
    """检查API是否已实现"""
    # 首先检查是否在mod.rs中定义
    if check_api_exists_in_mod_file(project, api_name):
        return True, f"API {api_name} found in mod.rs"

    # 构建可能的文件路径
    project_path = os.path.join(base_path, project)

    # 检查各种可能的目录结构
    possible_dirs = [
        os.path.join(project_path, 'old', 'v1'),
        os.path.join(project_path, 'old', 'v2'),
        os.path.join(project_path, 'v1'),
        os.path.join(project_path, 'v2'),
        os.path.join(project_path, 'v3'),
    ]

    for possible_dir in possible_dirs:
        if os.path.exists(possible_dir):
            # 直接查找对应名称的文件
            for root, dirs, files in os.walk(possible_dir):
                for file in files:
                    if file.endswith('.rs') and file != 'mod.rs' and file != 'models.rs':
                        # 文件名转换为小写进行比较
                        file_base = file.replace('.rs', '').lower()
                        if api_name.lower() in file_base or file_base in api_name.lower():
                            return True, os.path.join(root, file)

    return False, f"API {api_name} not found"

# 统计实现情况
total_apis = 0
implemented_apis = 0
missing_apis = []

print("=== CCM API精确实现情况检查 ===\n")

for project, apis in project_stats.items():
    print(f"\n{project}: {len(apis)}个API")
    project_implemented = 0

    for api in apis:
        total_apis += 1
        # 从URL提取API操作名称
        api_name = extract_api_name_from_url(api['url'])

        # 特殊处理某些API名称
        if api_name == 'announcement':
            if 'announcement' in api['url']:
                api_name = 'chat_announcement_get'
        elif api_name == 'blocks' and 'announcement' in api['url']:
            api_name = 'chat_announcement_blocks_list'
        elif api_name == 'children' and 'announcement' in api['url']:
            if 'POST' in api['url']:
                api_name = 'chat_announcement_block_children_create'
            else:
                api_name = 'chat_announcement_block_children_get'
        elif api_name == 'batch_update' and 'announcement' in api['url']:
            api_name = 'chat_announcement_block_batch_update'
        elif api_name == 'announcement' and 'chat' in api['url']:
            api_name = 'chat_announcement_get'
        elif api_name == 'block' and 'announcement' in api['url']:
            if 'GET' in api['url'] and ':' in api['url']:
                api_name = 'chat_announcement_block_get'
            elif 'PATCH' in api['url']:
                api_name = 'chat_announcement_block_batch_update'
            elif 'DELETE' in api['url'] and 'children' in api['url']:
                api_name = 'chat_announcement_block_children_batch_delete'
        elif api_name == 'document' and 'docx' in api['url']:
            if 'POST' in api['url']:
                api_name = 'document_create'
            else:
                api_name = 'document_get'
        elif api_name == 'documents' and 'docx' in api['url']:
            api_name = 'document_create'
        elif api_name == 'blocks' and 'docx' in api['url']:
            if 'POST' in api['url'] and 'convert' in api['url']:
                api_name = 'document_blocks_convert'
            else:
                api_name = 'document_blocks_list'
        elif api_name == 'children' and 'docx' in api['url']:
            if 'POST' in api['url'] and 'descendant' in api['url']:
                api_name = 'document_block_descendant_create'
            else:
                api_name = 'document_block_children_create'
        elif api_name == 'descendant' and 'docx' in api['url']:
            api_name = 'document_block_descendant_create'
        elif api_name == 'batch_delete' and 'docx' in api['url']:
            api_name = 'document_block_children_batch_delete'
        elif api_name == 'convert' and 'docx' in api['url']:
            api_name = 'document_blocks_convert'
        elif api_name == 'batch_query' and 'drive' in api['url']:
            api_name = 'meta_batch_query'
        elif api_name == 'upload_prepare' and 'drive' in api['url']:
            api_name = 'file_upload_prepare'
        elif api_name == 'upload_part' and 'drive' in api['url']:
            api_name = 'file_upload_part'
        elif api_name == 'upload_finish' and 'drive' in api['url']:
            api_name = 'file_upload_finish'
        elif api_name == 'download' and 'drive' in api['url']:
            if 'export_tasks' in api['url']:
                api_name = 'export_task_file_download'
            elif 'medias' in api['url']:
                api_name = 'media_download'
            else:
                api_name = 'file_download'
        elif api_name == 'batch_get_tmp_download_url' and 'drive' in api['url']:
            api_name = 'media_batch_get_tmp_download_url'
        elif api_name == 'versions' and 'drive' in api['url']:
            api_name = 'file_versions'
        elif api_name == 'public' and 'drive' in api['url']:
            if 'v2' in api['url']:
                api_name = 'permission_public_get_v2'
            else:
                api_name = 'permission_public_get_v1'
        elif api_name == 'password' and 'drive' in api['url']:
            if 'PUT' in api['url']:
                api_name = 'permission_public_password_refresh'
            elif 'DELETE' in api['url']:
                api_name = 'permission_public_password_delete'
            else:
                api_name = 'permission_public_password'
        elif api_name == 'members' and 'drive' in api['url']:
            if 'permissions' in api['url']:
                if 'GET' in api['url']:
                    api_name = 'permission_members_get'
                elif 'POST' in api['url'] and 'batch_create' in api['url']:
                    api_name = 'permission_members_batch_create'
                elif 'DELETE' in api['url']:
                    api_name = 'permission_members_delete'
                elif 'PUT' in api['url']:
                    api_name = 'permission_members_update'
                else:
                    api_name = 'permission_members_add'
            else:
                api_name = 'permission_members_add'
        elif api_name == 'auth' and 'drive' in api['url']:
            api_name = 'permission_members_auth'
        elif api_name == 'transfer_owner' and 'drive' in api['url']:
            api_name = 'permission_members_transfer_owner'
        elif api_name == 'batch_query' and 'drive' in api['url'] and 'comments' in api['url']:
            api_name = 'file_comments_batch_query'
        elif api_name == 'resolve' and 'drive' in api['url']:
            api_name = 'file_comments_resolve'
        elif api_name == 'replies' and 'drive' in api['url']:
            if 'GET' in api['url']:
                api_name = 'file_comment_replies_get'
            elif 'PUT' in api['url']:
                api_name = 'file_comment_replies_update'
            elif 'DELETE' in api['url']:
                api_name = 'file_comment_replies_delete'
        elif api_name == 'subscribe' and 'drive' in api['url']:
            if 'DELETE' in api['url']:
                api_name = 'file_delete_subscribe'
            else:
                api_name = 'file_subscribe'
        elif api_name == 'get_subscribe' and 'drive' in api['url']:
            api_name = 'file_get_subscribe'
        elif api_name == 'create' and 'wiki' in api['url']:
            if 'nodes' in api['url']:
                api_name = 'spaces_nodes_create'
            else:
                api_name = 'spaces_create'
        elif api_name == 'get_node' and 'wiki' in api['url']:
            api_name = 'spaces_nodes_get'
        elif api_name == 'move' and 'wiki' in api['url']:
            if 'docs' in api['url']:
                api_name = 'spaces_nodes_move_docs_to_wiki'
            else:
                api_name = 'spaces_nodes_move'
        elif api_name == 'update_title' and 'wiki' in api['url']:
            api_name = 'spaces_nodes_update_title'
        elif api_name == 'copy' and 'wiki' in api['url']:
            api_name = 'spaces_nodes_copy'
        elif api_name == 'get' and 'wiki' in api['url']:
            api_name = 'spaces_get'
        elif 'task' in api['url'] and 'wiki' in api['url']:
            api_name = 'tasks_get'
        elif api_name == 'conditions' and 'sheets' in api['url']:
            if 'GET' in api['url']:
                api_name = 'sheets_filter_views_conditions_get'
            else:
                api_name = 'sheets_filter_views_conditions'

        implemented, path = check_implementation(project, api_name, api['url'])
        if implemented:
            project_implemented += 1
            implemented_apis += 1
        else:
            missing_apis.append({
                'project': project,
                'name': api['name'],
                'url': api['url'],
                'api_name': api_name
            })

    print(f"  已实现: {project_implemented}/{len(apis)}")

print(f"\n总计:")
print(f"  总API数: {total_apis}")
print(f"  已实现: {implemented_apis}")
print(f"  缺失: {len(missing_apis)}")

if missing_apis:
    print(f"\n缺失的API:")
    for missing in missing_apis:
        print(f"  - {missing['project']}: {missing['name']} ({missing['api_name']}) - {missing['url']}")