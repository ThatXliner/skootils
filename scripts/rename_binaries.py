#!/usr/bin/env python
import platform
import re
import subprocess
from pathlib import Path

extension = ".exe" if platform.system() == "Windows" else ""

target_triple = re.search(
    r"host: (\S+)",
    # skipcq
    subprocess.run(["rustc", "-vV"], capture_output=True, text=True).stdout,
).group(1)
assert target_triple  # skipcq
for file in list(Path("src/learnatvcs/dist").iterdir()) + list(
    Path("src/powerschool/dist").iterdir()
):
    file.replace(file.with_name(f"{file.stem}-{target_triple}{extension}"))
