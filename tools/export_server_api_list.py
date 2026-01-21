#!/usr/bin/env python3
"""
从飞书开放平台「服务端 API 列表」导出数据，并生成与仓库根目录 `api_list_export.csv` 相同列结构的 CSV。

数据来源（均为开放平台页面使用的数据接口）：
1) API 目录树（用于拿到所有 API 的 id / fullPath / apiIdentity 等）：
   https://open.feishu.cn/api_explorer/v1/api_catalog
2) 文档详情（用于补齐 description / chargingMethod / updateTime / redirectUri 等）：
   https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=...

用法：
    python tools/export_server_api_list.py --output api_list_export.csv

说明：
- 脚本尽量不依赖第三方库，使用标准库实现 HTTP 请求与并发抓取。
- 旧版（apiIdentity 缺失）文档会从 content 中解析 HTTP Method / URL / 支持的应用类型等信息。
"""

from __future__ import annotations

import argparse
import csv
import json
import re
import sys
import time
import urllib.parse
import urllib.request
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass
from typing import Any, Dict, Iterable, List, Optional, Tuple


CATALOG_URL = "https://open.feishu.cn/api_explorer/v1/api_catalog"
DOC_DETAIL_URL = "https://open.feishu.cn/document_portal/v1/document/get_detail"
DOC_BASE_URL = "https://open.feishu.cn/document"

# CSV 列顺序必须与仓库内 api_list_export.csv 一致
CSV_HEADER = [
    "id",
    "name",
    "bizTag",
    "meta.Project",
    "meta.Version",
    "meta.Resource",
    "meta.Name",
    "detail",
    "chargingMethod",
    "fullDose",
    "fullPath",
    "url",
    "orderMark",
    "supportAppTypes",
    "tags",
    "updateTime",
    "isCharge",
    "meta.Type",
    "docPath",
]

# meta.Project -> bizTag 的归类（与仓库现有数据保持一致）
PROJECT_BIZTAG_OVERRIDES: Dict[str, str] = {
    "apaas": "app_engine",
    "authen": "auth",
    "bitable": "base",
    "card": "im",
    "ccm_doc": "ccm",
    "ccm_docs": "ccm",
    "ccm_drive_explorer": "ccm",
    "ccm_drive_permission": "ccm",
    "ccm_sheet": "ccm",
    "compensation": "compensation_management",
    "contact_search": "contact",
    "contact_user": "contact",
    "corehr": "feishu_people",
    "docs": "ccm",
    "document_ai": "ai",
    "docx": "ccm",
    "drive": "ccm",
    "face_verify": "human_authentication",
    "im_ephemeral": "im",
    "im_message": "im",
    "lingo": "baike",
    "oauth": "auth",
    "optical_char_recognition": "ai",
    "sheets": "ccm",
    "speech_to_text": "ai",
    "translation": "ai",
    "vc_meeting": "meeting_room",
    "verification": "verification_information",
    "wiki": "ccm",
}


# 旧版（apiIdentity 缺失）API 的 meta.Project 以及 meta.Name 的前缀截取规则
# 返回 (meta_project, prefix_to_strip_for_meta_name)
OLD_API_PATH_RULES: List[Tuple[str, str, str]] = [
    ("approval", "/approval/openapi/", "/approval/openapi/"),
    ("ccm_sheet", "/open-apis/sheets/v2/", "/open-apis/sheets/v2/"),
    ("ccm_drive_explorer", "/open-apis/drive/explorer/", "/open-apis/drive/explorer/"),
    ("ccm_doc", "/open-apis/doc/v2/", "/open-apis/doc/v2/"),
    ("ccm_drive_permission", "/open-apis/drive/permission/", "/open-apis/drive/permission/"),
    ("im_ephemeral", "/open-apis/ephemeral/", "/open-apis/ephemeral/"),
    ("im_message", "/open-apis/message/v4/batch_send/", "/open-apis/message/"),
    ("card", "/open-apis/interactive/", "/open-apis/interactive/"),
    ("vc_meeting", "/open-apis/meeting_room/", "/open-apis/meeting_room/"),
    ("contact_user", "/open-apis/user/v4/app_admin_user/", "/open-apis/user/"),
    ("oauth", "/open-apis/authen/v1/index", "/open-apis/authen/"),
    ("ccm_docs", "/open-apis/suite/docs-api/", "/open-apis/suite/"),
    ("contact_search", "/open-apis/search/", "/open-apis/search/"),
    ("application", "/open-apis/application/", "/open-apis/application/"),
    ("contact", "/open-apis/contact/", "/open-apis/contact/"),
    ("face_verify", "/open-apis/face_verify/", "/open-apis/face_verify/"),
]


