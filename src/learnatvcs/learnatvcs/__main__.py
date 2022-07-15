"""The main CLI entry point"""
import json
import sys

from learnatvcs import scrape
from learnatvcs.models import Date

# from pathlib import Path
# from platformdirs import user_cache_dir
# APP_NAME = "Skootils"
# APP_AUTHOR = "Bryan Hu"
# CACHE_DIR = Path(user_cache_dir(APP_NAME,APP_AUTHOR))
# So how the cache works is that we cache every day seperately
# e.g.
# cache/
#  2022-01-01.json

# To retrieve cache, we can concatenate as well


def main():
    dates = [Date.from_str(x) for x in json.loads(sys.argv[1]) or []]
    print(json.dumps(scrape(for_dates=dates or None)))


if __name__ == "__main__":
    main()
