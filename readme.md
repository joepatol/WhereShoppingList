# Disclaimer

This repo is for me to learn & experiment with new languages & technologies, it is *not* intended for any serious scenario.

# Run the application

Install docker desktop and run `docker-compose up` from the project root

# Goal

Create a website that can be used to create a shopping list and find the cheapest supermarket to buy it from

# What is this?

This repo contains:

- A webscraper that scrapes Dutch supermarket websites for product data
- An API to use the scraped data to search for products, create shopping lists etc.

# Extensions & todo's

Just so I don't forget :)

- More supermarkets, nice for usability but technically not super interesting
- TESTS!!
- Don't 'clear' the products table when scraping again, but instead update products where necessary
    - The API needs the id's for a product to stay sound
    - Maybe load all the products in the scraper do some magic to get the new 'updated & correct' table and write to the db
    - Should probably be a table 'scrapes' that contains 'raw' scraped data and a table 'products' used by the API
    - Should the matching be done be a different service? (or should it go through the API, feels like creating a hell...)
- API:
    - Security so you can't get a shoppinglist from another user for example
    - Patch/delete shopping lists
    - Matching products on a search query could be improved: should analyse some ideas
        - How to deal with similar or even identical products? (e.g. jumbo cola, ah cola, coca cola)
- Implement front-end app
    - Look for products, interactively create the cheapest shopping list at a single store
    - It's ok to have 'scrape' button, the scraper doesn't re-start if it's already running
    - Find the closest & cheapest supermarket. Something like a map where locations with total price is shown