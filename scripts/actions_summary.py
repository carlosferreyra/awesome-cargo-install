"""Write a compact Actions summary from step outcomes and existing reports."""
import json
import os
from pathlib import Path
import re
import sys

kind = sys.argv[1]
checks = {
    'catalog': [('Catalog', 'CATALOG_STATUS'), ('Regression tests', 'TESTS_STATUS'),
                ('README freshness', 'README_STATUS')],
    'installs': [('Catalog', 'CATALOG_STATUS'), ('Install validation', 'INSTALLS_STATUS')],
    'sync': [('Catalog before sync', 'BEFORE_STATUS'), ('Registry fetch', 'METADATA_STATUS'),
             ('Catalog after sync', 'AFTER_STATUS'), ('README', 'README_STATUS'),
             ('Publish', 'PUBLISH_STATUS')],
}[kind]
lines = [f'## {kind.capitalize()}: {os.environ.get("JOB_STATUS", "unknown")}', '']
catalog = Path('tools.json')
if catalog.exists():
    try:
        data = json.loads(catalog.read_text())
        categories = data['categories']
        count = sum(len(category['tools']) for category in categories)
        lines += [f'Catalog: **{count} tools**, **{len(categories)} categories**.', '']
    except (ValueError, KeyError, TypeError):
        lines += ['Catalog counts unavailable; see the validation log.', '']
lines += ['| Check | Result |', '|:--|:--|']
lines += [f'| {label} | {os.environ.get(key) or "not run"} |' for label, key in checks]
if kind == 'installs' and Path('install.log').exists():
    log = Path('install.log').read_text()
    result = re.search(r'Results: (\d+) passed, (\d+) failed', log)
    lines += ['', f'Installs: **{result[1]} passed**, **{result[2]} failed**.' if result
              else 'Install validation did not produce a final count; see the job log.']
    if Path('output.log').exists():
        failures = json.loads(Path('output.log').read_text())
        lines += ['', 'Failures:']
        lines += [f'- `{item["package"]}`: {item["reason"]}' for item in failures]
if kind == 'sync':
    changed = os.environ.get('PUBLISHED_CHANGES')
    lines += ['', {'true': 'Published updated metadata and README.',
                  'false': 'No changes to publish.'}.get(changed, 'No update was published.')]
if os.environ.get('JOB_STATUS') == 'failure':
    lines += ['', 'See the job logs and failure artifact for details.']
with Path(os.environ['GITHUB_STEP_SUMMARY']).open('a') as summary:
    summary.write('\n'.join(lines) + '\n')
