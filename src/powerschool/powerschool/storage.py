import json
import random
import sys
import time
from collections import defaultdict
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
        store = HISTORY_DIR / (quarters[-1] if quarter == "Latest" else quarter)
        (store).mkdir(exist_ok=True)

        (store / filename).touch()
        (store / filename).write_text(json.dumps(output[quarter]))


def main():
    # TODO: All vs class
    quarter = sys.argv[1]
    labels = []

    classes = defaultdict(list)
    for i, file in enumerate(sorted((HISTORY_DIR / quarter).iterdir()), start=1):
        labels.append(f"Scrape #{i}")
        scrape = json.loads(file.read_text())
        for period in scrape:
            try:
                classes[scrape[period]["class_name"]].append(
                    float(scrape[period]["quarter_info"]["overall_grade"]["percent"])
                )
            except ValueError:
                pass
    tension = 1 / len(classes)
    print(
        json.dumps(
            {
                "labels": labels,
                "datasets": [
                    {
                        "label": k,
                        "data": v,
                        "fill": False,
                        "borderColor": f"rgb{tuple(random.choices(range(256), k=3))}",
                        "tension": tension,
                    }
                    for k, v in classes.items()
                    if v
                ],
            }
        )
    )


if __name__ == "__main__":
    main()
