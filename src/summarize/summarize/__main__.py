import json
import re

from bs4 import BeautifulSoup

from summarize import ai

WORKS = ("classroom activities", "turned in", "deliverables")
NAME_RE = re.compile("|".join(f"({x})" for x in WORKS))


def extract_useful(soup: BeautifulSoup) -> str:
    """Extracts 'Classroom Activities', 'Deliverables', etc"""

    def query(tag) -> bool:
        return (
            tag.name == "h4"
            and tag.strong
            and NAME_RE.search(tag.strong.get_text().lower()) is not None
        )

    return "".join(map(lambda x: str(x) + str(x.next_sibling or ""), soup(query)))


def main():
    # TODO: Choose between ai and algorithm
    while True:
        inp = extract_useful(BeautifulSoup(json.loads(input()), features="html.parser"))
        print(json.dumps(ai(inp)))


if __name__ == "__main__":
    main()
