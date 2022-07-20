#!/bin/sh
# This script is mainly for developers.
# It only works on a UNIX-like environment
rm -rf src/learnatvcs/dist src/powerschool/dist

cd src/learnatvcs
echo *.spec | xargs -n 1 pyinstaller &
cd ../powerschool
echo *.spec | xargs -n 1 pyinstaller &
wait

cd ../..  # Project root
python scripts/verify_binaries.py
python scripts/rename_binaries.py
