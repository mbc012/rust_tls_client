"""
Simple script for downloading the latest release of the tls-client binary distribution releases.
"""


import requests

# Obtain latest release, extract release assets
assets = r = requests.get("https://api.github.com/repos/bogdanfinn/tls-client/releases/latest").json()["assets"]

# Download each asset
for asset in assets:
    file_name = asset["name"]
    dl_url = asset["browser_download_url"]
    print(f"Downloading {file_name}...")
    with open(file_name, "wb") as file:
        content = requests.get(dl_url).content
        file.write(content)
