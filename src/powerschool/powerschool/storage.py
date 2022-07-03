import json
import time

from pathlib import Path
from typing import Dict, List, Optional

import keyring
from platformdirs import user_data_dir

APP_NAME = "Skootils"
APP_AUTHOR = "Bryan Hu"
HISTORY_DIR = Path(user_data_dir(APP_NAME, APP_AUTHOR)) / "powerschool" / "history"
if not HISTORY_DIR.is_dir():
    HISTORY_DIR.mkdir(parents=True)


def get_auth() -> Optional[Dict[str, str]]:
    return json.loads(keyring.get_password("skootils", "powerschool") or "null")


def set_auth(auth: Dict[str, str]) -> bool:
    try:
        keyring.set_password("skootils", "powerschool", json.dumps(auth))
    except keyring.errors.PasswordSetError:
        return False
    return True


def save(output, quarters: List[str]) -> None:
    filename = f"{time.time()}.json"
    for quarter in output:
        store = HISTORY_DIR / (
            quarters[-1] if quarter.startswith("Latest") else quarter
        )
        (store).mkdir(exist_ok=True)

        (store / filename).touch()
        (store / filename).write_text(json.dumps(output[quarter]))
