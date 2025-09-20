#!/usr/bin/env python3
import argparse
import json
import os
import sys
import urllib.parse

import requests


def upsert_label(session, repo, label, dry_run=False):
    """Create or update a label to match the desired definition."""
    name = label["name"]
    base = f"https://api.github.com/repos/{repo}/labels"
    url_get = f"{base}/{urllib.parse.quote(name, safe='')}"
    response = session.get(url_get)

    payload = {
        "name": name,
        "color": label.get("color", "ededed").lstrip("#"),
        "description": label.get("description", "")
    }

    if response.status_code == 200:
        if dry_run:
            print(f"DRY-RUN: would UPDATE {name}")
            return True
        update = session.patch(url_get, json=payload)
        ok = update.status_code in (200, 201)
        print(("✅ Updated" if ok else "⚠️ Update failed"), name, update.status_code)
        if not ok:
            print(update.text)
        return ok
    if response.status_code == 404:
        if dry_run:
            print(f"DRY-RUN: would CREATE {name}")
            return True
        create = session.post(base, json=payload)
        ok = create.status_code in (200, 201)
        print(("✅ Created" if ok else "⚠️ Create failed"), name, create.status_code)
        if not ok:
            print(create.text)
        return ok

    print("⚠️ GET failed", name, response.status_code, response.text)
    return False


def delete_missing(session, repo, labels_json_names, dry_run=False):
    """Delete labels present in GitHub but absent from labels.json."""
    base = f"https://api.github.com/repos/{repo}/labels"
    ok = True
    page = 1
    existing = []

    while True:
        response = session.get(base, params={"per_page": 100, "page": page})
        if response.status_code != 200:
            print("⚠️ Failed to fetch existing labels", response.status_code, response.text)
            return False
        items = response.json()
        if not items:
            break
        existing.extend(items)
        page += 1

    to_delete = [label["name"] for label in existing if label["name"] not in labels_json_names]

    for name in to_delete:
        url = f"{base}/{urllib.parse.quote(name, safe='')}"
        if dry_run:
            print(f"DRY-RUN: would DELETE {name}")
            continue
        response = session.delete(url)
        ok = ok and response.status_code == 204
        print(("🗑️ Deleted" if response.status_code == 204 else "⚠️ Delete failed"), name, response.status_code)
        if response.status_code != 204:
            print(response.text)

    return ok


def main():
    parser = argparse.ArgumentParser(description="Synchronise GitHub labels with labels.json")
    parser.add_argument("--repo", default=os.getenv("GITHUB_REPO", "luna-assistant-ai/luna-assistant-ai-core"))
    parser.add_argument("--labels", default="labels.json")
    parser.add_argument("--dry-run", type=int, default=0, help="Print actions without modifying labels")
    parser.add_argument("--delete-missing", type=int, default=0, help="Delete labels missing from labels.json")
    args = parser.parse_args()

    token = os.getenv("GITHUB_TOKEN")
    if not token:
        print("Missing GITHUB_TOKEN env var", file=sys.stderr)
        sys.exit(2)

    try:
        with open(args.labels, "r", encoding="utf-8") as handle:
            labels = json.load(handle)
    except FileNotFoundError:
        print(f"Could not find labels file: {args.labels}", file=sys.stderr)
        sys.exit(2)

    session = requests.Session()
    session.headers.update({
        "Authorization": f"Bearer {token}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
        "User-Agent": "luna-label-sync"
    })

    ok = True
    names = []
    for label in labels:
        names.append(label["name"])
        ok = upsert_label(session, args.repo, label, dry_run=bool(args.dry_run)) and ok

    if args.delete_missing:
        ok = delete_missing(session, args.repo, set(names), dry_run=bool(args.dry_run)) and ok

    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
