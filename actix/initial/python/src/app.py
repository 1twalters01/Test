import requests


def start():
    url = "http://localhost:8080"

    url1 = url + "/json/"
    print(url1)
    response = requests.get(url1)
    print(response)
    print(response.text)

    url2 = url + "/json/get-single/test"
    print(url2)
    response = requests.get(url2)
    print(response.text)
    print(response.json())
    print("name: " + response.json()["name"])

    url3 = url + "/json/get-double/test/45.62"
    print(url3)
    response = requests.get(url3)
    print(response.text)
    print(response.json())
    print("name: " + response.json()["name"])
    print("age: " + str(response.json()["age"]))

    url4 = url + "/json/echo"
    print(url4)
    payload1 = {"field_1": "value", "amount": 5}
    response = requests.post(url4, json=payload1)
    print(response.text)
    print(response.json())
    print("field 1: " + response.json()["field_1"])
    print("amount: " + str(response.json()["amount"]))

    url5 = url + "/json/post-single/test"
    print(url5)
    payload = {"field_1": "value", "amount": 25}
    response = requests.post(url5, json=payload)
    print(response.text)
    print(response.json())
    print("field 1: " + response.json()["field_1"])
    print("amount: " + str(response.json()["amount"]))
    print("param: " + response.json()["param"])

    url6 = url + "/json/post-double/test/73.99"
    print(url6)
    payload = {"field_1": "value", "amount": 25}
    response = requests.post(url6, json=payload)
    print(response.text)
    print(response.json())
    print("field 1: " + response.json()["field_1"])
    print("amount: " + str(response.json()["amount"]))
    print("param 1: " + response.json()["param_1"])
    print("param 2: " + str(response.json()["param_2"]))

