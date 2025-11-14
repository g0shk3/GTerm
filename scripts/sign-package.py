#!/usr/bin/env python3
import pexpect
import sys

try:
    # Spawn the signing command
    import os
    project_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    tar_path = os.path.join(project_dir, 'src-tauri/target/release/bundle/macos/GTerm.app.tar.gz')

    child = pexpect.spawn(
        f'tauri signer sign {tar_path} --private-key-path ~/.tauri/gterm.key',
        timeout=30
    )

    # Wait for password prompt and send empty password
    try:
        child.expect('Password:', timeout=5)
        child.sendline('')
    except:
        pass  # Maybe no password prompt

    # Wait for completion
    child.expect(pexpect.EOF, timeout=10)
    print(child.before.decode('utf-8'))

    child.close()

    if child.exitstatus == 0:
        print("\n✓ Package signed successfully!")
        sys.exit(0)
    else:
        print(f"\n✗ Signing failed with exit code {child.exitstatus}")
        sys.exit(1)

except Exception as e:
    print(f"Error: {e}")
    sys.exit(1)
