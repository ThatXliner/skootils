"""A browser-automation-based API for PowerSchool"""
import asyncio
import re
from dataclasses import dataclass, field
from functools import reduce
from typing import Dict, List, Optional

import aiohttp
from bs4 import BeautifulSoup, SoupStrainer

import interprog

__version__ = "0.1.0"


NAME_RE = re.compile(r"Email (\w+),\s*(\w+\.)\s*(\w+)")
NOT_AVAILABLE = "N/A"

OutputType = Dict[str, Dict[str, str]]


@dataclass
class PowerSchool:
    url: str
    username: str
    password: str
    session: aiohttp.ClientSession = field(init=False)
    home_table: BeautifulSoup = field(init=False)
    student_id: Optional[int] = None
    _reporter: interprog.TaskManager = field(default_factory=interprog.TaskManager)
    _class_to_sectionid: Dict[str, int] = field(default_factory=dict)

    async def __aenter__(self) -> "PowerSchool":
        self.session = aiohttp.ClientSession()
        async with self.session.post(
            self.url + "/guardian/home.html",
            data={"account": self.username, "pw": self.password},
        ) as resp:
            resp.raise_for_status()
        async with self.session.get(self.url + "/guardian/home.html") as resp:
            self.home_table = BeautifulSoup(
                await resp.text(), features="html.parser"
            ).find(id="quickLookup")
        # print(self.home_table)
        return self

    async def __aexit__(self, exc_type, exc_value, traceback) -> None:
        await self.session.close()

    @staticmethod
    def _get_flags(score) -> int:
        # not including "is collected"
        # because that one can be false even
        # when we have full score.
        # it's weird :/
        FLAGS = {
            "late": 1 << 0,
            "exempt": 1 << 1,
            "missing": 1 << 2,
            "incomplete": 1 << 3,
            "absent": 1 << 4,
        }
        return reduce(
            lambda a, b: a | b,
            [value if score[f"is{key}"] else 0 for key, value in FLAGS.items()],
            0,
        )

    def _make_assignment(self, assignment):
        #   info = {
        #   "_name": "assignmentsection",
        #   "isscorespublish": true,
        #   "scoreentrypoints": 5.0,
        #   "weight": 1.0,
        #   "isscoringneeded": true,
        #   "sectionsdcid": 40915,
        #   "totalpointvalue": 5.0,
        #   "assignmentsectionid": 467070,
        #   "iscountedinfinalgrade": true,
        #   "duedate": "2022-02-02",
        #   "_assignmentscores": [
        #     {
        #       "scorepoints": 5.0,
        #       "islate": false,
        #       "_name": "assignmentscore",
        #       "isexempt": false,
        #       "studentsdcid": 18500,
        #       "scorelettergrade": "A+",
        #       "isabsent": false,
        #       "ismissing": false,
        #       "isincomplete": false,
        #       "whenmodified": "2022-02-08",
        #       "iscollected": false,
        #       "actualscorekind": "REAL_SCORE",
        #       "authoredbyuc": false,
        #       "scoreentrydate": "2022-02-08 18:40:46",
        #       "actualscoreentered": "5",
        #       "scorepercent": 100.0
        #     }
        #   ],
        #   "name": "Book Report Tone Square",
        #   "scoretype": "POINTS",
        #   "_id": 467070,
        #   "_assignmentcategoryassociations": [
        #     {
        #       "assignmentcategoryassocid": 323300,
        #       "_name": "assignmentcategoryassoc",
        #       "_teachercategory": {
        #         "_name": "teachercategory",
        #         "color": "2",
        #         "name": "Homework"
        #       },
        #       "_id": 323300
        #     }
        #   ]
        # }
        sections = assignment["_assignmentsections"]
        assert len(sections) == 1
        info = sections[0]
        scores = info["_assignmentscores"]
        assert len(scores) == 1
        score = scores[0]
        assignment_types = info["_assignmentcategoryassociations"]
        assert len(assignment_types) == 1
        assignment_type = assignment_types[0]
        assert info["scoreentrypoints"] == info["totalpointvalue"]
        return {
            "due_date": info["duedate"],
            "description": info.get("description"),
            "comment": info["_assignmentscorecomment"]["commentvalue"]
            if "_assignmentscorecomment" in info
            else None,
            "type": assignment_type["_teachercategory"]["name"],
            # types:
            # - Classwork
            # - Homework
            # - Participation
            # - Lab
            # - Project
            # - Quiz
            # - Test
            # - Progress Assessments
            # - Mastery Assessments
            "name": info["name"],
            # TODO: Flags (missing, late, etc)
            "flag": self._get_flags(score),
            "_raw": info,
            "score": {  # Can also be --/20 or smth like that
                "total": info["totalpointvalue"],
                "recieved": score.get("scorepoints"),
            },
            # these two does not exist when it's extra credit or extempted
            "percent": score.get(
                "scorepercent"  # , int(score["actualscoreentered"]) * 100
            ),
            "grade": score.get("scorelettergrade"),  # A+, B-, etc.
        }

    def get_quarters(self) -> List[str]:
        return [
            el.string
            for el in self.home_table.table.tr.select("th")
            if el.string[1].isdigit()
        ]

    # DEPRECATE: Don't allow None as an option
    async def get(self, quarters: Optional[List[str]] = None) -> OutputType:
        soup = self.home_table
        output: OutputType = {}

        self._reporter.add_task("Scraping")
        tasks = []
        self._reporter.start()
        for quarter in quarters or [None]:
            tasks.append(
                asyncio.create_task(self._scrape_quarter(soup, output, quarter))
            )
        await asyncio.gather(*tasks)
        self._reporter.finish()
        return output

    async def _scrape_quarter(self, soup: BeautifulSoup, output, quarter: str):
        offset = -3  # Because the rightmost contains "Absences" and "Tardies"
        heads = soup.table.tr.select("th")
        if quarter is not None:
            offset = -(len(heads) - [el.string for el in heads].index(quarter))

        current_quarter_name = heads[offset].string

        quarter_key = quarter or f"Latest ({current_quarter_name})"
        output[quarter_key] = {}
        _output = output[quarter_key]  # Don't use references if we're using a lock

        # Row of class info
        classes = soup.table.select("[id^='ccid_']")

        for row in classes:
            await self._scrape_class(offset, current_quarter_name, _output, row)

    async def _scrape_class(
        self, offset: int, quarter: str, _output, row: BeautifulSoup
    ):
        period = row.td.string
        misc = row.find("td", align="left")
        class_name = misc.contents[0].strip()
        # Email and teacher name
        email_info = misc("a")[-1]
        email = email_info["href"][len("mailto:") :]
        name_info = NAME_RE.search(email_info.get_text())
        last_name = name_info.group(1)
        title = name_info.group(2)
        # first_name = name_info.group(3)
        name = title + " " + last_name

        current_quarter = row.select("td")[offset]
        current_grade = current_quarter.a
        current_grade_name = current_grade_percent = NOT_AVAILABLE
        assignments = []
        # TODO: Only add classes that have grades for this quarter
        # (in other words, don't add to the output)
        if current_grade:
            try:
                current_grade_name = current_grade.contents[0]
                current_grade_percent = current_grade.contents[-1]
            except AttributeError:
                current_grade_name = current_grade_percent = NOT_AVAILABLE
            await self._setup_cachables(class_name, current_grade)

            assignments.extend(await self._scrape_assignments(quarter, class_name))
        _output[period.upper()] = {  # TODO: Change format
            "name": name,
            "class_name": class_name,
            "email": email,
            "quarter_info": {
                "name": quarter,
                "overall_grade": {
                    "name": current_grade_name,
                    "percent": current_grade_percent,
                },
                "scores": assignments,
            },
        }

    async def _setup_cachables(self, class_name: str, current_grade: Dict[str, str]):
        """Initializes self.student_id and self._class_to_sectionid"""
        _page = None
        if self.student_id is None:
            if _page is None:
                async with self.session.get(
                    "https://powerschool.vcs.net/guardian/" + current_grade["href"]
                ) as resp:
                    _page = await resp.text()
            self.student_id = int(re.search(r"studentFRN.+?(\d+)", _page).group(1)[3:])
        if class_name not in self._class_to_sectionid:
            if _page is None:
                async with self.session.get(
                    "https://powerschool.vcs.net/guardian/" + current_grade["href"]
                ) as resp:
                    _page = await resp.text()
            self._class_to_sectionid[class_name] = int(
                BeautifulSoup(
                    _page,
                    features="html.parser",
                    parse_only=SoupStrainer(attrs={"data-sectionid": True}),
                ).div["data-sectionid"]
            )

    async def _scrape_assignments(self, quarter: str, class_name: str):
        async with self.session.post(
            "https://powerschool.vcs.net/ws/xte/assignment/lookup",
            headers={"Referer": "https://powerschool.vcs.net/public"},
            json={
                # dunno why it's a list
                "section_ids": [self._class_to_sectionid[class_name]],
                "student_ids": [self.student_id],
                # 'Specify store_codes, or start_date and end_date'
                "store_codes": [quarter],
            },
        ) as resp:
            return [
                self._make_assignment(assignment) for assignment in await resp.json()
            ]
