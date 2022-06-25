import json
import random
import sys
from collections import defaultdict
from pathlib import Path
import time
from typing import Dict, Optional, List

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
        store = HISTORY_DIR / (quarters[-1] if quarter == "Latest" else quarter)
        (store).mkdir(exist_ok=True)

        (store / filename).touch()
        (store / filename).write_text(json.dumps(output[quarter]))


def main():
    quarter = sys.argv[1]
    labels = []

    classes = defaultdict(list)
    for file in sorted((HISTORY_DIR / quarter).iterdir()):
        labels.append(file.stem)
        scrape = json.loads(file.read_text())
        for period in scrape:
            classes[scrape[period]["class_name"]].append(
                float(scrape[period]["quarter_info"]["percent"])
            )
    print(
        json.dumps(
            {
                "labels": labels,
                "datasets": [
                    {
                        "label": k,
                        "data": v,
                        "fill": False,
                        "borderColor": f"rgb({tuple(random.choices(range(256), k=3))})",
                        "tension": 0.1,
                    }
                    for k, v in classes.items()
                ],
            }
        )
    )


if __name__ == "__main__":
    main()