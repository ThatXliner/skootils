import subprocess
from pathlib import Path

for file in list(Path("src/learnatvcs/dist").iterdir()) + list(
    Path("src/powerschool/dist").iterdir()
):
    if not (
        subprocess.run(
            [str(file), "--test"], check=True, capture_output=True, text=True
        ).stdout.strip()
        == "Program is valid"
    ):
        print(f"{file} is not working")
        print("reason: invalid output")
        exit(1)

print("All programs are working")
