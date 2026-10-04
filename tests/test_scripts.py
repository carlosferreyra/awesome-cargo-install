"""Regression checks for isolated installs and partial release-sync failures."""
import http.server
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import threading
import unittest

ROOT = Path(__file__).resolve().parents[1]
CARGO = shutil.which('cargo')


class InstallValidationTests(unittest.TestCase):
    def run_validator(self, mode):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fake = root / 'cargo'
            fake.write_text('''#!/usr/bin/env python3
import os, pathlib, sys
args = sys.argv[1:]
root = pathlib.Path(args[args.index('--root') + 1])
mode = os.environ['TEST_INSTALL_MODE']
if mode == 'fallback' and args[0] == 'binstall':
    print('network timeout', file=sys.stderr)
    sys.exit(1)
if mode == 'failure':
    print('underlying compiler failure', file=sys.stderr)
    sys.exit(1)
(root / 'bin').mkdir()
for name in (['first', 'second'] if mode in ['all', 'fallback'] else ['first']):
    binary = root / 'bin' / name
    binary.write_text('#!/bin/sh\\nexit 0\\n')
    binary.chmod(0o755)
''')
            fake.chmod(0o755)
            # The missing secondary executable exists on PATH, but must not count.
            stale = root / 'second'
            stale.write_text('#!/bin/sh\nexit 0\n')
            stale.chmod(0o755)
            env = dict(os.environ, PATH=str(root) + os.pathsep + os.environ['PATH'],
                       TEST_INSTALL_MODE=mode)
            report = root / 'report.json'
            result = subprocess.run(
                [CARGO, '+nightly', '-Zscript', str(ROOT / 'scripts/test_clients.rs'), '--',
                 '--tools', json.dumps([{'package': 'fixture', 'execs': ['first', 'second']}]),
                 '--output', str(report)], cwd=ROOT, env=env, capture_output=True, text=True)
            return result, json.loads(report.read_text()) if report.exists() else []

    def test_missing_secondary_binary_ignores_path(self):
        result, report = self.run_validator('missing')
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertEqual(report[0]['reason'], 'wrong_binary')
        self.assertIn('Missing executables: second', report[0]['details'])

    def test_all_binaries_present(self):
        result, _ = self.run_validator('all')
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_source_fallback(self):
        result, _ = self.run_validator('fallback')
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_failure_retains_installer_logs(self):
        result, report = self.run_validator('failure')
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertIn('underlying compiler failure', report[0]['details'])


class ReleaseSyncTests(unittest.TestCase):
    def test_partial_sync_reports_failure_and_preserves_missing_metadata(self):
        class Handler(http.server.BaseHTTPRequestHandler):
            def do_GET(self):
                if self.path.endswith('/missing'):
                    self.send_error(404)
                    return
                self.send_response(200)
                self.end_headers()
                self.wfile.write(json.dumps({'crate': {'max_stable_version': '2.0.0'},
                    'versions': [{'num': '2.0.0', 'created_at': '2026-10-04T00:00:00Z'}]}).encode())

            def log_message(self, *args):
                pass

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'scripts').mkdir()
            server = http.server.ThreadingHTTPServer(('127.0.0.1', 0), Handler)
            thread = threading.Thread(target=server.serve_forever, daemon=True)
            thread.start()
            try:
                source = (ROOT / 'scripts/latest_release.rs').read_text().replace(
                    'https://crates.io/api/v1/crates/{crate}',
                    f'http://127.0.0.1:{server.server_port}/crates/{{crate}}')
                script = root / 'scripts/latest_release.rs'
                script.write_text(source)
                catalog = {'categories': [{'tools': {
                    'ok': {'version': '1.0.0'},
                    'missing': {'version': '1.0.0', 'last_release': '2025-01-01'}}}]}
                (root / 'tools.json').write_text(json.dumps(catalog))
                result = subprocess.run([CARGO, '+nightly', '-Zscript', str(script)],
                                        cwd=root, capture_output=True, text=True)
                self.assertEqual(result.returncode, 1, result.stderr)
                tools = json.loads((root / 'tools.json').read_text())['categories'][0]['tools']
                self.assertEqual(tools['ok']['version'], '2.0.0')
                self.assertEqual(tools['missing'], catalog['categories'][0]['tools']['missing'])
                self.assertIn('sync incomplete: missing', result.stderr)
            finally:
                server.shutdown()
                server.server_close()
                thread.join()


if __name__ == '__main__':
    unittest.main()
