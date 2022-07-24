"""Highlight date and assignments"""
# TODO: AI
import re

from bs4 import BeautifulSoup

from learnatvcs.process import to_soup

# TODO: Re-organize
_SHORT_DATE = r"(?:\d+)/(?:\d+)"
_START_KEYWORDS = r"(?:on|(?:start\w+)|(?:begin\w+)|(?:next\w+))"
_DUAL_DATE = rf"(?:{_SHORT_DATE}(?:\s*&\s*{_SHORT_DATE})?)"
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
times = (
    [rf"(?:{month}(?: \d+\w*)?)" for month in MONTHS]
    + MONTHS
    + [
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "today",
        "tomorrow",
        "week",
        "month",
        "year",
        "period",
        "class",
    ]
)
_RELATIVE_KEYWORDS = "|".join(
    f"(?:{pat})"
    for pat in {"next", "the following", "following", "in", "during", "at", "when"}
)
_WORD_DATE = f"(?:{'|'.join(times)})"
DATE = (
    rf"(?:(?:{_START_KEYWORDS}|{_RELATIVE_KEYWORDS})?\s*(?:{_DUAL_DATE}|{_WORD_DATE}))"
)
# TIMES = ""
#
# periods = (
#     [
#         "((next|following|in) )?class",
#         "in-class",
#         "((next|following) )?period",
#         "((next|following) )?week",
#         "((next|following) )?month",
#         "((next|following) )?year",
#     ]
#     + [rf"{month}(?: \d+\w*)?" for month in MONTHS]
#     + [rf"((next|following) )?{time}" for time in times]
# )
# DAYS = f'(?:{"|".join(periods)})'
wordlist = [
    rf"(due|by)(.+{DATE})",
    r"(begin|continue|finish)( \w+ing)?",
    # "(next|following) (class|period|week|month|year)",
    # r"(begin|continue|finish) \w+ing",
    # f"due(.+{DAYS})",
    # rf"by(.+{DAYS})",
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
