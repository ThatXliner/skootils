"""Highlight date and assignments"""
# TODO: AI
import re

from bs4 import BeautifulSoup

from .process import to_soup

# TODO: Re-organize
MONTHS = [
    "january",
    "february",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december",
]
times = [
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "today",
    "tomorrow",
]
periods = (
    [
        "((next|following|in) )?class",
        "in-class",
        "((next|following) )?period",
        "((next|following) )?week",
        "((next|following) )?month",
        "((next|following) )?year",
    ]
    + [rf"{month}(?: \d+\w*)?" for month in MONTHS]
    + [rf"((next|following) )?{time}" for time in times]
)
DAYS = f'(?:{"|".join(periods)})'
wordlist = [
    "(next|following) (class|period|week|month|year)",
    r"(begin|continue|finish) \w+ing",
    f"due(.+{DAYS})",
    rf"by(.+{DAYS})",
]

HIGHLIGHT_ME = re.compile(
    "|".join(f"(?:{pat})" for pat in wordlist), flags=re.IGNORECASE
)


def create_highlight(x: str) -> str:
    return f"""<span data-highlight="true">{x}</span>"""


def highlight(soup: BeautifulSoup) -> BeautifulSoup:
    for body in soup(string=HIGHLIGHT_ME):
        match = HIGHLIGHT_ME.search(str(body.string))
        span = match.span()
        left, right = body.string[: span[0]], body.string[span[1] :]
        body.replace_with(
            left, to_soup(create_highlight(match[0])), highlight(to_soup(right))
        )
    return soup
