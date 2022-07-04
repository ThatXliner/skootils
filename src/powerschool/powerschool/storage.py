import json
import time
from pathlib import Path
from typing import Dict, List, Optional

import keyring
from platformdirs import user_data_dir

APP_NAME = "Skootils"
APP_AUTHOR = "Bryan Hu"
ROOT_DIR = Path(user_data_dir(APP_NAME, APP_AUTHOR)) / "powerschool"
HISTORY_DIR = ROOT_DIR / "history"


def get_auth() -> Optional[Dict[str, str]]:
    return json.loads(keyring.get_password("skootils", "powerschool") or "null")


def set_auth(auth: Dict[str, str]) -> bool:
    try:
        keyring.set_password("skootils", "powerschool", json.dumps(auth))
    except keyring.errors.PasswordSetError:
        return False
    return True


def save_teachers(output) -> None:
    store = ROOT_DIR / ("teachers.json")
    if not store.is_file():
        store.touch(exist_ok=True)
        store.write_text("{}")
    teachers = json.loads(store.read_text())
    for quarter in output:
        for period in output[quarter]:
            info = output[quarter][period]
            teachers[info["name"]] = {
                "email": info["email"],
                "period": period,
                "class_name": info["class_name"],
            }
    store.write_text(json.dumps(teachers))


def save(output, quarters: List[str]) -> None:
    HISTORY_DIR.mkdir(parents=True, exist_ok=True)
    filename = f"{time.time()}.json"
    for quarter in output:
        store = HISTORY_DIR / (
            quarters[-1] if quarter.startswith("Latest") else quarter
        )
        (store).mkdir(exist_ok=True)

        (store / filename).touch()
        (store / filename).write_text(json.dumps(output[quarter]))