def _user_agent() -> str:
    return "open-lark-tools/1.0 (+https://github.com/openai/codex-cli)"


def http_get_json(url: str, timeout: int, retries: int) -> Dict[str, Any]:
    last_err: Optional[Exception] = None
    for attempt in range(retries + 1):
        try:
            req = urllib.request.Request(url, headers={"User-Agent": _user_agent()})
            with urllib.request.urlopen(req, timeout=timeout) as resp:
                raw = resp.read()
            return json.loads(raw.decode("utf-8"))
        except Exception as e:  # noqa: BLE001 - 脚本工具允许兜底重试
            last_err = e
            if attempt < retries:
                time.sleep(min(2 ** attempt, 8))
                continue
            raise
    raise RuntimeError(str(last_err))


def http_get_doc_detail(full_path: str, timeout: int, retries: int) -> Dict[str, Any]:
    url = DOC_DETAIL_URL + "?" + urllib.parse.urlencode({"fullPath": full_path})
    return http_get_json(url, timeout=timeout, retries=retries)


def iter_api_leaves(items: Iterable[Dict[str, Any]]) -> Iterable[Dict[str, Any]]:
    stack = list(items)
    while stack:
        node = stack.pop(0)
        children = node.get("children") or []
        if children:
            stack[0:0] = children
            continue
        if node.get("type") == 1:
            yield node


def normalize_support_app_types(types: Iterable[str]) -> List[str]:
    seen = set()
    cleaned: List[str] = []
    for t in types:
        if not t:
            continue
        v = str(t).strip().lower()
        if not v or v in seen:
            continue
        seen.add(v)
        cleaned.append(v)

    priority = {"isv": 0, "custom": 1}
    return sorted(cleaned, key=lambda x: (priority.get(x, 99), x))


def extract_support_app_types_from_content(content: str) -> List[str]:
    if not content:
        return []
    # 例：<md-app-support types="custom,isv"></md-app-support>
    m = re.search(r'<md-app-support[^>]*\stypes="([^"]+)"', content)
    if not m:
        return []
    raw = m.group(1)
    parts = [p.strip().lower() for p in raw.split(",") if p.strip()]
    return normalize_support_app_types(parts)


def extract_description(schema: Any, content: str) -> str:
    if isinstance(schema, dict):
        desc = schema.get("description")
        if isinstance(desc, str) and desc.strip():
            return desc.strip()

    if not content:
        return ""

    # 尽量取标题之后的第一段文本作为简介
    lines = [ln.rstrip() for ln in content.splitlines()]
    i = 0
    # 跳过开头的标题行（通常以 "# " 开头）
    while i < len(lines) and not lines[i].strip():
        i += 1
    if i < len(lines) and lines[i].lstrip().startswith("#"):
        i += 1
    # 跳过空行
    while i < len(lines) and not lines[i].strip():
        i += 1

    para: List[str] = []
    while i < len(lines):
        ln = lines[i].strip()
        if not ln:
            break
        # 避免把二级标题/列表当成简介
        if ln.startswith("#") or ln.startswith("##") or ln.startswith("- "):
            break
        para.append(ln)
        i += 1
    return "\n".join(para).strip()


def sanitize_singleline_text(text: str) -> str:
    """
    CSV 单行化：
    - 去掉换行（避免字段内换行导致部分工具解析异常）
    - 合并多余空白
    """
    if not text:
        return ""
    normalized = str(text).replace("\r\n", "\n").replace("\r", "\n").replace("\n", " ")
    normalized = re.sub(r"\s+", " ", normalized)
    return normalized.strip()


