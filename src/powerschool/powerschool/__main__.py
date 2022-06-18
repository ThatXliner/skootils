import json

from powerschool import PowerSchool
import keyring

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
        print(json.dumps(bot.get_quarters()))
        print(json.dumps(bot.get(json.loads(input()))))


if __name__ == "__main__":
    main()
