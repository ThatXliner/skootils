from powerschool import PowerSchool
import json


def main():
    HAS_CREDENTIALS = False
    print(json.dumps(HAS_CREDENTIALS))
    auth = json.loads(input())

    site = PowerSchool(
        "https://powerschool.vcs.net/public", auth["username"], auth["password"]
    )
    print(json.dumps(site.full_info_scrape()))


if __name__ == "__main__":
    main()
