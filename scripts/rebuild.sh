#!/bin/sh
# This script is mainly for developers.
# It only works on a UNIX-like environment
rm -rf src/learnatvcs/dist src/powerschool/dist
if [ $1 = "--full" ]
then
    rm -rf src/learnatvcs/build src/powerschool/build
fi
cd src/learnatvcs
echo *.spec | xargs -n 1 pyinstaller &
cd ../powerschool
echo *.spec | xargs -n 1 pyinstaller &
wait

cd ../..  # Project root
python scripts/verify_binaries.py
python scripts/rename_binaries.py
