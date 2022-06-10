"""The main CLI entry point"""
import json
import sys

from learnatvcs import Bot, Date


def main():
    dates = [Date.from_str(x) for x in json.loads(sys.argv[1]) or []]
    print(Bot().get_json(for_dates=dates or None))


if __name__ == "__main__":
    main()
