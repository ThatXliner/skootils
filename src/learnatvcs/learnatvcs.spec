# -*- mode: python ; coding: utf-8 -*-
from pathlib import Path
import platform

block_cipher = None


a = Analysis(
    ['learnatvcs/__main__.py'],
    pathex=Path('.venv/lib').glob("python*/site-packages"),
    binaries=[],
    datas=[],
    hiddenimports=[],
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    win_no_prefer_redirects=False,
    win_private_assemblies=False,
    cipher=block_cipher,
    noarchive=False,
)
pyz = PYZ(a.pure, a.zipped_data, cipher=block_cipher)

exe = EXE(
    pyz,
    a.scripts,
    # Python runtime options
    [('u', None, 'OPTION'), ('W ignore', None, 'OPTION')],
    a.binaries,
    a.zipfiles,
    a.datas,
    [],
    name='learnatvcs',
    debug=False,
    bootloader_ignore_signals=False,
    strip=platform.system() != 'Windows',
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=True,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
)
