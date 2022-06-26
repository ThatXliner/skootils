import requests

USERNAME = ""
PASSWORD = ""
s = requests.Session()
login = s.post(
    "https://powerschool.vcs.net/guardian/home.html",
    data={"account": USERNAME, "pw": PASSWORD},
)
s.cookies.update(login.cookies)
s.post(
    "https://powerschool.vcs.net/ws/xte/assignment/lookup",
    headers={
        "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Safari/605.1.15"
    },
    json={
        "section_ids": [47018],
        "student_ids": [18500],
        "store_codes": ["S2"],
    },
    cookies=s.cookies,
)
