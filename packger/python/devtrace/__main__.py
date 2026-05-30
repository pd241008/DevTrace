import sys
import os
import subprocess

from .installer import ensure_binary

def main():
    try:
        bin_path = ensure_binary()
    except Exception as e:
        print(f"[DevTrace CLI] Error: {e}", file=sys.stderr)
        sys.exit(1)

    args = sys.argv[1:]
    if not args:
        args = ["serve"]
        
    print(f"[DevTrace CLI] Running: devtrace {' '.join(args)}")
    
    try:
        # Replace the current process with the Rust binary
        # This is more efficient and handles signals naturally
        if os.name == 'posix':
            os.execv(bin_path, [bin_path] + args)
        else:
            # Fallback for Windows
            sys.exit(subprocess.call([bin_path] + args))
    except Exception as e:
        print(f"[DevTrace CLI] Failed to execute binary: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    main()
