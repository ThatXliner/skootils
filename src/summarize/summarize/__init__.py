"""Summarize text, AI or algorithmic"""

from sumy.nlp.stemmers import Stemmer
from sumy.nlp.tokenizers import Tokenizer
from sumy.parsers.html import HtmlParser
from sumy.summarizers.lsa import LsaSummarizer as Summarizer
from sumy.utils import get_stop_words
from transformers import pipeline


def algorithm(html, language: str = "english", sentence_count: int = 10):
    parser = HtmlParser(html, Tokenizer(language))
    summarizer = Summarizer(Stemmer(language))
    summarizer.stop_words = get_stop_words(language)

    return "".join(summarizer(parser.document, sentence_count))


__ai = None
AI_MODEL = "facebook/bart-large-cnn"


def ai(text: str, max_length: int = 300, min_length: int = 30) -> str:
    global __ai  # skipcq
    if __ai is None:
        __ai = pipeline("summarization", model=AI_MODEL)
    return __ai(text, max_length=max_length, min_length=min_length, do_sample=False)[0][
        "summary_text"
    ]


__version__ = "0.1.0"
