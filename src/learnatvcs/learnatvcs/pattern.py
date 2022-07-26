import re


def _group(x) -> str:
    return f"(?:{x})"


def _union(*args) -> str:
    return _group("|".join(map(_group, args)))


def _(*args) -> str:
    return _group(r"\s+".join(map(_group, args)))


# Originally a Lark grammar
RELATIVE_KEYWORD = _union("next", _(r"(the)?", "following"))
LONG_MONTH = r"january|february|march|april|may|june|july|august|september|october|november|december"
SHORT_MONTH = r"jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec"
MONTH = f"({LONG_MONTH})|({SHORT_MONTH})"
ENTITY = rf"class|period|year|month|week|({MONTH})"
relative_date = _union(_(RELATIVE_KEYWORD, ENTITY), "tomorrow", "today")
NUM_DATE = _union(
    _(MONTH, r"(\d+(/\d+)?)|(\d+\w+)"),
    r"\d+/\d+/\d+",
)
date = re.compile(_union(NUM_DATE, relative_date), flags=re.IGNORECASE)
