import os
import platform
import urllib.request
import sys
import stat

VERSION = "v0.1.1"
REPO = "pd241008/DevTrace"

def get_binary_name():
    os_name = platform.system().lower()
    if os_name == "darwin":
        os_name = "macos"
    
    arch_name = platform.machine().lower()
    if arch_name in ["x86_64", "amd64"]:
        arch_name = "x64"
    elif arch_name in ["arm64", "aarch64"]:
        arch_name = "arm64"
        
    ext = ".exe" if os_name == "windows" else ""
    return f"devtrace-{os_name}-{arch_name}{ext}"

def get_download_url(binary_name):
    custom_url = os.environ.get("DEVTRACE_BINARY_URL")
    if custom_url:
        return f"{custom_url}/{binary_name}"
    return f"https://github.com/{REPO}/releases/download/{VERSION}/{binary_name}"

def ensure_binary():
    home = os.path.expanduser("~")
    bin_dir = os.path.join(home, ".devtrace", "bin")
    os.makedirs(bin_dir, exist_ok=True)
    
    binary_name = "devtrace.exe" if platform.system().lower() == "windows" else "devtrace"
    bin_path = os.path.join(bin_dir, binary_name)
    
    if os.path.exists(bin_path):
        return bin_path
        
    remote_binary_name = get_binary_name()
    url = get_download_url(remote_binary_name)
    
    print(f"[DevTrace] Downloading binary for {platform.system()} {platform.machine()} from {url}...")
    
    try:
        with urllib.request.urlopen(url) as response, open(bin_path, 'wb') as out_file:
            data = response.read()
            out_file.write(data)
    except Exception as e:
        print(f"[DevTrace] Failed to download binary: {e}")
        print("[DevTrace] Ensure you have an internet connection and the release exists.")
        sys.exit(1)
        
    if platform.system().lower() != "windows":
        st = os.stat(bin_path)
        os.chmod(bin_path, st.st_mode | stat.S_IEXEC)
        
    print(f"[DevTrace] Successfully installed to {bin_path}")
    return bin_path
