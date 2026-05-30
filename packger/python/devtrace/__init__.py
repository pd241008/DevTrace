import os
import subprocess
import signal
import atexit

from .installer import ensure_binary

class DevTrace:
    def __init__(self, env=None):
        """
        Initialize the DevTrace Engine wrapper
        :param env: Optional dictionary of environment variables to pass to the Rust proxy
        """
        self.env = env or {}
        self.process = None

    def start(self):
        """
        Start the DevTrace proxy server
        """
        if self.process:
            print("[DevTrace] Proxy is already running.")
            return

        bin_path = ensure_binary()
        
        print("[DevTrace] Starting proxy server...")
        
        env_vars = os.environ.copy()
        env_vars.update(self.env)
        
        self.process = subprocess.Popen(
            [bin_path, "serve"],
            env=env_vars,
            stdout=sys.stdout if 'sys' in globals() else None,
            stderr=sys.stderr if 'sys' in globals() else None
        )
        
        # Ensure cleanup on normal exit
        atexit.register(self.stop)
        
        # We don't hijack sigint completely, but we can rely on atexit for cleanup in Python
        
    def stop(self):
        """
        Stop the running DevTrace proxy server
        """
        if self.process and self.process.poll() is None:
            print("[DevTrace] Stopping proxy...")
            self.process.terminate()
            try:
                self.process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.process.kill()
            self.process = None

    def replay(self, request_id):
        """
        Fire a replay for a specific request ID directly
        """
        bin_path = ensure_binary()
        print(f"[DevTrace] Replaying request ID: {request_id}")
        
        env_vars = os.environ.copy()
        env_vars.update(self.env)
        
        subprocess.run(
            [bin_path, "replay", str(request_id)],
            env=env_vars,
            check=True
        )
