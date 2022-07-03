"""Summarize text, AI or algorithmic"""
import re

from bs4 import BeautifulSoup
from sumy.nlp.stemmers import Stemmer
from sumy.nlp.tokenizers import Tokenizer
from sumy.parsers.html import HtmlParser
from sumy.summarizers.lsa import LsaSummarizer as Summarizer
from sumy.utils import get_stop_words
from transformers import pipeline


def extract_useful(
    soup: BeautifulSoup,
    headers: Tuple[str, ...] = ("classroom activities", "turned in", "deliverables"),
) -> str:
    """Extracts 'Classroom Activities', 'Deliverables', etc"""
    # TODO: Deprecate and use JS parser
    name_re = re.compile("|".join(f"({x})" for x in headers))

    def query(tag) -> bool:
        return (
            tag.name == "h4"
            and tag.strong
            and name_re.search(tag.strong.get_text().lower()) is not None
        )

    return "".join(map(lambda x: str(x) + str(x.next_sibling or ""), soup(query)))


def algorithm(html, language: str = "english", sentence_count: int = 10):
    parser = HtmlParser(html, Tokenizer(language))
    summarizer = Summarizer(Stemmer(language))
    summarizer.stop_words = get_stop_words(language)

    return "".join(summarizer(parser.document, sentence_count))


__ai = None
AI_MODEL = "facebook/bart-large-cnn"


def ai(text: str, max_length: int = 300, min_length: int = 30) -> str:
    global __ai
    if __ai is None:
        __ai = pipeline("summarization", model=AI_MODEL)
    return __ai(text, max_length=max_length, min_length=min_length, do_sample=False)[0][
        "summary_text"
    ]


__version__ = "0.1.0"
