import subprocess
from pathlib import Path
import sys

for file in list(Path("src/learnatvcs/dist").iterdir()) + list(
    Path("src/powerschool/dist").iterdir()
):
    proc = subprocess.run(
        [str(file), "--test"], check=False, capture_output=True, text=True
    )
    if proc.returncode != 0:
        print(f"{file} is not valid")
        print("reason: errored during verification")
        print("\nStdout:\n")
        print(proc.stdout)
        print("\nStderr:\n")
        print(proc.stderr)
        sys.exit(1)
    if not proc.stdout.strip() == "Program is valid":
        print(f"{file} is not valid")
        print("reason: invalid output")
        sys.exit(1)

print("All programs are working")
