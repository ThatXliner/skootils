"""Highlight date and assignments"""
# TODO: AI


from bs4 import BeautifulSoup

from learnatvcs.process import to_soup
from learnatvcs.pattern import date as DATE_RE, assignment as ASSIGNMENT_RE

HIGHLIGHT_ME = lambda x: DATE_RE.search(x) or ASSIGNMENT_RE.search(
    x
)  # type: (str) -> bool


def create_highlight(x: str) -> str:
    return f"""<span data-highlight="true">{x}</span>"""


def highlight(soup: BeautifulSoup) -> BeautifulSoup:
    for body in soup(string=HIGHLIGHT_ME):
        search_space = body.get_text()
        match = DATE_RE.search(search_space)
        span = match.span()
        left, right = search_space[: span[0]], search_space[span[1] :]
        body.replace_with(
            left, to_soup(create_highlight(match[0])), highlight(to_soup(right))
        )
    return soup


if __name__ == "__main__":
    import sys

    print(highlight(BeautifulSoup(sys.stdin.read(), features="html.parser")))
