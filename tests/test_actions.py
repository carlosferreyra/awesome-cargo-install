"""Exercise Actions summaries and the workflow's exact publishing shell block."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


class SummaryTests(unittest.TestCase):
    def summarize(self, kind, files, env):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name, value in files.items():
                (root / name).write_text(value)
            summary = root / 'summary.md'
            result = subprocess.run([sys.executable, str(ROOT / 'scripts/actions_summary.py'), kind],
                cwd=root, env=dict(os.environ, GITHUB_STEP_SUMMARY=str(summary), **env),
                capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            return summary.read_text()

    def test_invalid_catalog_still_reports_failure(self):
        text = self.summarize('catalog', {'tools.json': '{'},
                              {'JOB_STATUS': 'failure', 'CATALOG_STATUS': 'failure'})
        self.assertIn('counts unavailable', text)
        self.assertIn('| Catalog | failure |', text)
        self.assertIn('| README freshness | not run |', text)

    def test_install_counts_and_failures(self):
        text = self.summarize('installs', {
            'install.log': 'Results: 4 passed, 1 failed\n',
            'output.log': json.dumps([{'package': 'fixture', 'reason': 'wrong_binary'}])},
            {'JOB_STATUS': 'failure'})
        self.assertIn('**4 passed**, **1 failed**', text)
        self.assertIn('`fixture`: wrong_binary', text)

    def test_sync_no_change(self):
        text = self.summarize('sync', {},
                              {'JOB_STATUS': 'success', 'PUBLISHED_CHANGES': 'false'})
        self.assertIn('No changes to publish.', text)


class PublishingTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        root = Path(self.directory.name)
        self.remote = root / 'remote.git'
        self.repo = root / 'checkout'
        self.repo.mkdir()
        self.output = root / 'output'
        self.git('init', '--bare', str(self.remote), cwd=root)
        self.git('init', '-b', 'main')
        self.git('config', 'user.name', 'Test')
        self.git('config', 'user.email', 'test@example.com')
        for name in ['tools.json', 'README.md', 'unrelated.txt']:
            (self.repo / name).write_text('baseline\n')
        self.git('add', '.')
        self.git('commit', '-m', 'baseline')
        self.git('remote', 'add', 'origin', str(self.remote))
        self.git('push', '-u', 'origin', 'main')
        self.baseline = self.git('rev-parse', 'HEAD').stdout.strip()
        source = (ROOT / '.github/workflows/sync_releases.yml').read_text()
        step = source.split('      - name: Commit and push metadata changes\n', 1)[1]
        block = step.split('        run: |\n', 1)[1].split('\n      - name:', 1)[0]
        self.script = '\n'.join(line[10:] for line in block.splitlines())

    def git(self, *args, cwd=None):
        result = subprocess.run(['git', *args], cwd=cwd or self.repo,
                                capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        return result

    def publish(self):
        return subprocess.run(['bash', '-eo', 'pipefail', '-c', self.script], cwd=self.repo,
            env=dict(os.environ, GITHUB_OUTPUT=str(self.output)), capture_output=True, text=True)

    def test_no_change_creates_no_commit(self):
        result = self.publish()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.git('rev-parse', 'HEAD').stdout.strip(), self.baseline)
        self.assertEqual(self.output.read_text(), 'changed=false\n')

    def test_only_generated_files_are_published(self):
        for name in ['tools.json', 'README.md', 'unrelated.txt']:
            (self.repo / name).write_text('changed\n')
        result = self.publish()
        self.assertEqual(result.returncode, 0, result.stderr)
        files = self.git('diff-tree', '--no-commit-id', '--name-only', '-r', 'HEAD').stdout.splitlines()
        self.assertEqual(set(files), {'tools.json', 'README.md'})
        self.assertEqual(self.git('status', '--porcelain').stdout.strip(), 'M unrelated.txt')
        self.assertEqual(self.output.read_text(), 'changed=true\n')
        remote_head = self.git('--git-dir', str(self.remote), 'rev-parse', 'main').stdout
        self.assertEqual(remote_head, self.git('rev-parse', 'HEAD').stdout)

    def test_concurrent_main_update_is_not_overwritten(self):
        other = Path(self.directory.name) / 'other'
        self.git('clone', '-b', 'main', str(self.remote), str(other))
        self.git('config', 'user.name', 'Other', cwd=other)
        self.git('config', 'user.email', 'other@example.com', cwd=other)
        (other / 'unrelated.txt').write_text('concurrent change\n')
        self.git('commit', '-am', 'concurrent change', cwd=other)
        self.git('push', 'origin', 'main', cwd=other)
        expected = self.git('rev-parse', 'HEAD', cwd=other).stdout
        (self.repo / 'tools.json').write_text('metadata change\n')
        result = self.publish()
        self.assertNotEqual(result.returncode, 0)
        actual = self.git('--git-dir', str(self.remote), 'rev-parse', 'main').stdout
        self.assertEqual(actual, expected)
        self.assertNotIn('changed=true', self.output.read_text() if self.output.exists() else '')


if __name__ == '__main__':
    unittest.main()