def parse_method_and_path_from_content(content: str) -> Tuple[str, str]:
    if not content:
        return "", ""

    # 常见：md-table 中有 “HTTP Method” 与 “TTP URL/URL”
    m_method = re.search(r"<md-th>\s*HTTP Method\s*</md-th>\s*<md-td>\s*([A-Z]+)\s*</md-td>", content)
    m_url = re.search(
        r"<md-th>\s*(?:TTP URL|HTTP URL|URL)\s*</md-th>\s*<md-td>\s*([^<]+)\s*</md-td>", content
    )
    method = (m_method.group(1).strip() if m_method else "").upper()
    path = ""
    if m_url:
        raw_url = m_url.group(1).strip()
        try:
            parsed = urllib.parse.urlparse(raw_url)
            path = parsed.path or ""
        except Exception:  # noqa: BLE001
            path = ""

    if method and path:
        return method, path

    # 兜底：从内容里直接找形如 GET:/open-apis/xxx 的片段
    m = re.search(r"\b(GET|POST|PUT|PATCH|DELETE)\b\s*[:：]\s*(/[^)\s\"`]+)", content)
    if m:
        return m.group(1).upper(), m.group(2)

    # 再兜底：GET /open-apis/xxx
    m2 = re.search(r"\b(GET|POST|PUT|PATCH|DELETE)\b\s+(/[^)\s\"`]+)", content)
    if m2:
        return m2.group(1).upper(), m2.group(2)

    return method, path


def derive_old_meta_project_and_strip_prefix(path: str) -> Tuple[str, str]:
    for meta_project, match_prefix, strip_prefix in OLD_API_PATH_RULES:
        if path.startswith(match_prefix):
            return meta_project, strip_prefix

    # 未覆盖的旧接口：做一个保守推断
    if path.startswith("/open-apis/"):
        parts = [p for p in path.split("/") if p]
        if len(parts) >= 2:
            seg = parts[1]
            return seg, f"/open-apis/{seg}/"
    # 其它情况：用第一段做 project
    parts = [p for p in path.split("/") if p]
    if parts:
        seg = parts[0]
        return seg, f"/{seg}/"
    return "unknown", "/"


@dataclass
class ApiRowDraft:
    api_id: str
    name: str
    meta_project: str
    meta_version: str
    meta_resource: str
    meta_name_base: str
    http_method: str
    http_path: str
    detail: str
    charging_method: str
    support_app_types: List[str]
    update_time: int
    doc_path: str
    full_path: str
    catalog_index: int
    catalog_order_mark: str

    @property
    def biz_tag(self) -> str:
        return PROJECT_BIZTAG_OVERRIDES.get(self.meta_project, self.meta_project)


