import json

from powerschool import PowerSchool
import keyring
import time
from pathlib import Path
from platformdirs import user_data_dir

APP_NAME = "Skootils"
APP_AUTHOR = "Bryan Hu"
HISTORY_DIR = Path(user_data_dir(APP_NAME, APP_AUTHOR)) / "powerschool" / "history"
if not HISTORY_DIR.is_dir():
    HISTORY_DIR.mkdir(parents=True)

# Discuss: maybe put login details at argv?
def main() -> None:
    cred = json.loads(keyring.get_password("skootils", "powerschool") or "null")
    HAS_CREDENTIALS = cred is not None
    print(json.dumps(HAS_CREDENTIALS))

    auth = cred or json.loads(input())  # TODO: Remember?
    if cred is None:
        try:
            keyring.set_password("skootils", "powerschool", json.dumps(auth))
        except keyring.errors.PasswordSetError:
            pass

    with PowerSchool(
        "https://powerschool.vcs.net/public", auth["username"], auth["password"]
    ) as bot:
        quarters = bot.get_quarters()
        print(json.dumps(quarters))
        output = json.dumps(bot.get(json.loads(input())))
        print(output)
        filename = f"{time.time()}.json"
        for quarter in output:
            key = quarters[-1] if quarter == "Latest" else quarter
            (HISTORY_DIR / key).mkdir(exist_ok=True)

            (HISTORY_DIR / key / filename).touch()
            (HISTORY_DIR / key / filename).write_text(output[quarter])
            (HISTORY_DIR / key / "info.json").write_text(json.dumps({"last": filename}))


if __name__ == "__main__":
    main()
