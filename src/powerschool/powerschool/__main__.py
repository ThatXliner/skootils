import json

from powerschool import PowerSchool, storage


def main() -> None:
    cred = storage.get_auth()
    HAS_CREDENTIALS = cred is not None
    print(json.dumps(HAS_CREDENTIALS))

    auth = cred or json.loads(input())  # TODO: Remember?
    if cred is None:
        storage.set_auth(auth)
    # input('["haha","L"]\n')
    # print(
    #     json.dumps(
    #         json.loads(
    #             (Path(__file__).parent.parent / "example-large.json").read_text()
    #         )
    #     )
    # )

    with PowerSchool(
        "https://powerschool.vcs.net/public", auth["username"], auth["password"]
    ) as bot:
        quarters = bot.get_quarters()
        print(json.dumps(quarters))
        output = bot.get(json.loads(input()))
        print(json.dumps(output))
        storage.save(output, quarters)


if __name__ == "__main__":
    main()
