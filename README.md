# Weather-cli 

## How to install weather-cli

```bash
git clone https://github.com/10Keshav/weather-cli
cd weather-cli
cargo install --path
```

## API Key
Get your api key from from [OpenWeather](https://home.openweathermap.org/api_keys) \
Copy the long string, that would be the api key (Henceforth this string will be referred to as API_KEY)

## How to use this

1. API_KEY is a CLI argument in itself
```bash
weather-cli -c CITY -a API_KEY 

weather-cli --city CITY --api-key API_KEY
```

2. (Recommended) API_KEY is stored in a txt file in your system
Linux/macOS
```bash
mkdir ~/.config/weather-cli
cd weather-cli
echo API_KEY > api_key.txt
```
Windows
```
Create a directory in %APPDATA% called 'weather-cli'
Create a file 'api_key.txt'
Paste the API_KEY in 'api_key.txt'
```

3. Similar to Alternative 2, You can just make a file called 'api_key.txt' in your current working directory
