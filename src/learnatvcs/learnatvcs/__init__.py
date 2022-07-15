"""Awesome lesson plan utilities"""
import contextlib
import os
from pathlib import Path
from typing import Optional

import selenium
from data49 import web
from platformdirs import user_data_dir
from selenium import webdriver
from selenium.webdriver.chrome.service import Service as ChromeService
from webdriver_manager.chrome import ChromeDriverManager

import interprog
from learnatvcs import process as process_data

from .highlighter import highlight
from .models import ClassDay, Date

HEADLESS = False  # TODO: get it to work with headless
APP_NAME = "Skootils"
APP_AUTHOR = "Bryan Hu"
DATA_DIR = Path(user_data_dir(APP_NAME, APP_AUTHOR)) / "learnatvcs"


# FIXME: Restructure (again, ugh) to be similar to lesson plan strucure?
# or keep it like this: the space-optimized way, at the cost of information
# (or in this case, the lack of knowledge of information loss)
RawOutput = dict[Optional[str], dict[str, str]]


def _lesson_plan_pipeline(contents: str) -> str:
    # TODO Give highlighting function what date is 'today'
    return str(highlight(process_data.clean_html(process_data.to_soup(contents))))


def _get_browser() -> web.RawWebDriver:

    options = webdriver.ChromeOptions()
    options.add_argument(f"user-data-dir={DATA_DIR/'.webdriver_profile'}")
    if HEADLESS:
        options.add_argument("--headless")

    # webdriver_manager makes stdout dirty
    with open(os.devnull, "w") as devnull:
        with contextlib.redirect_stdout(devnull):
            # TODO: Suppress stdout
            # TODO: pin version
            driver = webdriver.Chrome(
                options=options,
                service=ChromeService(
                    executable_path=ChromeDriverManager(
                        path=str(DATA_DIR / "chromedriver")
                    ).install()
                ),
            )
    return driver


reporter = interprog.TaskManager()


def _scrape(
    browser: web.BrowserContext, for_dates: Optional[list[Date]] = None
) -> RawOutput:
    """Pure logic in scraping learn@vcs"""
    output: RawOutput = {}
    reporter.add_task("Log in")
    reporter.start()
    # Log in
    browser.query_selector(".login-btn").click()
    browser.query_selector("div.potentialidp:nth-child(1) > a:nth-child(1)").click()
    # For every class...
    try:
        browser.query_selector("#side-panel-button").click()
    except web.browser_exceptions.NoSuchElementException:
        # Please sign in
        print("false")
        input()
    else:
        print("true")
    reporter.finish()
    # List of classes
    raw_links = browser.query_selector_all("#inst6206 > div > div > ul > li > div > a")[
        2:-1
    ]
    links = [link["href"] for link in raw_links]
    class_names = [link["title"] for link in raw_links]
    for name in class_names:
        reporter.add_task(
            f"Scrape {name}",
            total=len(for_dates) if for_dates and len(for_dates) > 1 else None,
        )
    for link, class_name in zip(links, class_names):
        reporter.start()
        browser.go(to=link)
        try:
            browser.query_selector('a[title^="Lesson"]').click()
            browser.query_selector_all(".activity.book.modtype_book a")[-1].click()
        except (selenium.common.exceptions.NoSuchElementException, IndexError):
            reporter.error("No lesson plans")
            continue
        # ...go to that day's lesson plans...
        date2link = {
            ClassDay.from_str(date["innerText"]): date.get("href")
            for date in browser.query_selector_all(".book_toc a,.book_toc strong")
        }
        for date in for_dates or [list(date2link)[-1]]:
            day_key = str(date) if for_dates is not None else None
            if day_key not in output:
                output[day_key] = {}  # or should we use a list?
            chosen = [
                date2link[link_date] for link_date in date2link if link_date == date
            ]
            if len(chosen) > 1:
                # Should never happen
                assert False, f"Ambigous lesson plan dates for date {date}"
            if len(chosen) == 0:
                # TODO: Error reporting
                # output[day_key][class_name] = None
                # output[day_key][class_name] = f"No lesson plans for {date}"
                reporter.increment(silent=True)
                continue
            if chosen[0] is not None:
                browser.go(to=chosen[0])
            # ...and scrape it
            content = browser.css("section#region-main > div[role='main']")
            output[day_key][class_name] = _lesson_plan_pipeline(content["innerHTML"])
            reporter.increment(silent=True)
        reporter.finish()
    return output


def scrape(for_dates: Optional[list[Date]] = None) -> RawOutput:
    """Get raw scrape data. Manages browser, etc"""
    # TODO: Choose quarters
    reporter.add_task("Initalizing browser")
    reporter.start()
    _start_browser = web.Browser("https://learn.vcs.net/", driver=_get_browser)
    try:
        browser = _start_browser.open()
    except web.browser_exceptions.WebDriverException:
        reporter.error(
            "Please close the window and install Google Chrome (and try again after)"
        )
        input()
    reporter.finish()

    return _scrape(browser=browser, for_dates=for_dates)


def mock(for_dates: Optional[list] = None) -> str:  # type: ignore
    """how to kill a mockingbird?"""
    import time

    reporter.add_task("Log in")
    reporter.start()
    time.sleep(1)
    reporter.finish()
    class_names = ["English", "Math", "Science", "History"]
    for name in class_names:
        reporter.add_task(
            f"Scrape {name}",
            total=len(for_dates) if for_dates and len(for_dates) > 1 else None,  # type: ignore
        )
    reporter.start()
    time.sleep(1)
    reporter.increment(silent=True)
    time.sleep(2)
    reporter.error("Lmao get good")
    for _ in class_names[1:]:
        reporter.start()
        time.sleep(0.2)
        for _ in for_dates or [None]:  # type: ignore
            time.sleep(1)
            reporter.increment(silent=True)
        reporter.finish()
    return (Path(__file__).parent.parent / "example.json").read_text()


__version__ = "0.1.0"
