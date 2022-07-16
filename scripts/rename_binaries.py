from pathlib import Path
import subprocess
import re

target_triple = re.search(
    r"host: (\S+)",
    subprocess.run(["rustc", "-vV"], capture_output=True, text=True).stdout,
).group(1)
assert target_triple
for file in list(Path("src/learnatvcs/dist").iterdir()) + list(
    Path("src/powerschool/dist").iterdir()
):
    file.replace(file.with_name(f"{file.stem}-{target_triple}{file.suffix}"))