def build_row_draft(leaf: Dict[str, Any], catalog_index: int, timeout: int, retries: int) -> ApiRowDraft:
    api_id = str(leaf.get("id", "")).strip()
    name = str(leaf.get("name", "")).strip()
    catalog_order_mark = str(leaf.get("orderMark") if leaf.get("orderMark") is not None else "0")
    api_summary = leaf.get("apiSummary") or {}
    doc_full_path = str(api_summary.get("fullPath") or "").strip()
    if not doc_full_path:
        # 理论上不会发生：没有 fullPath 就没法取文档
        raise RuntimeError(f"API {api_id} 缺少 fullPath，无法获取文档详情")

    doc_detail = http_get_doc_detail(doc_full_path, timeout=timeout, retries=retries)
    data = doc_detail.get("data") or {}

    schema = data.get("schema") or {}
    content = data.get("content") or ""

    # 文档更新时间：毫秒 -> 秒（与仓库 CSV 一致）
    update_time_ms = data.get("updateTime")
    update_time = 0
    if isinstance(update_time_ms, (int, float)):
        update_time = int(update_time_ms // 1000)

    redirect_uri = str(data.get("redirectUri") or "").strip()
    doc_path = (DOC_BASE_URL + redirect_uri) if redirect_uri.startswith("/") else ""

    detail = sanitize_singleline_text(extract_description(schema, content))

    # chargingMethod：优先使用 schema.apiSchema
    charging_method = "none"
    api_schema = (schema.get("apiSchema") or {}) if isinstance(schema, dict) else {}
    if isinstance(api_schema, dict):
        charging_strategy = api_schema.get("apiChargingStrategy") or {}
        if isinstance(charging_strategy, dict):
            cm = charging_strategy.get("chargingMethod")
            if isinstance(cm, str) and cm.strip():
                charging_method = cm.strip()

    # 支持的应用类型：优先 apiSummary，其次从 content 的 md-app-support 解析
    support_app_types = normalize_support_app_types(api_summary.get("supportAppTypes") or [])
    if not support_app_types:
        support_app_types = extract_support_app_types_from_content(content)

    # HTTP method/path：优先 schema.apiSchema，其次 apiSummary，再从 content 抽取
    http_method = ""
    http_path = ""
    if isinstance(api_schema, dict):
        hm = api_schema.get("httpMethod")
        p = api_schema.get("path")
        if isinstance(hm, str) and isinstance(p, str) and hm.strip() and p.strip():
            http_method = hm.strip().upper()
            http_path = p.strip()

    if not (http_method and http_path):
        hm2 = api_summary.get("httpMethod")
        p2 = api_summary.get("apiPath")
        if isinstance(hm2, str) and isinstance(p2, str) and hm2.strip() and p2.strip():
            http_method = hm2.strip().upper()
            http_path = p2.strip()

    if not (http_method and http_path):
        http_method, http_path = parse_method_and_path_from_content(content)

    # meta.*：优先 apiSummary.apiIdentity；缺失则按旧接口处理
    meta_project = ""
    meta_version = ""
    meta_resource = ""
    meta_name_base = ""

    identity = api_summary.get("apiIdentity")
    if isinstance(identity, dict):
        meta_project = str(identity.get("project") or "").strip()
        meta_version = str(identity.get("version") or "").strip()
        meta_resource = str(identity.get("resource") or "").strip()
        meta_name_base = str(identity.get("apiName") or "").strip()
    else:
        meta_version = "old"
        meta_resource = "default"
        meta_project, strip_prefix = derive_old_meta_project_and_strip_prefix(http_path)
        # meta.Name 取去掉前缀后的剩余路径（保留尾部斜杠），并去掉起始 '/'
        rest = http_path
        if strip_prefix and rest.startswith(strip_prefix):
            rest = rest[len(strip_prefix) :]
        meta_name_base = rest.lstrip("/")

        # 旧版接口中，绝大多数为 none；仅少量例外（例如 im_message）为 basic
        if meta_project == "im_message":
            charging_method = "basic"

    full_path_csv = "/document" + doc_full_path if not doc_full_path.startswith("/document") else doc_full_path

    return ApiRowDraft(
        api_id=api_id,
        name=name,
        meta_project=meta_project,
        meta_version=meta_version,
        meta_resource=meta_resource,
        meta_name_base=meta_name_base,
        http_method=http_method,
        http_path=http_path,
        detail=detail,
        charging_method=charging_method,
        support_app_types=support_app_types,
        update_time=update_time,
        doc_path=doc_path,
        full_path=full_path_csv,
        catalog_index=catalog_index,
        catalog_order_mark=catalog_order_mark,
    )


def finalize_rows(drafts: List[ApiRowDraft]) -> List[Dict[str, str]]:
    # 处理旧接口中：相同 meta_name_base 但方法不同的情况，用 method# 前缀做区分
    key_to_drafts: Dict[Tuple[str, str, str], List[ApiRowDraft]] = {}
    for d in drafts:
        key = (d.meta_project, d.meta_version, d.meta_name_base)
        key_to_drafts.setdefault(key, []).append(d)

    final: List[Dict[str, str]] = []
    for d in drafts:
        meta_name = d.meta_name_base
        key = (d.meta_project, d.meta_version, d.meta_name_base)
        group = key_to_drafts.get(key) or []
        if d.meta_version == "old" and len(group) > 1 and d.http_method:
            meta_name = f"{d.http_method.lower()}#{d.meta_name_base}"

        support_json = json.dumps(normalize_support_app_types(d.support_app_types), ensure_ascii=False)
        tags_json = "[]"

        # isCharge：与仓库 CSV 一致，none -> false，其它 -> true
        is_charge = "true" if (d.charging_method and d.charging_method != "none") else "false"

        final.append(
            {
                "id": d.api_id,
                "name": d.name,
                "bizTag": d.biz_tag,
                "meta.Project": d.meta_project,
                "meta.Version": d.meta_version,
                "meta.Resource": d.meta_resource,
                "meta.Name": meta_name,
                "detail": d.detail,
                "chargingMethod": d.charging_method or "none",
                "fullDose": "true",
                "fullPath": d.full_path,
                "url": f"{d.http_method}:{d.http_path}" if (d.http_method and d.http_path) else "",
                # 这里保留 catalog 的原始 orderMark；同时脚本默认输出顺序按文档顺序（catalog_index）保证。
                "orderMark": d.catalog_order_mark if d.catalog_order_mark else "0",
                "supportAppTypes": support_json,
                "tags": tags_json,
                "updateTime": str(d.update_time),
                "isCharge": is_charge,
                "meta.Type": "1",
                "docPath": d.doc_path,
            }
        )

    return final


def export_csv(rows: List[Dict[str, str]], output_path: str) -> None:
    with open(output_path, "w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=CSV_HEADER)
        writer.writeheader()
        for row in rows:
            writer.writerow(row)


def main() -> int:
    parser = argparse.ArgumentParser(description="导出飞书服务端 API 列表为 CSV（列结构与 api_list_export.csv 一致）")
    parser.add_argument("--output", default="api_list_export.csv", help="输出 CSV 路径（默认覆盖仓库根目录 api_list_export.csv）")
    parser.add_argument("--max-workers", type=int, default=16, help="并发抓取线程数（默认 16）")
    parser.add_argument("--timeout", type=int, default=30, help="单次 HTTP 请求超时时间（秒）")
    parser.add_argument("--retries", type=int, default=3, help="失败重试次数（默认 3）")
    parser.add_argument("--limit", type=int, default=0, help="仅导出前 N 条（用于调试）")
    parser.add_argument(
        "--sort",
        choices=("doc", "meta"),
        default="doc",
        help="排序方式：doc=按文档默认顺序导出（默认），meta=按 meta.* 字段排序",
    )
    args = parser.parse_args()

    catalog = http_get_json(CATALOG_URL, timeout=args.timeout, retries=args.retries)
    items = (catalog.get("data") or {}).get("items") or []
    leaves = list(iter_api_leaves(items))

    if args.limit and args.limit > 0:
        leaves = leaves[: args.limit]

    total = len(leaves)
    print(f"📥 获取到 API 条目: {total}")

    drafts_by_index: Dict[int, ApiRowDraft] = {}
    failed: List[Tuple[str, str]] = []

    with ThreadPoolExecutor(max_workers=max(1, args.max_workers)) as pool:
        future_map = {
            pool.submit(build_row_draft, leaf, idx, args.timeout, args.retries): idx for idx, leaf in enumerate(leaves)
        }

        done = 0
        last_print = time.time()
        for fut in as_completed(future_map):
            idx = future_map[fut]
            leaf = leaves[idx]
            api_id = str(leaf.get("id", ""))
            try:
                drafts_by_index[idx] = fut.result()
            except Exception as e:  # noqa: BLE001
                failed.append((api_id, str(e)))
            done += 1
            now = time.time()
            if now - last_print >= 1:
                print(f"⏳ 进度: {done}/{total}（失败 {len(failed)}）")
                last_print = now

    if failed:
        print(f"⚠️  有 {len(failed)} 条抓取失败，将继续导出成功部分。")
        for api_id, err in failed[:10]:
            print(f"  - {api_id}: {err}")

    drafts_in_order = [drafts_by_index[i] for i in range(total) if i in drafts_by_index]
    rows = finalize_rows(drafts_in_order)

    if args.sort == "meta":
        # 保持输出稳定：按 bizTag/meta.Project/meta.Version/meta.Resource/meta.Name 排序
        rows.sort(
            key=lambda r: (
                r.get("bizTag", ""),
                r.get("meta.Project", ""),
                r.get("meta.Version", ""),
                r.get("meta.Resource", ""),
                r.get("meta.Name", ""),
            )
        )

    export_csv(rows, args.output)
    print(f"✅ 导出完成：{args.output}（{len(rows)} 条）")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
