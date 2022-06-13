import json
import sys

from powerschool import PowerSchool


def main() -> None:
    HAS_CREDENTIALS = False
    print(json.dumps(HAS_CREDENTIALS))
    auth = json.loads(input())

    site = PowerSchool(
        "https://powerschool.vcs.net/public", auth["username"], auth["password"]
    )
    print(json.dumps(site.get(json.loads(sys.argv[1]))))


if __name__ == "__main__":
    main()
