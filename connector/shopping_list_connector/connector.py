import requests  # type: ignore
from pydantic import BaseModel
from typing import Any

URL = "http://localhost:8080/all_products"


class Product(BaseModel):
    name: str
    price: float
    

def main() -> list[Product]:
    response = requests.get(URL)
    products: list[dict[str, Any]] = response.json()
    return [Product(**data) for data in products]


if __name__ == "__main__":
    print(main())
