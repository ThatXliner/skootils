import asyncio
import json
import sys

from powerschool import PowerSchool, storage


async def main() -> None:
    if len(sys.argv) == 2 and sys.argv[1] == "--test":
        print("Program is valid")
        return
    cred = storage.get_auth()
    HAS_CREDENTIALS = cred is not None
    print(json.dumps(HAS_CREDENTIALS))

    auth = cred or json.loads(input())  # TODO: Remember?
    if cred is None:
        storage.set_auth(auth)
    # if MOCK:
    #     input('["haha","L"]\n')
    #     print(
    #         json.dumps(
    #             json.loads(
    #                 (Path(__file__).parent.parent / "example-large.json").read_text()
    #             )
    #         )
    #     )
    #     return

    async with PowerSchool(
        "https://powerschool.vcs.net", auth["username"], auth["password"]
    ) as bot:
        quarters = bot.get_quarters()
        print(json.dumps(quarters))
        output = await bot.get(json.loads(input()))
        print(json.dumps(output))
        storage.save(output, quarters)
        storage.save_teachers(output)


if __name__ == "__main__":
    asyncio.run(main())
