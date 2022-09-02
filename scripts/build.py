#!/usr/bin/env python
import glob
import platform
import re
import shutil
import subprocess
import sys
import time
from os.path import getmtime
from pathlib import Path
from typing import NamedTuple

EXTENSION = ".exe" if platform.system() == "Windows" else ""

TARGET_TRIPLE = re.search(
    r"host: (\S+)",
    # skipcq
    subprocess.run(["rustc", "-vV"], capture_output=True, text=True).stdout,
).group(1)


def get_new_name(binary: Path) -> Path:
    return binary.with_name(f"{binary.stem}-{TARGET_TRIPLE}{EXTENSION}")


def rename(binary: Path):
    binary.replace(get_new_name(binary))


class Job(NamedTuple):
    proc: subprocess.Popen
    output: Path


def start_build(spec: Path) -> subprocess.Popen:
    return subprocess.Popen(
        ["pyinstaller", str(spec.name)],
        cwd=str(spec.parent),
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )


specfiles = map(
    Path, glob.glob("src/learnatvcs/*.spec") + glob.glob("src/powerschool/*.spec")
)
targets = {
    prereq.parent / "dist" / prereq.stem: [prereq]
    + list((prereq.parent / prereq.parent.stem).glob("**/*.py"))
    for prereq in specfiles
}
jobs = set()
for target, prerequisites in targets.items():
    expected_output = get_new_name(target)
    if not expected_output.exists() or any(
        map(
            # "Cell variable defined in loop"
            # shouldn't be a problem because
            # we're using the `any` function
            # to use it immediantly after
            lambda mod_time: mod_time >= getmtime(expected_output),
            map(getmtime, prerequisites),
        )
    ):
        if (
            len(sys.argv) == 2
            and sys.argv[1] == "--full"
            and (target.parent / "build").exists()
        ):
            shutil.rmdir(target.parent / "build")
        jobs.add(Job(start_build(prerequisites[0]), target))
        print("Sent", prerequisites[0], "to build worker")
if not jobs:
    print("Nothing needs to be built")
    sys.exit(0)
print(f"Building {len(jobs)} binaries")
start = time.time()
while jobs:
    for job in list(jobs):
        proc = job.proc
        if proc.poll() == 0:
            rename(job.output.with_suffix(EXTENSION))
            print("Finished building", proc.args[1])
            jobs.remove(job)
    time.sleep(0.5)
print(f"Finished in {time.time() - start:0.2f} seconds (binaries unverified)")
