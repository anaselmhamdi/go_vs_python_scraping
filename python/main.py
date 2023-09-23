import asyncio
import os
import time

import aiohttp
import requests
from bs4 import BeautifulSoup
from dotenv import load_dotenv

load_dotenv(".env")
PROXY_URL = os.environ.get("PROXY_URL")

# def main():
#     start = time.time()
#     url = "https://en.wikipedia.org/wiki/List_of_NBA_All-Stars"
#     all_links = requests.get(url)
#     soup = BeautifulSoup(all_links.content, 'html.parser')
#     table = soup.select_one('table.wikitable.sortable')
#     links = [a['href'] for a in table.select("span.fn > a")]
#     print(links)
#     for link in links:
#         soup = BeautifulSoup(requests.get(f"https://en.wikipedia.org{link}").content, 'html.parser')
#         name = soup.select_one('caption').get_text() if soup.select_one('caption') else ""
#         all_labels = [td.get_text() for td in soup.select('td.infobox-data')]
#         print(name, all_labels[1], all_labels[2])
#     end = time.time()
#     print(f"Time taken to run the scraper: {end - start} s")

# def main():
#     start = time.time()
#     url = "https://en.wikipedia.org/wiki/List_of_NBA_All-Stars"
#     all_links = requests.get(url)
#     soup = BeautifulSoup(all_links.content, 'html.parser')
#     table = soup.select_one('table.wikitable.sortable')
#     links = [a['href'] for a in table.select("span.fn > a")]
#     print(links)

#     def scrape_all_star(link):
#         soup = BeautifulSoup(requests.get(f"https://en.wikipedia.org{link}").content, 'html.parser')
#         name = soup.select_one('caption').get_text() if soup.select_one('caption') else ""
#         all_labels = [td.get_text() for td in soup.select('td.infobox-data')]
#         print(name, all_labels[1], all_labels[2])

#     threads = [threading.Thread(target=scrape_all_star, args=(link,)) for link in links]

#     for thread in threads:
#         thread.start()

#     for thread in threads:
#         thread.join()


async def main():
    start = time.time()
    url = "https://en.wikipedia.org/wiki/List_of_NBA_All-Stars"
    all_links = requests.get(url)
    soup = BeautifulSoup(all_links.content, 'html.parser')
    table = soup.select_one('table.wikitable.sortable')
    links = [a['href'] for a in table.select("span.fn > a")]
    print(links)

    async def scrape_all_star(link):
        async with aiohttp.ClientSession() as session:
            async with session.get(link, proxy=PROXY_URL) as response:
                html_text = await response.text()
                soup = BeautifulSoup(html_text, 'html.parser')
                name = soup.select_one('caption').get_text() if soup.select_one('caption') else ""
                all_labels = [td.get_text() for td in soup.select('td.infobox-data')]
                print(name, all_labels[1], all_labels[2])
    tasks = [scrape_all_star(f"https://en.wikipedia.org{link}") for link in links]
    await asyncio.gather(*tasks)
    end = time.time()
    print(f"Time taken to run the scraper: {end - start} s")


if __name__ == '__main__':
    # main()
    asyncio.run(main())
