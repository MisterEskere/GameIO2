import requests
import sys

try:
    url = sys.argv[1]
    domain = sys.argv[2]
    headers = {'Host': domain}
except:
    url = f"https://190.115.31.179/?s=gta"
    domain = "fitgirl-repacks.site"
    headers = {'Host': domain}

print("URL:", url)
print("Headers:", headers)

# Make the request to get the page
try:
    response = requests.get(url, headers=headers, verify=True)
except:
    response = requests.get(url, headers=headers, verify=False)


# save the page to a file
with open(f"tmp.html", "w") as file:
    file.write(response.text)
