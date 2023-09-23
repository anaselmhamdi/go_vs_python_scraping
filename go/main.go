package main

import (
	"fmt"
	"log"
	"os"
	"sync"
	"time"

	"github.com/gocolly/colly"
	"github.com/joho/godotenv"
)

// func main() {
// 	start := time.Now()
// 	linkCollector := colly.NewCollector(
// 		colly.AllowedDomains("en.wikipedia.org"),
// 	)

// 	allStarsCollector := linkCollector.Clone()

// 	linkCollector.OnHTML(".wikitable.sortable", func(e *colly.HTMLElement) {
// 		links := e.ChildAttrs("span.fn > a", "href")

// 		for _, link := range links {
// 			allStarsCollector.Visit(fmt.Sprintf("https://en.wikipedia.org%s", link))
// 		}
// 	})

// 	allStarsCollector.OnHTML(".infobox.vcard", func(e *colly.HTMLElement) {
// 		name := e.ChildText("caption")
// 		allLabels := make([]string, 0, 5)
// 		e.ForEach("td.infobox-data", func(_ int, el *colly.HTMLElement) {
// 			allLabels = append(allLabels, el.Text)
// 		})
// 		log.Println(name, allLabels[1], allLabels[2])
// 	})

// 	linkCollector.Visit("https://en.wikipedia.org/wiki/List_of_NBA_All-Stars")
// 	elapsed := time.Since(start)
// 	log.Printf("Scraping all the NBA all stars took %s", elapsed)
// }

func main() {
	start := time.Now()
	godotenv.Load()
	var proxyURL string = os.Getenv("PROXY_URL")
	var wg sync.WaitGroup

	linkCollector := colly.NewCollector(
		colly.AllowedDomains("en.wikipedia.org"),
	)

	linkCollector.SetProxy(proxyURL)

	allStarsCollector := linkCollector.Clone()

	linkCollector.OnHTML(".wikitable.sortable", func(e *colly.HTMLElement) {
		links := e.ChildAttrs("span.fn > a", "href")

		for _, link := range links {
			wg.Add(1)
			go func(link string) {
				defer wg.Done()
				allStarsCollector.Visit(fmt.Sprintf("https://en.wikipedia.org%s", link))
			}(link) // parallel call now
		}
	})

	allStarsCollector.OnHTML(".infobox.vcard", func(e *colly.HTMLElement) {
		name := e.ChildText("caption")
		allLabels := make([]string, 0, 5)
		e.ForEach("td.infobox-data", func(_ int, el *colly.HTMLElement) {
			allLabels = append(allLabels, el.Text)
		})
		log.Println(name, allLabels[1], allLabels[2])
	})

	linkCollector.Visit("https://en.wikipedia.org/wiki/List_of_NBA_All-Stars")
	wg.Wait()
	elapsed := time.Since(start)
	log.Printf("Scraping all the NBA all stars took %s", elapsed)
}
