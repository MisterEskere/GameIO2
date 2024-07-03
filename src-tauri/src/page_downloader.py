import requests
import sys

# Check if a game name is provided as a command-line argument
if len(sys.argv) < 2:
    print("Usage: python page_downloader.py <url>")
    sys.exit(1)

url = sys.argv[1]

# Make the request to get the page
try:
    response = requests.get(url, verify=True)
except:
    response = requests.get(url, verify=False)


# save the page to a file
with open(f"tmp.html", "w") as file:
    file.write(response.text)
