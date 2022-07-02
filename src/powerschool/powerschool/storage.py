import json
import random
import sys
import time
from collections import defaultdict, deque
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


def main():
    # TODO: All vs class
    quarter = sys.argv[1]
    labels = []
    colors = deque(
        [
            "#802392",
            "#DE4D86",
            "#7C90DB",
            "#84E6F8",
            "#4d7976",
            "#A5F8D3",
            "#489b51",
            "#e9b04d",
            "#df4949",
        ]
    )

    def pickColor():
        colors.rotate()
        return colors[0]

    classes = defaultdict(lambda: (list(), pickColor()))
    # TODO: multithreading for maximum efficiency
    for i, file in enumerate(sorted((HISTORY_DIR / quarter).iterdir()), start=1):
        labels.append(f"Scrape #{i}")
        scrape = json.loads(file.read_text())
        for period in scrape:
            try:
                classes[scrape[period]["class_name"]][0].append(
                    int(scrape[period]["quarter_info"]["overall_grade"]["percent"])
                )
            except ValueError:
                pass
    print(
        json.dumps(
            {
                "labels": labels,
                "datasets": [
                    {
                        "label": k,
                        "data": v[0],
                        "fill": False,
                        "backgroundColor": v[1],
                        "pointBackgroundColor": v[1],
                        "tension": 0,  # 0 = straight line. this is for bezier curve
                    }
                    for k, v in classes.items()
                    if v[0]
                ],
            }
        )
    )


if __name__ == "__main__":
    main()
