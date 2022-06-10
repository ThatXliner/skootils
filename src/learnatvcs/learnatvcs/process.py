import re

from bs4 import BeautifulSoup


def to_soup(x: str) -> BeautifulSoup:
    return BeautifulSoup(x, features="html.parser")


def clean_html(soup: BeautifulSoup, allow_embeds: bool = True) -> BeautifulSoup:
    for element in soup():
        if element.name == "h5":
            element.name = "h4"  # TailwindCSS prose doesn't work on h5

        # remove empty elements
        if (element.name not in {"span", "iframe"} if allow_embeds else True) and len(
            element.get_text(strip=True)
        ) == 0:
            element.decompose()
        # Unwrap nested divs
        while element.name == "div" and len(element.contents) == 1:
            element.unwrap()
    # Change attrs
    for element in soup():
        del element["class"]
        if element.name == "a":
            element["target"] = "_blank"

    return soup


def extract_useful(soup: BeautifulSoup) -> str:
    """Extracts 'Classroom Activities', 'Deliverables', etc"""
    works = ("classroom activities", "turned in", "deliverables")
    name_re = re.compile("|".join(f"({x})" for x in works))

    def query(tag) -> bool:
        return (
            tag.name == "h4"
            and tag.strong
            and name_re.search(tag.strong.get_text().lower()) is not None
        )

    return "".join(map(lambda x: str(x) + str(x.next_sibling or ""), soup(query)))
